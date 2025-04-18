use std::fmt::Display;

use maud::Render;

pub enum AlertStyle {
    Primary,
    Info,
    Success,
    Warning,
    Danger,
}

impl Display for AlertStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertStyle::Primary => write!(f, "message is-primary"),
            AlertStyle::Info => write!(f, "message is-info"),
            AlertStyle::Success => write!(f, "message is-success"),
            AlertStyle::Warning => write!(f, "message is-warning"),
            AlertStyle::Danger => write!(f, "message is-danger"),
        }
    }
}

pub struct Alert<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub style: AlertStyle,
}

impl Render for Alert<'_> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class=(self.style) {
                div class="message-header" {
                    p { (self.title) }
                }
                div class="message-body" {
                    p { (self.content) }
                }
            }
        }
    }
}
