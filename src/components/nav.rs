use crate::Route;
use dioxus::prelude::*;

static ICON: Asset = asset!("/assets/filedrop.svg");

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id: "title",
            img { src: ICON }
            Link { to: Route::AppView,
                "FileDrop"
            }
        }
        Outlet::<Route> {}
    }
}