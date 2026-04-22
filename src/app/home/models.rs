use dioxus_primitives::toast::Toasts;

use crate::app::home::utils::popup_error;

#[derive(Default, Clone, PartialEq)]
pub struct AuthState {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) joined: bool,
}

impl AuthState {
    pub fn validate_username(&self, toast_api: Toasts) -> bool {
        if self.username.is_empty() {
            popup_error(toast_api, "A csapatnév nem lehet üres");
            return false;
        }
        true
    }
    pub fn validate_password(&self, toast_api: Toasts) -> bool {
        if self.password.is_empty() {
            popup_error(toast_api, "A jelszó nem lehet üres");
            return false;
        }
        true
    }

    // TODO more validation (everywhere)
    // pub fn validate(&self, toast_api: Toasts) -> bool {
    //     if !self.validate_username(toast_api) {
    //         return false;
    //     };
    //     if !self.validate_password(toast_api) {
    //         return false;
    //     };

    //     true
    // }
}
