use leptos::*;
mod app;
mod components;
mod api;

use app::App;

fn main() {
    mount_to_body(|| view! { <App /> });
}

