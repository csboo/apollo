use dioxus::prelude::*;

use crate::{
    app::{
        AuthState, Message,
        utils::{popup_error, popup_normal},
    },
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

pub fn load_title(mut title: Signal<Option<String>>, message: Signal<Option<(Message, String)>>) {
    use_future(move || async move {
        let result = crate::backend::endpoints::event_title()
            .await
            .inspect_err(|e| popup_error(message, format!("Hiba: {}", e)))
            .ok();

        title.set(result.unwrap_or_else(|| "Apollo esemény".into()).into());
    });
}

pub fn check_auth(mut auth: Signal<AuthState>, message: Signal<Option<(Message, String)>>) {
    use_future(move || async move {
        if let Ok(name) = crate::backend::endpoints::auth_state().await {
            auth.write().username = name.clone();
            auth.write().joined = true;
            popup_normal(message, format!("Üdv újra, {name}"));
        }
    });
}

pub fn subscribe_stream(
    mut teams_state: Signal<Vec<(String, SolvedPuzzles)>>,
    mut puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
) {
    use_future(move || async move {
        let mut stream = crate::backend::endpoints::state_stream().await?;
        while let Some(Ok((new_team_state, new_puzzles))) = stream.next().await {
            let mut puzzles_sorted: Vec<_> = new_puzzles.into_iter().collect();
            puzzles_sorted.sort();

            let mut teams_sorted: Vec<_> = new_team_state.into_iter().collect();
            teams_sorted.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));

            puzzles.set(puzzles_sorted);
            teams_state.set(teams_sorted);
        }
        dioxus::Ok(())
    });
}

pub fn auto_hide_message(mut message: Signal<Option<(Message, String)>>) {
    use_effect(move || {
        if message.read().is_some() {
            spawn(async move {
                #[cfg(feature = "web")]
                gloo_timers::future::sleep(core::time::Duration::from_secs(5)).await;
                #[cfg(feature = "desktop")]
                tokio::time::sleep(core::time::Duration::from_secs(5)).await;

                message.set(None);
            });
        }
    });
}
