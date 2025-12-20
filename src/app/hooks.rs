use dioxus::prelude::*;

use crate::{
    app::{AuthState, utils::get_points_of},
    backend::models::{PuzzleId, PuzzleValue, SolvedPuzzles},
};

pub fn load_title(mut title: Signal<Option<String>>) {
    use_future(move || async move {
        let result = crate::backend::endpoints::event_title()
            .await
            // .inspect_err(|e| popup_error(message, format!("Hiba: {}", e))) //TODO WARN
            .ok();

        title.set(result.unwrap_or_else(|| "Apollo esemény".into()).into());
    });
}

pub fn check_auth(mut auth: Signal<AuthState>) {
    use_future(move || async move {
        if let Ok(name) = crate::backend::endpoints::auth_state().await {
            auth.write().username = name.clone();
            auth.write().joined = true;
            // popup_normal(message, format!("Üdv újra, {name}")); //TODO WARN
        }
    });
}

pub fn subscribe_stream(
    mut teams_state: Signal<Vec<(String, SolvedPuzzles)>>,
    mut puzzles: Signal<Vec<(PuzzleId, PuzzleValue)>>,
) {
    use_future(move || async move {
        let mut stream = crate::backend::endpoints::state_stream().await?; // TODO WARN error handling
        while let Some(Ok((new_team_state, new_puzzles))) = stream.next().await {
            let mut puzzles_sorted: Vec<_> = new_puzzles.into_iter().collect();
            puzzles_sorted.sort_by(|p1, p2| p1.1.cmp(&p2.1).then_with(|| p1.0.cmp(&p2.0)));

            let mut teams_sorted: Vec<_> = new_team_state.into_iter().collect();
            teams_sorted.sort_by(|a, b| {
                get_points_of(b, puzzles.read().clone())
                    .cmp(&get_points_of(a, puzzles.read().clone()))
                    .then_with(|| a.0.cmp(&b.0))
            });

            puzzles.set(puzzles_sorted);
            teams_state.set(teams_sorted);
        }
        dioxus::Ok(())
    });
}
