use std::sync::Arc;

use crate::components::note_component::NoteComponent;
use leptos::*;
use shared::models::note::Note;

pub mod components;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (notes, set_notes) = create_signal::<Vec<Note>>(cx, vec![]);

    let get_notes = create_action(cx, |_| async {
        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:8081/api/notes").send().await;

        // set_notes.clone().set(res.unwrap().json::<Vec<Note>>().await.unwrap());
    });

    let add_note = create_action(cx, |_| async {
        let client = reqwest::Client::new();
        let note = Note::default();

        let res = client
            .post("http://127.0.0.1:8081/api/notes")
            .json(&note)
            .send()
            .await;

        return ();
    });

    view! { cx,
        <div><button on:click=move |_| { get_notes.dispatch(""); }> "Get Notes" </button></div>
        <div><button on:click=move |_| { add_note.dispatch(""); }> "Add a note" </button></div>

        <div>
            <For
                each = move || notes.get()
                key = |note| note.id
                view = move |cx, note: Note| {
                        view! {cx, <NoteComponent note=note/>}
                    }
                />
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}
