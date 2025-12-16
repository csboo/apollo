use dioxus::signals::Signal;

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
    pub fn validate_username(&self, message: Signal<Option<(Message, String)>>) -> bool {
        if self.username.is_empty() {
            popup_error(message, "A csapatnév nem lehet üres");
            return false;
        }
        true
    }
    pub fn validate_password(&self, message: Signal<Option<(Message, String)>>) -> bool {
        if self.is_admin && self.password.is_empty() {
            popup_error(message, "A jelszó nem lehet üres");
            return false;
        }
        true
    }

    pub fn validate(&self, message: Signal<Option<(Message, String)>>) -> bool {
        if !self.validate_username(message) {
            return false;
        };
        if !self.validate_password(message) {
            return false;
        };

        true
    }
}

#[derive(Clone, PartialEq)]
pub enum Message {
    MsgNorm,
    MsgErr,
}
