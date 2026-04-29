//! Module to provide a backend indipendent way to colorize and output strings.

use crate::core::error::Result;

#[macro_export]
macro_rules! style {
    ($($arg:expr),+ $(,)?) => {{
        $(
            let _styled: &mut dyn $crate::style::Style = &mut $arg;
            _styled.visualize()?;
        )+
        Ok(())
    }};
}

/// Trait providing all functionalities
pub trait Style {
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

    /// Visualize the styled output on the frontend.
    fn visualize(&mut self) -> Result<()>;
}

/// Implementation of Style to visualize the text on the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermStyle {
    text: String,
    is_color_set: bool,
    term_color: &'static str,
    term_decor: &'static str,
    text_type: TextType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextType {
    Normal,
    Error,
    Warning,
}

const WHITE: &'static str = "\x1b[37m";
const RED: &'static str = "\x1b[31m";
const LGREEN: &'static str = "\x1b[32m";
const GREEN: &'static str = "\x1b[32m";
const YELLOW: &'static str = "\x1b[33m";
const BLUE: &'static str = "\x1b[34m";
const PURPLE: &'static str = "\x1b[35m";
const BOLD: &'static str = "\x1b[1m";
const UNDERLINE: &'static str = "\x1b[4m";

impl TermStyle {
    /// Create new TermStyle
    pub fn new(text: String) -> Self {
        Self {
            text,
            is_color_set: false,
            term_color: "",
            term_decor: "",
            text_type: TextType::Normal,
        }
    }
}

impl Style for TermStyle {
    fn white(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = WHITE;
        self
    }

    fn red(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = RED;
        self
    }

    fn lgreen(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = LGREEN;
        self
    }

    fn green(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = GREEN;
        self
    }

    fn yellow(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = YELLOW;
        self
    }

    fn blue(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = BLUE;
        self
    }

    fn purple(&mut self) -> &mut Self {
        self.is_color_set = true;
        self.term_color = PURPLE;
        self
    }

    fn bold(&mut self) -> &mut Self {
        self.term_decor = BOLD;
        self
    }

    fn underline(&mut self) -> &mut Self {
        self.term_decor = UNDERLINE;
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

    fn visualize(&mut self) -> Result<()> {
        todo!()
    }
}

impl From<String> for TermStyle {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for TermStyle {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}
