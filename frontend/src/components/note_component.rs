use leptos::*;
use shared::models::note::Note;

#[component]
pub fn NoteComponent(cx: Scope, note: Note) -> impl IntoView {
    return view! {cx, <div> "Note Component" </div>};
}
