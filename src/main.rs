mod app;
mod backend;

fn main() {
    dioxus::logger::initialize_default();

    eprintln!("{}", env!("BANNER").replace(r"\n", "\n").trim_matches('"')); // had to be escaped, see build.rs

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::cli_config::fullstack_address_or_localhost as dx_server_addr;
        use dioxus::prelude::*;

        info!("serving on http://{}", dx_server_addr());

        let router = dioxus::server::router(app::App);
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}
