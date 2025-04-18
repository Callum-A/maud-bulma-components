use std::fmt::Display;

use maud::{Render, html};

use crate::button::Button;

pub enum HtmxMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Display for HtmxMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmxMethod::GET => write!(f, "hx-get"),
            HtmxMethod::POST => write!(f, "hx-post"),
            HtmxMethod::PUT => write!(f, "hx-put"),
            HtmxMethod::DELETE => write!(f, "hx-delete"),
        }
    }
}

pub struct HtmxForm<'a, T: Render> {
    pub method: HtmxMethod,
    pub url: &'a str,
    pub hx_trigger: Option<&'a str>,
    pub hx_target: Option<&'a str>,
    pub hx_swap: Option<&'a str>,
    pub inputs: Vec<T>,
    pub submit_button: Button<'a>,
}

impl<T: Render> Render for HtmxForm<'_, T> {
    fn render(&self) -> maud::Markup {
        let form_classes = "box";
        match self.method {
            HtmxMethod::GET => {
                html! {
                    form class=(form_classes) hx-get=(self.url) hx-trigger=[self.hx_trigger] hx-target=[self.hx_target] hx-swap=[self.hx_swap] {
                        @for input in &self.inputs {
                            (input)
                        }

                        (self.submit_button)
                    }
                }
            }
            HtmxMethod::POST => {
                html! {
                    form class=(form_classes) hx-post=(self.url) hx-trigger=[self.hx_trigger] hx-target=[self.hx_target] hx-swap=[self.hx_swap] {
                        @for input in &self.inputs {
                            (input)
                        }

                        (self.submit_button)
                    }
                }
            }
            HtmxMethod::PUT => {
                html! {
                    form class=(form_classes) hx-put=(self.url) hx-trigger=[self.hx_trigger] hx-target=[self.hx_target] hx-swap=[self.hx_swap] {
                        @for input in &self.inputs {
                            (input)
                        }

                        (self.submit_button)
                    }
                }
            }
            HtmxMethod::DELETE => {
                html! {
                    form class=(form_classes) hx-delete=(self.url) hx-trigger=[self.hx_trigger] hx-target=[self.hx_target] hx-swap=[self.hx_swap] {
                        @for input in &self.inputs {
                            (input)
                        }

                        (self.submit_button)
                    }
                }
            }
        }
    }
}
