use serde::{Serialize, Deserialize};
use yew::prelude::*;
use gloo::console::log;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "raph";
    let my_object = MyObject {username: name.to_owned(), favorite_language: "Rust".to_owned()};

    log!(serde_json::to_string_pretty(&my_object).unwrap());
    log!("My name is ", name);
    html! {
        <h1>{"Hello World !!!"}</h1>
    }
}