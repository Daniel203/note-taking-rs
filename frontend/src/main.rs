use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let post_call = move |_| {
        todo!()
    };

    view! { cx,
        <button on:click=post_call >
            "Adda a reminder"
        </button>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}
