use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
static ICON: Asset = asset!("/assets/filedrop.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut connected = use_signal(|| false);
    rsx! {
        document::Stylesheet { href: CSS}
        div {
            id: "title",
            img { src: ICON },
            "FileDrop"
        }

        div { id: "container",
            if connected() {
                div {
                    id: "dropzone",
                    "Drag and drop files here or click to select"
                }
                div { id: "file-list",
                    p { "No files selected yet." }
                }
                button {
                    id: "send-button",
                    "Send Files"
                }
            } else {
                div {
                    id: "qr-section",
                    img {
                        id: "qr-img",
                        src: "/assets/qr.png",
                        alt: "Scan to connect"
                    }
                    button {
                        id: "connect-btn",
                        onclick: move |_| connected.set(true),
                        "Simulate Connection"
                    }
                }
            }
        }
    }
}
