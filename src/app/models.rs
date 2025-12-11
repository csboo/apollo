#[derive(Default)]
pub struct AuthState {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) joined: bool,
    pub(crate) is_admin: bool,
    pub(crate) show_password_prompt: bool,
}

#[derive(Clone, PartialEq)]
pub enum Message {
    MsgNorm,
    MsgErr,
}
