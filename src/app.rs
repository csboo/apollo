use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use dioxus_i18n::unic_langid::LanguageIdentifier;
use dioxus_i18n::unic_langid::langid;
use dioxus_i18n::{prelude::*, t};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(target_arch = "wasm32")]
fn browser_language() -> Option<LanguageIdentifier> {
    let from_browser = web_sys::window()
        .map(|w| w.navigator())
        .and_then(|n| n.language())
        .and_then(|raw| raw.parse::<LanguageIdentifier>().ok());

    if let Some(lang) = from_browser {
        if lang.language.as_str() == "en" {
            return Some(langid!("en-US"));
        }
        if lang.language.as_str() == "hu" {
            return Some(langid!("hu-HU"));
        }
    }

    None
}

#[component]
pub fn App() -> Element {
    let mut i18n = use_init_i18n(|| {
        // Keep first server/client render identical; set browser locale after hydration.
        I18nConfig::new(langid!("hu-HU"))
            .with_fallback(langid!("hu-HU"))
            .with_locale((langid!("hu-HU"), include_str!("../assets/i18n/hu-HU.ftl")))
            .with_locale((langid!("en-US"), include_str!("../assets/i18n/en-US.ftl")))
    });
    #[cfg(target_arch = "wasm32")]
    let mut browser_language_applied = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if browser_language_applied() {
            return;
        }
        browser_language_applied.set(true);

        if let Some(lang) = browser_language() {
            if i18n.language() != lang {
                i18n.set_language(lang);
            }
        }
    });

    let current_language = i18n.language().to_string();
    let is_hungarian = current_language.starts_with("hu");

    rsx! {
        head {
            document::Link { rel: "icon", href: FAVICON }
            document::Title { {t!("app-title")} }
            // document::Link { rel: "stylesheet", href: MAIN_CSS }
            // document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        }

        body {
            div {
                if !is_hungarian {
                    button { onclick: move |_| i18n.set_language(langid!("hu-HU")),
                        {t!("lang-hungarian")}
                    }
                } else {
                    button { onclick: move |_| i18n.set_language(langid!("en-US")), {t!("lang-english")} }
                }
            }
            h1 { {t!("app-heading")} }
            {t!("app-client-is")}
            " "
            a { href: "https://github.com/csboo/apollo/pull/4", {t!("app-wip-link")} }
            br {}
            {t!("app-meanwhile")}
            " "
            a { href: "https://github.com/csboo/apollo", {t!("app-github-link")} }
        }
    }
}
