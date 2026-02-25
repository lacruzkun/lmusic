use dioxus::prelude::*;


const HOME_CSS: Asset = asset!("/assets/home.css");
const ALBUM_ART: Asset = asset!("/assets/tpab.png");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet{ href: HOME_CSS }
        Playback {}
        section {
            id: "home",
            Navigation {}
            Main {}
            Queue {}
        }
    }
}

#[component]
fn Navigation() -> Element {
    rsx!{
        section {
            id: "navigation",
            div {
                id: "nav-title",
                h3 { "Home"}
            }
            div {
                id: "nav-button",
                class: "active",
                div {class: "bar",}
                p { "Songs" }
            }
            div {
                id: "nav-button",
                p { "Artists" }
            }
            div {
                id: "nav-button",
                p { "Albums" }
            }

            div {
                id: "nav-title",
                h3 { "Library"}
            }
            div {
                id: "nav-button",
                p { "Playlist" }
            }
            div {
                id: "nav-button",
                p { "Favourite" }
            }
            div {
                id: "nav-button",
                p { "Recently Played" }
            }

            div {
                id: "nav-title",
                h3 { "Other"}
            }
            div {
                id: "nav-button",
                p { "Setting" }
            }
        }
    }
}

#[component]
fn Main() -> Element {
    rsx!{
        section {
            id: "view",
            h1 { "Hello from main" }
        }
    }
}

#[component]
fn Queue() -> Element {
    rsx!{
        section {
            id: "queue",
            h1 { "Hello from queue" }
        }
    }
}

#[component]
fn Playback() -> Element {
    rsx!{
        section{
            id: "playback",
            div { id: "track-info",
                div {id: "track-album-art",
                    img {src: ALBUM_ART}
                }
                div { id: "tracks",
                    div {id: "track-title",
                        h3 { "King Kunta" }
                    }
                    div {id: "track-artist",
                        p { "Kendrick Lamar" }
                    }
                }
            }

            div { id: "track-status",
            }

            div { id: "track-control",
            }
        }
    }
}
