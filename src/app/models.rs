use dioxus_primitives::toast::Toasts;

use crate::app::utils::popup_error;

#[derive(Default, Clone, PartialEq)]
pub struct AuthState {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) joined: bool,
    pub(crate) is_admin: bool,
    pub(crate) show_password_prompt: bool,
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
        if self.is_admin && self.password.is_empty() {
            popup_error(toast_api, "A jelszó nem lehet üres");
            return false;
        }
        true
    }

    pub fn validate(&self, toast_api: Toasts) -> bool {
        if !self.validate_username(toast_api) {
            return false;
        };
        if !self.validate_password(toast_api) {
            return false;
        };

        true
    }
}
