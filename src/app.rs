use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        head {
            document::Link { rel: "icon", href: FAVICON }
            document::Title { "Apollo" }
            // document::Link { rel: "stylesheet", href: MAIN_CSS }
            // document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        }

        body {
            h1 { "Apollo" }
            "our client is "
            a { href: "https://github.com/csboo/apollo/pull/4", "work-in-progress" }
            br {}
            "meanwhile, make sure to check out our "
            a { href: "https://github.com/csboo/apollo", "github page" }
        }
    }
}
