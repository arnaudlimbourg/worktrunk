use anstyle::{AnsiColor, Color, Style};

/// Centralized theme for terminal output styling.
///
/// Provides consistent colors and styles across the application.
/// All style definitions are in one place for easy customization.
#[derive(Debug, Clone)]
pub struct Theme {
    // Status/message styles
    pub error: Style,
    pub warning: Style,
    pub hint: Style,
    pub success: Style,

    // Emphasis styles
    pub bold: Style,
    pub dim: Style,
    pub error_bold: Style,

    // Worktree-specific styles
    pub primary: Style,
    pub current: Style,

    // Diff/stat styles
    pub addition: Style,
    pub deletion: Style,
    pub neutral: Style,
}

impl Theme {
    /// Create a new theme with default colors.
    pub fn new() -> Self {
        Self {
            // Status/message styles
            error: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red))),
            warning: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
            hint: Style::new().dimmed(),
            success: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))),

            // Emphasis styles
            bold: Style::new().bold(),
            dim: Style::new().dimmed(),
            error_bold: Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Red)))
                .bold(),

            // Worktree-specific styles
            primary: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
            current: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Magenta))),

            // Diff/stat styles
            addition: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))),
            deletion: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red))),
            neutral: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new();
        // Just verify it doesn't panic
        let _ = theme.error;
        let _ = theme.primary;
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        let _ = theme.warning;
    }
}
