use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::tailwind_constants::{BUTTON, INPUT},
    app::home::{AuthState, actions},
};

#[derive(Clone, PartialEq)]
pub enum UserType {
    Admin,
    Player,
}

#[component]
pub fn Login(mut auth: Signal<AuthState>, usertype: UserType) -> Element {
    let toast_api = use_toast();
    let auth_current = auth.read().clone();

    rsx! {
        div { class: "space-y-1.5",
            label { class: "block text-sm font-medium text-(--text-secondary)",
                if usertype == UserType::Admin { "Admin név" } else { "Csapatnév" }
            }
            input { class: INPUT,
                r#type: "text",
                placeholder: if usertype == UserType::Admin { "Add meg az admin nevet" } else { "Add meg a csapatnevet" },
                value: "{auth_current.username}",
                oninput: move |evt| auth.write().username = evt.value()
            }
        }

        if auth_current.show_password_prompt {
            div { class: "space-y-1.5",
                label { class: "block text-sm font-medium text-(--text-secondary)",
                    "Jelszó"
                }
                input { class: "{INPUT}",
                    r#type: "password",
                    placeholder: "Add meg a jelszót",
                    value: "{auth_current.password}",
                    oninput: move |evt| auth.write().password = evt.value()
                }
            }
        }

        div { class: "flex items-end",
            if usertype == UserType::Admin {
                button { class: "{BUTTON}",
                    onclick: actions::handle_admin_join(auth, toast_api),
                    "Bejelentkezés"
                }
            } else {
                button { class: "{BUTTON}",
                    onclick: actions::handle_user_join(auth, toast_api),
                    "Bejelentkezés"
                }
            }
        }
    }
}
