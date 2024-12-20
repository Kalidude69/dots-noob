use colored::*;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
struct PackageResult {
    name: String,
    status: InstallStatus,
}

#[derive(Debug)]
enum InstallStatus {
    AlreadyInstalled,
    Installed,
    Failed(String),
}

// Using a single sudo authentication for the entire session
fn ensure_sudo_session() -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["-v"]) // Validate and update timestamp
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Failed to validate sudo privileges".to_string());
    }
    Ok(())
}

fn is_package_installed(package: &str) -> bool {
    Command::new("pacman")
        .args(["-Q", package])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_package(package: &str) -> Result<(), String> {
    // Using sudo directly since we validated the session earlier
    let output = Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", package])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Check for common pacman errors
        if stderr.contains("could not be found") {
            Err(format!("Package '{}' not found in repositories", package))
        } else if stderr.contains("could not satisfy dependencies") {
            Err("Dependency resolution failed".to_string())
        } else {
            Err(stderr.to_string())
        }
    }
}

fn process_package(package: &str) -> PackageResult {
    let name = package.to_string();

    // Print start of processing with lock
    {
        println!("Processing package: {}", name);
    }

    if is_package_installed(package) {
        return PackageResult {
            name,
            status: InstallStatus::AlreadyInstalled,
        };
    }

    // Keep sudo session alive
    let _ = Command::new("sudo").args(["-v"]).output();

    let status = match install_package(package) {
        Ok(_) => InstallStatus::Installed,
        Err(e) => InstallStatus::Failed(e),
    };

    PackageResult { name, status }
}

fn read_packages<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let packages: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();
    Ok(packages)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if running as root
    if !nix::unistd::Uid::effective().is_root() {
        eprintln!("{}", "This script must be run with sudo privileges.".red());
        std::process::exit(1);
    }

    // Ensure sudo session is active
    if let Err(e) = ensure_sudo_session() {
        eprintln!("{}: {}", "Failed to establish sudo session".red(), e);
        std::process::exit(1);
    }

    // Read package list
    let packages = match read_packages("packages.txt") {
        Ok(pkgs) => pkgs,
        Err(e) => {
            eprintln!("{}: {}", "Error reading packages.txt".red(), e);
            std::process::exit(1);
        }
    };

    println!("Found {} packages to process", packages.len());

    // Track results
    let results = Arc::new(Mutex::new(Vec::new()));
    let results_clone = Arc::clone(&results);

    // Process packages in parallel with a controlled thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(4) // Limit concurrent installations
        .build_global()
        .unwrap();

    packages.par_iter().for_each(|package| {
        let result = process_package(package);

        // Print progress with lock to avoid garbled output
        {
            let status_str = match &result.status {
                InstallStatus::AlreadyInstalled => {
                    format!("{} {} is already installed", "✓".green(), package).green()
                }
                InstallStatus::Installed => {
                    format!("{} Successfully installed {}", "✓".green(), package).green()
                }
                InstallStatus::Failed(err) => {
                    format!("{} Failed to install {}: {}", "✗".red(), package, err).red()
                }
            };
            println!("{}", status_str);
        }

        results_clone.lock().unwrap().push(result);
    });

    // Print summary
    let results = results.lock().unwrap();
    let total = results.len();
    let (success, already, failed): (Vec<_>, Vec<_>, Vec<_>) = results
        .iter()
        .partition3(|r| matches!(r.status, InstallStatus::Installed));

    println!("\n{}", "=".repeat(50));
    println!("Installation Summary:");
    println!("Total packages processed: {}", total);
    println!("Already installed: {}", already.len());
    println!("Successfully installed: {}", success.len());
    println!("Failed installations: {}", failed.len());

    if !failed.is_empty() {
        println!("\nFailed packages:");
        for result in failed {
            if let InstallStatus::Failed(err) = &result.status {
                println!("- {}: {}", result.name.red(), err);
            }
        }
    }

    Ok(())
}
