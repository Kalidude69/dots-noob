use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // INFO
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let host = fs::read_to_string("/etc/hostname").unwrap_or_else(|_| "unknown".to_string()).trim().to_string();
    let os = "Cachy OS";
    let kernel = Command::new("uname").arg("-sr").output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let uptime = Command::new("uptime").arg("-p").output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().replace("up ", ""))
        .unwrap_or_else(|_| "unknown".to_string());
    let packages = Command::new("sh").arg("-c").arg("pacman -Q | wc -l").output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());
    let shell = env::var("SHELL").map(|s| s.split('/').last().unwrap_or("unknown").to_string()).unwrap_or_else(|_| "unknown".to_string());

    // UI DETECTION
    fn parse_rcs(paths: &[&str]) -> Option<String> {
        for path in paths {
            if let Ok(content) = fs::read_to_string(path) {
                if let Some(wm) = content.lines().last().and_then(|line| line.split_whitespace().nth(1)) {
                    return Some(wm.to_string());
                }
            }
        }
        None
    }

    let rcwm = parse_rcs(&["~/.xinitrc", "~/.xsession"]);
    let mut ui = "unknown".to_string();
    let mut uitype = "UI".to_string();

    if let Ok(de) = env::var("DE") {
        ui = de;
        uitype = "DE".to_string();
    } else if let Ok(wm) = env::var("WM") {
        ui = wm;
        uitype = "WM".to_string();
    } else if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        ui = desktop;
        uitype = "DE".to_string();
    } else if let Ok(session) = env::var("DESKTOP_SESSION") {
        ui = session;
        uitype = "DE".to_string();
    } else if let Some(rc) = rcwm {
        ui = rc;
        uitype = "WM".to_string();
    } else if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        ui = session_type;
    }

    ui = ui.split('/').last().unwrap_or("unknown").to_string();

    // OUTPUT
    println!(r"
        /\                  {}@{}
       /  \                 OS:        {}
      /\  /\                KERNEL:    {}
     /  __  \               UPTIME:    {}
    /  (  )  \              PACKAGES:  {}
   / __|  |__ \             SHELL:     {}
  /.`        `.\            {}:        {}
",
        user, host, os, kernel, uptime, packages, shell, uitype, ui);
}












































































