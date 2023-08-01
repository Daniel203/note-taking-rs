use leptos::*;
use shared::models::note::Note;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let post_call = move |_| {
        let note = Note::default();
        let client = reqwest::Client::new();

        let res = client
            .post("http://localhost:8000/api/notes")
            .json(&note)
            .send();

        return;
    };

    view! { cx,
        <button on:click=post_call >
            "Add a note"
        </button>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}
