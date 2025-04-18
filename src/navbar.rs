use maud::Render;

pub struct NavbarItem<'a> {
    pub text: &'a str,
    pub url: &'a str,
}

impl<'a> NavbarItem<'a> {
    pub fn new(text: &'a str, url: &'a str) -> Self {
        Self { text, url }
    }
}

pub trait AuthContext {
    fn is_authenticated(&self) -> bool;
    fn display_name(&self) -> Option<String>;
}

pub struct Navbar<'a, T: Render, U: AuthContext> {
    pub title: T,
    pub auth_context: U,
    pub items_requiring_auth: Vec<NavbarItem<'a>>,
    pub items_not_requiring_auth: Vec<NavbarItem<'a>>,
    pub items_always_visible: Vec<NavbarItem<'a>>,
    pub signup_url: &'a str,
    pub login_url: &'a str,
    pub logout_url: &'a str,
}

impl<'a, T: Render, U: AuthContext> Render for Navbar<'a, T, U> {
    fn render(&self) -> maud::Markup {
        maud::html! {
            nav class="navbar" role="navigation" aria-label="main navigation" x-data="{ open: false }" {
                div class="navbar-brand" {
                    a class="navbar-item" href="/" {
                        (self.title)
                    }

                    a role="button" ":class"="open ? 'navbar-burger is-active' : 'navbar-burger'" aria-label="menu" aria-expanded="false" data-target="navbarMenu" "x-on:click"="open = !open" {
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                    }
                }

                div id="navbarMenu" ":class"="open ? 'navbar-menu is-active' : 'navbar-menu'" {
                    div class="navbar-start" {
                        @for item in &self.items_always_visible {
                            a class="navbar-item" href=(item.url) {
                                (item.text)
                            }
                        }
                        @if !self.auth_context.is_authenticated() {
                            @for item in &self.items_not_requiring_auth {
                                a class="navbar-item" href=(item.url) {
                                    (item.text)
                                }
                            }
                        }
                        @if self.auth_context.is_authenticated() {
                            @for item in &self.items_requiring_auth {
                                a class="navbar-item" href=(item.url) {
                                    (item.text)
                                }
                            }
                        }
                    }

                    div class="navbar-end" {
                        @if self.auth_context.is_authenticated() {
                            @if let Some(display_name) = self.auth_context.display_name() {
                                a class="navbar-item" {
                                    "Hello, " (display_name)
                                }
                            }
                            a class="navbar-item" href=(self.logout_url) {
                                "Logout"
                            }
                        } @else {
                            a class="navbar-item" href=(self.login_url) {
                                "Login"
                            }
                            a class="navbar-item" href=(self.signup_url) {
                                "Sign Up"
                            }
                        }
                    }
                }
            }
        }
    }
}
