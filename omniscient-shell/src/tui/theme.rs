//! Theme system for consistent styling

use crate::utils::config::ThemeConfig;
use ratatui::style::Color;

pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
}

impl Theme {
    pub fn from_config(config: &ThemeConfig) -> Self {
        Theme {
            name: config.name.clone(),
            background: parse_color(&config.background),
            foreground: parse_color(&config.foreground),
            accent: parse_color(&config.accent),
        }
    }

    pub fn neo_cyan() -> Self {
        Theme {
            name: "NeoCyan".to_string(),
            background: Color::Rgb(11, 14, 16),
            foreground: Color::Rgb(201, 209, 217),
            accent: Color::Rgb(0, 209, 255),
        }
    }
}

fn parse_color(hex: &str) -> Color {
    // Simple hex color parser
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    Color::White
}

impl Default for Theme {
    fn default() -> Self {
        Theme::neo_cyan()
    }
}
