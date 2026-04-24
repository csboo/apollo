use dioxus::prelude::*;

#[component]
pub fn TeamStatus(team: String, points: u32) -> Element {
    rsx!(
        div { class: "bg-(--bg-elevated) rounded-xl border border-(--border-subtle) p-6",
            div { class: "flex items-center justify-between",
                div {
                    p { class: "text-sm text-(--text-muted) mb-1",
                        "Csapat"
                    }
                    p { class: "text-xl font-medium text-(--text-primary)",
                        "{team}"
                    }
                }
                div { class: "text-right",
                    p { class: "text-sm text-(--text-muted) mb-1",
                        "Pontszám"
                    }
                    p { class: "text-3xl font-semibold text-(--accent-primary)",
                        "{points}"
                    }
                }
            }
        }
    )
}
