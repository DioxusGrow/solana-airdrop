#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
mod constant;

pub const STYLE: &str = asset!("./assets/tailwind.css");

#[derive(PartialEq, Debug, Clone, Copy)]
enum Validator {
    Local,
    Testnet,
    Devnet,
    Mainnet,
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        head::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut validator = use_signal(|| Validator::Local);
    let mut validator_select = use_signal(|| "local".to_string());

    let url = match validator_select.read().as_str() {
        "local" => {
            // validator.set(Validator::Local);
            "http://127.0.0.1:8899";
        }
        "devnet" => {
            // validator.set(Validator::Devnet);
            "https://api.devnet.solana.com";
        }
        "testnet" => {
            // validator.set(Validator::Testnet);
            "https://api.testnet.solana.com";
        }
        "mainnet-beta" => {
            // validator.set(Validator::Mainnet);
            "https://api.mainnet-beta.solana.com";
        }
        _ => {}
    };

    rsx! {
        label { r#for: "validator", "Choose a validator:" }
        select {
            name: "validator",
            id: "validator",
            value: "{validator_select}",
            oninput: move |ev| { validator_select.set(ev.value()) },
            option { value: "local", "local" }
            option { value: "devnet", "devnet" }
            option { value: "testnet", "testnet" }
            option { value: "mainnet-beta", "mainnet-beta" }
        }
        div { "{url:?}" }
    }
}

// mainnet-beta https://api.mainnet-beta.solana.com
// devnet https://api.devnet.solana.com
// testnet https://api.testnet.solana.com
// local "http://127.0.0.1:8899"
// div {
//     {
//         match (validator_memo.read())().as_str()  {
//             "local" => {
//                 // validator.set(Validator::Local);
//                 url.set("http://127.0.0.1:8899".to_string());
//             },
//             "devnet" => {
//                 // validator.set(Validator::Devnet);
//                 url.set("https://api.devnet.solana.com".to_string());
//             }
//             "testnet" => {
//                 // validator.set(Validator::Testnet);
//                 url.set("https://api.testnet.solana.com".to_string());
//             }
//             "mainnet-beta" => {
//                 // validator.set(Validator::Mainnet);
//                 url.set("https://api.mainnet-beta.solana.com".to_string());
//             }
//             _ => {}
//         }
//     },

//     "{url()}"
// }
