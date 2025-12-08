mod app;
mod backend;
mod components;

fn main() {
    dioxus::logger::initialize_default();

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        backend::prepare_startup().await;

        let router = dioxus::server::router(app::App);
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}
