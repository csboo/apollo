#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

pub mod admin;
mod components;
pub mod home;

use dioxus::prelude::*;

use crate::routes::ApolloRoutes;
use components::toast::ToastProvider;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn App() -> Element {
    use dioxus_router::Router;

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: DX_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        ToastProvider {
            Router::<ApolloRoutes> {}
        }
    }
}
