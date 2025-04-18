use std::fmt::Display;

use maud::{Render, html};

pub enum ButtonStyle {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonType::Submit => write!(f, "submit"),
            ButtonType::Reset => write!(f, "reset"),
            ButtonType::Button => write!(f, "button"),
        }
    }
}

impl Display for ButtonStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonStyle::Primary => write!(f, "button is-primary"),
            ButtonStyle::Link => write!(f, "button is-link"),
            ButtonStyle::Info => write!(f, "button is-info"),
            ButtonStyle::Success => write!(f, "button is-success"),
            ButtonStyle::Warning => write!(f, "button is-warning"),
            ButtonStyle::Danger => write!(f, "button is-danger"),
        }
    }
}

pub struct Button<'a> {
    pub text: &'a str,
    pub style: ButtonStyle,
    pub button_type: ButtonType,
}

impl<'a> Button<'a> {
    pub fn new(text: &'a str, style: ButtonStyle, button_type: ButtonType) -> Self {
        Self {
            text,
            style,
            button_type,
        }
    }
}

impl Render for Button<'_> {
    fn render(&self) -> maud::Markup {
        html! {
            button class=(self.style) type=(self.button_type) {
                (self.text)
            }
        }
    }
}
