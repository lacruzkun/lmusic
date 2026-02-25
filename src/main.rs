use dioxus::prelude::*;
use crate::home::Home;

mod home;
mod setting;
mod playback;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(dioxus::desktop::Config::default().with_menu(None))
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{ href: MAIN_CSS }
        body {
            Home {}
        }
    }
}
