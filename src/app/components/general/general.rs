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
    let mut is_contestant_ready = use_signal(|| false);
    use_future(move || async move {
        *is_contestant_ready.write() = crate::backend::endpoints::contestant_ready().await.is_ok()
    });

    rsx! {
        if usertype == UserType::Player {
            div { class: "space-y-1.5",
                label { class: "block text-sm font-medium text-(--text-secondary)",
                    "Csapatnév"
                }
                input { class: INPUT,
                    r#type: "text",
                    placeholder: "Add meg a csapatnevet",
                    value: "{auth().username}",
                    oninput: move |evt| auth.write().username = evt.value()
                }
            }
        }

        if usertype == UserType::Admin {
            div { class: "space-y-1.5",
                label { class: "block text-sm font-medium text-(--text-secondary)",
                    "Jelszó"
                }
                if !*is_contestant_ready.read() {
                    input { class: "{INPUT}",
                        r#type: "password",
                        placeholder: "Elsődleges admin jelszó",
                        value: "{auth().init_password}",
                        cursor: "text",
                        oninput: move |evt| auth.write().init_password = evt.value()
                    }
                }
                input { class: "{INPUT}",
                    r#type: "password",
                    placeholder: "Add meg a jelszót",
                    value: "{auth().password}",
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
