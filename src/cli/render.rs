//! This module contains an interface to nicely render to a frontend.

use std::fmt::Display;

// colors
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const LGREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const GREEN: &str = "\x1b[32m";
const WHITE: &str = "\x1b[37m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";

/// All possible styles for the strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Style {
    /// Color white.
    White,
    /// Color yellow.
    Yellow,
    /// Color red.
    Red,
    /// Color light green.
    LGreen,
    /// Color green.
    Green,
    /// Color blue.
    Blue,
    /// Color purple.
    Purple,

    /// Bold intensity.
    Bold,
    /// Apply underline.
    Underline,
}

/// Options for the `Renderer`.
#[derive(Debug, Clone, Default)]
pub struct RendererOptions {
    pub has_colors: bool,
}

/// Struct that implements `Renderer` to write to the terminal.
#[derive(Debug, Clone, Default)]
pub struct TermRenderer {
    options: RendererOptions,
}

/// Render strings nicely to a frontend, be it a terminal or whatever.
pub trait Renderer {
    /// Allow setting options for the `Renderer`.
    fn options(&mut self) -> &mut RendererOptions;

    /// Write a nicely formatted string to the frontend.
    fn write(&mut self, str: impl Display, styles: &[Style]);

    /// Write with an ending newline.
    fn writeln(&mut self, str: impl Into<String>, styles: &[Style]) {
        self.write(str.into(), styles);
        self.write("\n", &[]);
    }

    /// Write an error to the frontend.
    fn error(&mut self, str: impl Display);

    /// Write a warning to the frontend.
    fn warning(&mut self, str: impl Display);
}

impl RendererOptions {
    /// Create new option struct.
    pub fn new(has_colors: bool) -> Self {
        Self { has_colors }
    }
}

impl TermRenderer {
    /// Create new struct.
    pub fn new(options: RendererOptions) -> Self {
        Self { options }
    }
}

impl Renderer for TermRenderer {
    fn options(&mut self) -> &mut RendererOptions {
        &mut self.options
    }

    fn write(&mut self, str: impl Display, styles: &[Style]) {
        let colors = styles
            .iter()
            .map(|f| match f {
                Style::White => WHITE,
                Style::Yellow => YELLOW,
                Style::Red => RED,
                Style::LGreen => LGREEN,
                Style::Green => GREEN,
                Style::Blue => BLUE,
                Style::Purple => PURPLE,
                Style::Bold => BOLD,
                Style::Underline => UNDERLINE,
            })
            .collect::<Vec<_>>()
            .join("");
        match self.options.has_colors {
            true => print!("{colors}{str}{RESET}"),
            false => print!("{str}"),
        };
    }

    fn error(&mut self, str: impl Display) {
        match self.options.has_colors {
            true => eprint!("{RED}{BOLD}ERROR: {str}{RESET}"),
            false => eprint!("ERROR: {str}"),
        };
    }

    fn warning(&mut self, str: impl Display) {
        match self.options.has_colors {
            true => eprint!("{YELLOW}{BOLD}WARNING: {str}{RESET}"),
            false => eprint!("WARNING: {str}"),
        };
    }
}
