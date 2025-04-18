use std::fmt::Display;

use maud::Render;

pub enum TextInputType {
    Text,
    Email,
    Password,
}

impl Display for TextInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextInputType::Text => write!(f, "text"),
            TextInputType::Email => write!(f, "email"),
            TextInputType::Password => write!(f, "password"),
        }
    }
}

pub struct TextInput<'a> {
    pub html_type: TextInputType,
    pub name: &'a str,
    pub label: &'a str,
    pub value: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub required: bool,
}

impl Render for TextInput<'_> {
    fn render(&self) -> maud::Markup {
        let required = if self.required {
            Some("required")
        } else {
            None
        };
        maud::html! {
            div class="field" {
                label class="label" for=(self.name) { (self.label) }
                div class="control" {
                    input class="input" type=(self.html_type) name=(self.name) value=[self.value] placeholder=[self.placeholder] required=[required] {}
                }
            }
        }
    }
}
