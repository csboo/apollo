use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx!(
        div { class: "loading",
            div { class: "flex flex-col items-center gap-6",
                div { class: "relative",
                    div { class: "w-12 h-12 border-2 border-(--border-default) border-t-(--accent-primary) rounded-full animate-spin" }
                }
                p { class: "text-(--text-secondary) font-medium",
                    "Várakozás az Apollo kiszolgálóra..."
                }
            }
        }
    )
}
