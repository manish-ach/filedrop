use dioxus::prelude::*;

// Assets (you can update paths as per your setup)
static CSS: Asset = asset!("/assets/main.css");
static LOGO: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "app",
            file_transfer_section {}
        }
    }
}

#[component]
fn file_transfer_section() -> Element {
    rsx! {
        div { class: "file-transfer-container",
            div { class: "sidebar",
                div { class: "sidebar-header",
                    img { src: LOGO, alt: "FileDrop", class: "logo" }
                    h2 { class: "app-title", "FileDrop" }
                }
                div { class: "sidebar-menu",
                    div { class: "menu-item active",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7,10 12,15 17,10" }
                            line { x1: "12", y1: "15", x2: "12", y2: "3" }
                        }
                        span { "Send Files" }
                    }
                    div { class: "menu-item",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "17,8 12,3 7,8" }
                            line { x1: "12", y1: "3", x2: "12", y2: "15" }
                        }
                        span { "Receive Files" }
                    }
                    div { class: "menu-item",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14,2 14,8 20,8" }
                            line { x1: "16", y1: "13", x2: "8", y2: "13" }
                            line { x1: "16", y1: "17", x2: "8", y2: "17" }
                            polyline { points: "10,9 9,9 8,9" }
                        }
                        span { "History" }
                    }
                    div { class: "menu-item",
                        svg { width: "20", height: "20", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M12 1v6m0 6v6m11-7h-6m-6 0H1" }
                        }
                        span { "Settings" }
                    }
                }
            }
            
            div { class: "main-content",
                div { class: "content-header",
                    h1 { class: "page-title", "Send Files" }
                    p { class: "page-description", "Transfer files to nearby devices securely and quickly" }
                }
                
                div { class: "content-body",
                    div { class: "upload-section",
                        div { class: "file-drop-zone",
                            div { class: "upload-icon",
                                svg { width: "80", height: "80", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "1.5",
                                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                    polyline { points: "7,10 12,15 17,10" }
                                    line { x1: "12", y1: "15", x2: "12", y2: "3" }
                                }
                            }
                            h3 { class: "upload-title", "Drop files here" }
                            p { class: "upload-description", "or click to browse your files" }
                            button { class: "select-files-btn", "Select Files" }
                        }
                    }
                    
                    div { class: "file-list-section",
                        h3 { class: "section-title", "Selected Files" }
                        div { class: "file-list",
                            div { class: "file-item",
                                div { class: "file-icon",
                                    svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                        path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                        polyline { points: "14,2 14,8 20,8" }
                                    }
                                }
                                div { class: "file-info",
                                    span { class: "file-name", "document.pdf" }
                                    span { class: "file-size", "2.4 MB" }
                                }
                                button { class: "remove-file-btn",
                                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                                    }
                                }
                            }
                            div { class: "file-item",
                                div { class: "file-icon image",
                                    svg { width: "24", height: "24", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                        rect { x: "3", y: "3", width: "18", height: "18", rx: "2", ry: "2" }
                                        circle { cx: "8.5", cy: "8.5", r: "1.5" }
                                        polyline { points: "21,15 16,10 5,21" }
                                    }
                                }
                                div { class: "file-info",
                                    span { class: "file-name", "photo.jpg" }
                                    span { class: "file-size", "1.8 MB" }
                                }
                                button { class: "remove-file-btn",
                                    svg { width: "16", height: "16", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                                    }
                                }
                            }
                        }
                    }
                    
                    div { class: "transfer-actions",
                        button { class: "btn btn-primary", "Start Transfer" }
                        button { class: "btn btn-secondary", "Clear All" }
                    }
                }
            }
        }
    }
}
