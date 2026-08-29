use crate::app::admin::Admin;
use crate::app::home::Home;
use dioxus::prelude::*;
use dioxus_router::Routable;

#[derive(Routable, Clone, PartialEq)]
pub enum ApolloRoutes {
    #[route("/")]
    Home {},

    #[route("/admin")]
    Admin {},
}
