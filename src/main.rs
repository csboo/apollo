mod app;
mod backend;
mod components;

fn main() {
    dioxus::logger::initialize_default();

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::cli_config as dxconf;
        use dioxus::prelude::*;

        backend::prepare_startup().await;
        info!("serving on {}", dxconf::fullstack_address_or_localhost());

        let router = dioxus::server::router(app::App);
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}
