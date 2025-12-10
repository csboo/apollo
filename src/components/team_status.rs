use dioxus::prelude::*;

#[component]
pub fn TeamStatus(team: String, points: u32) -> Element {
    rsx!(
        span {
            "csapat: {team}"
        }
        br {  }
        span {
            "pontok: {points}"
        }
    )
}
