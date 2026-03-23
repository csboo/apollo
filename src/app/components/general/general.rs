use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::tailwind_constants::{BUTTON, FLASH, INPUT},
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
        // Join form
        input { class: INPUT,
            r#type: "text",
            placeholder: if usertype == UserType::Admin { "Admin név" } else { "Csapatnév"},
            value: "{auth_current.username}",
            cursor: "text",
            oninput: move |evt| auth.write().username = evt.value()
        }

        if auth_current.show_password_prompt {
            input { class: "{INPUT}",
                r#type: "password",
                placeholder: "Admin jelszó",
                value: "{auth_current.password}",
                cursor: "text",
                oninput: move |evt| auth.write().password = evt.value()
            }
        }

        if usertype == UserType::Admin {
            button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_admin_join(auth, toast_api), "Belépés" }
        } else {
            button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_user_join(auth, toast_api), "Belépés" }
        }
    }
}
