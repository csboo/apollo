use dioxus::prelude::*;

use crate::app::Message;

#[component]
pub fn MessagePopup(text: String, level: Message) -> Element {
    rsx! {
        div {
            class: "popup",
            id: match level {
                Message::MsgNorm => {
                    "msgnorm"
                },
                Message::MsgErr => {
                    "msgerr"
                },
            },
            "{text}"
        }
    }
}
