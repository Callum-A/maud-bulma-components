use axum::routing::get;
use maud::html;
use maud_component_library::{
    alert::{Alert, AlertStyle},
    button::{Button, ButtonStyle, ButtonType},
    htmx_form::{HtmxForm, HtmxMethod},
    input::{TextInput, TextInputType},
    navbar::{AuthContext, Navbar, NavbarItem},
};

pub struct MyUser {
    pub id: u32,
    pub name: String,
}

impl AuthContext for MyUser {
    fn is_authenticated(&self) -> bool {
        // Implement your authentication logic here
        true
    }

    fn display_name(&self) -> Option<String> {
        Some(self.name.clone())
    }
}

async fn index() -> impl axum::response::IntoResponse {
    let content = html! {
        html {
            head {
                title { "My App" }
                meta charset="utf-8" {}
                meta name="viewport" content="width=device-width, initial-scale=1.0" {}
            }
        }
        // CDNs are not suitable for production!
        link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@1.0.2/css/bulma.min.css" {}
        script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"{}
        (Navbar {
            title: html! {
                h1 class="title" { "My App" }
            },
            auth_context: MyUser { id: 1, name: "John Doe".to_string() },
            items_requiring_auth: vec![NavbarItem::new("Profile", "/profile")],
            items_not_requiring_auth: vec![],
            items_always_visible: vec![
                NavbarItem::new("Home", "/"),
                NavbarItem::new("About", "/about"),
                NavbarItem::new("Contact", "/contact"),
            ],
            signup_url: "/signup",
            login_url: "/login",
            logout_url: "/logout"
        })
        main class="container" {
            div class="container is-max-tablet" {
                (HtmxForm {
                    method: HtmxMethod::POST,
                    url: "/",
                    hx_trigger: None,
                    hx_target: None,
                    hx_swap: None,
                    inputs: vec![TextInput {
                        html_type: TextInputType::Text,
                        name: "name",
                        label: "Name",
                        value: None,
                        placeholder: Some("Enter your name"),
                        required: true,
                    }],
                    submit_button: Button::new("Primary", ButtonStyle::Primary, ButtonType::Submit)
                })
            }
            (Alert {
                title: "Alert Title",
                content: "This is an alert message.",
                style: AlertStyle::Primary,
            })
        }
    };

    let response = axum::response::Html(content.into_string());
    response
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3000";

    let tcp_listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    let app = axum::Router::new().route("/", get(index));

    axum::serve(tcp_listener, app).await.unwrap();
}
