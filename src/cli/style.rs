//! Module to provide a backend indipendent way to colorize and output strings.

use crate::core::error::Result;

/// Trait providing all functionalities
pub trait Styler {
    /// Apply white color on text.
    fn white(&mut self) -> &mut Self;
    /// Apply red color on text.
    fn red(&mut self) -> &mut Self;
    /// Apply light green color on text.
    fn lgreen(&mut self) -> &mut Self;
    /// Applu green color on text.
    fn green(&mut self) -> &mut Self;
    /// Apply yellow color on text.
    fn yellow(&mut self) -> &mut Self;
    /// Apply blue color on text.
    fn blue(&mut self) -> &mut Self;
    /// Apply purple color on text.
    fn purple(&mut self) -> &mut Self;

    /// Apply bold on the text.
    fn bold(&mut self) -> &mut Self;
    /// Apply underline on the text.
    fn underline(&mut self) -> &mut Self;

    /// Treat the text as an error (ignores styling).
    fn error(&mut self) -> &mut Self;
    /// Treat the text as a warning (ignores styling).
    fn warning(&mut self) -> &mut Self;

    /// Render the styled output on the frontend.
    fn render(&mut self) -> Result<()>;
}

/// Implementation of Style to render the text on the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermStyler {
    text: String,
    color: bool,
    colors: Vec<&'static str>,
    decorations: Vec<&'static str>,
    text_type: TextType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextType {
    Normal,
    Error,
    Warning,
}

const WHITE: &str = "\x1b[37m";
const RED: &str = "\x1b[31m";
const LGREEN: &str = "\x1b[32m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";
const RESET: &str = "\x1b[m";

impl TermStyler {
    /// Create new TermStyle.
    pub fn new(text: String) -> Self {
        Self::new_with(text, true)
    }

    /// Create new TermStyle with options.
    ///
    /// `color` sets if the string should be colored, or even decorated at all.
    /// It is useful on terminal to disable escape code outputs!
    pub fn new_with(text: String, color: bool) -> Self {
        Self {
            text,
            color,
            colors: Default::default(),
            decorations: Default::default(),
            text_type: TextType::Normal,
        }
    }

    /// Toggle colors.
    pub fn set_colors(&mut self, color: bool) {
        self.color = color;
    }
}

impl Styler for TermStyler {
    fn white(&mut self) -> &mut Self {
        self.colors.push(WHITE);
        self
    }

    fn red(&mut self) -> &mut Self {
        self.colors.push(RED);
        self
    }

    fn lgreen(&mut self) -> &mut Self {
        self.colors.push(LGREEN);
        self
    }

    fn green(&mut self) -> &mut Self {
        self.colors.push(GREEN);
        self
    }

    fn yellow(&mut self) -> &mut Self {
        self.colors.push(YELLOW);
        self
    }

    fn blue(&mut self) -> &mut Self {
        self.colors.push(BLUE);
        self
    }

    fn purple(&mut self) -> &mut Self {
        self.colors.push(PURPLE);
        self
    }

    fn bold(&mut self) -> &mut Self {
        self.decorations.push(BOLD);
        self
    }

    fn underline(&mut self) -> &mut Self {
        self.decorations.push(UNDERLINE);
        self
    }

    fn error(&mut self) -> &mut Self {
        self.text_type = TextType::Error;
        self
    }

    fn warning(&mut self) -> &mut Self {
        self.text_type = TextType::Warning;
        self
    }

    fn render(&mut self) -> Result<()> {
        match self.text_type {
            TextType::Normal => {
                assert!(!self.colors.is_empty(), "No colors set!");
                let colors = self.colors.join("");
                let decorations = self.decorations.join("");
                if self.color {
                    println!("{colors}{decorations}{}{RESET}", self.text);
                } else {
                    println!("{}", self.text);
                }
            }
            TextType::Error => {
                assert!(self.colors.is_empty(), "Error can't have colors too!");
                assert!(
                    self.decorations.is_empty(),
                    "Error can't have decorations too!"
                );
                if self.color {
                    eprintln!("{RED}{BOLD}ERROR: {}{RESET}", self.text);
                } else {
                    eprintln!("ERROR: {}", self.text);
                }
            }
            TextType::Warning => {
                assert!(self.colors.is_empty(), "Warning can't have colors too!");
                assert!(
                    self.decorations.is_empty(),
                    "Warning can't have decorations too!"
                );
                if self.color {
                    eprintln!("{YELLOW}{BOLD}WARNING: {}{RESET}", self.text);
                } else {
                    eprintln!("WARNING: {}", self.text);
                }
            }
        }
        Ok(())
    }
}

impl From<String> for TermStyler {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for TermStyler {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}
