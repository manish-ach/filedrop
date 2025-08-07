use dioxus::prelude::*;

#[component]
pub fn AppView() -> Element {
    let mut connected = use_signal(|| false);
    rsx! {
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