local wezterm = require("wezterm")

local config = {}

-- Wayland display server configuration
config.enable_wayland = true

-- Shell
config.default_prog = { "/bin/zsh" }

-- Font
config.font = wezterm.font("JetBrainsMono Nerd Font", {weight="Regular", italic=false})
config.font_size = 14.0
config.bold_brightens_ansi_colors = "BrightAndBold"
config.font = wezterm.font_with_fallback({
  "JetBrainsMono Nerd Font",
  "Noto Sans",
  "DejaVu Sans"
})

-- Background opacity
config.window_background_opacity = 0.6

-- Disable window close confirmation
config.exit_behavior = "Close"
config.color_scheme = 'Catppuccin Mocha'
-- Colors
config.colors = {
  foreground = "#cdd6f4",
  background = "#191724",
  selection_fg = "#1e1e2e",
  selection_bg = "#f5e0dc",
  cursor_fg = "#1e1e2e",
  cursor_bg = "#f5e0dc",
  cursor_border = "#f5e0dc",
  ansi = {
    "#45475a", "#f38ba8", "#a6e3a1", "#f9e2af", 
    "#89b4fa", "#f5c2e7", "#94e2d5", "#bac2de"
  },
  brights = {
    "#585b70", "#f38ba8", "#a6e3a1", "#f9e2af", 
    "#89b4fa", "#f5c2e7", "#94e2d5", "#a6adc8"
  },
  indexed = {
    [16] = "#b4befe",
    [17] = "#cba6f7",
    [18] = "#74c7ec"
  }
}

-- Tab bar customization
config.use_fancy_tab_bar = true
config.tab_bar_at_bottom = true
config.hide_tab_bar_if_only_one_tab = true

config.colors.tab_bar = {
  background = "#11111b",
  active_tab = {
    bg_color = "#cba6f7",
    fg_color = "#11111b",
  },
  inactive_tab = {
    bg_color = "#181825",
    fg_color = "#cdd6f4",
  },
  new_tab = {
    bg_color = "#11111b",
    fg_color = "#cdd6f4",
  },
}

-- Load theme from a file if desired (assuming current-theme.conf in ~/.config/wezterm)
-- config.color_scheme = wezterm.color.load_scheme_from_file("current-theme.conf")
return config

