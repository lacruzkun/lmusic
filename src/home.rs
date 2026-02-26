use dioxus::prelude::*;


const HOME_CSS: Asset = asset!("/assets/home.css");
const ALBUM_ART: Asset = asset!("/assets/tpab.png");
const PLAY_SVG: Asset = asset!("/assets/play.svg");
const NEXT_SVG: Asset = asset!("/assets/next.svg");
const SHUFFLE_SVG: Asset = asset!("/assets/shuffle.svg");
const REPEAT_SVG: Asset = asset!("/assets/repeat.svg");
const QUEUE_SVG: Asset = asset!("/assets/queue.svg");
const TIMER_SVG: Asset = asset!("/assets/timer.svg");
const VOLUME_SVG: Asset = asset!("/assets/volume.svg");
const KENNY_JPG: Asset = asset!("/assets/kendrick_lamar.jpg");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet{ href: HOME_CSS }
        section {
            id: "home",
            Navigation {}
            Main {}
            Queue {}
        }
        Playback {}
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
                div {class: "bar",}
                p { "Songs" }
            }
            div {
                id: "nav-button",
                class: "active",
                div {class: "bar",}
                p { "Artists" }
            }
            div {
                id: "nav-button",
                div {class: "bar",}
                p { "Albums" }
            }

            div {
                id: "nav-title",
                div {class: "bar",}
                h3 { "Library"}
            }
            div {
                id: "nav-button",
                div {class: "bar",}
                p { "Playlist" }
            }
            div {
                id: "nav-button",
                div {class: "bar",}
                p { "Favourite" }
            }
            div {
                id: "nav-button",
                div {class: "bar",}
                p { "Recently Played" }
            }

            div {
                id: "nav-title",
                div {class: "bar",}
                h3 { "Other"}
            }
            div {
                id: "nav-button",
                div {class: "bar",}
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
            div { id: "artist-image", 
                img { src: KENNY_JPG }
            }
            h1 {"music"}
            p {"sympathe"}
            p {"music"}
            h1 {"music"}
            h1 {"music"}
            h1 {"music"}
            h1 {"music"}
            h1 {"music"}
            h1 {"music"}
        }
    }
}

#[component]
fn Queue() -> Element {
    rsx!{
        section {
            id: "queue",
            h3 { "Mini Queue" }
            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "1"}
                p { id: "title", "Wesley's Theory"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "2"}
                p { id: "title", "For Free? (Interlude)"}
            }

            div {id: "queue-card", class: "active",
                div {class: "bar",}
                p { id: "position", "3"}
                p { id: "title", "King Kunta"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "4"}
                p { id: "title", "Institutionalized"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "5"}
                p { id: "title", "These Walls"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "6"}
                p { id: "title", "u"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "7"}
                p { id: "title", "Alright"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "8"}
                p { id: "title", "For Sale? (Interlude)"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "9"}
                p { id: "title", "Momma"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "10"}
                p { id: "title", "Hood Politics"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "11"}
                p { id: "title", "How Much A Dollar Cost"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "12"}
                p { id: "title", "Complexion (A Zulu Love)"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "13"}
                p { id: "title", "The Blacker The Berry"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "14"}
                p { id: "title", "You Ain't Gotta Lie (Momma Said)"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "15"}
                p { id: "title", "i"}
            }

            div {id: "queue-card",
                div {class: "bar",}
                p { id: "position", "16"}
                p { id: "title", "Mortal Man"}
            }
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
                div { id: "controls",
                    img { src: SHUFFLE_SVG}
                    img { src: NEXT_SVG, class: "back"}
                    img { src: PLAY_SVG}
                    img { src: NEXT_SVG}
                    img { src: REPEAT_SVG}
                }
                input { type: "range", min: "0", value: "60", max: "100", class: "slider"}
            }

            div { id: "track-control",
                img { src: QUEUE_SVG}
                img { src: VOLUME_SVG}
                img { src: TIMER_SVG}
            }
        }
    }
}
