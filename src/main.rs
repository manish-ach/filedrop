use dioxus::prelude::*;

mod components;
mod backend;

use components::{AppView, NavBar};

static CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    AppView,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS}
        Router::<Route> {}
    }
}
