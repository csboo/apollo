use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::{
    app::components::tailwind_constants::{BUTTON, FLASH, INPUT},
    app::home::{AuthState, actions},
};

#[component]
pub fn Login(mut auth: Signal<AuthState>) -> Element {
    let toast_api = use_toast();
    let auth_current = auth.read().clone();
    let mut is_contestant_ready = use_signal(|| false);
    use_future(move || async move {
        *is_contestant_ready.write() = crate::backend::endpoints::contestant_ready().await.is_ok()
    });

    rsx! {
        // Join form
        if !auth.read().is_admin {
            input { class: INPUT,
                r#type: "text",
                placeholder: "Csapatnév",
                value: "{auth_current.username}",
                cursor: "text",
                oninput: move |evt| auth.write().username = evt.value()
            }
            button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_user_join(auth, toast_api), "Belépés" }
        } else {
            if !*is_contestant_ready.read() {
                input { class: "{INPUT}",
                    r#type: "password",
                    placeholder: "Elsődleges admin jelszó",
                    value: "{auth_current.init_password}",
                    cursor: "text",
                    oninput: move |evt| auth.write().init_password = evt.value()
                }
            }
            input { class: "{INPUT}",
                r#type: "password",
                placeholder: "Admin jelszó",
                value: "{auth_current.password}",
                cursor: "text",
                oninput: move |evt| auth.write().password = evt.value()
            }
            button { class: "{BUTTON} {FLASH}", cursor: "pointer", onclick: actions::handle_admin_join(auth, toast_api), "Belépés" }
        }
    }
}
