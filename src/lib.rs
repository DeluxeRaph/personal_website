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
    let title = "my_title";
    let about= "my_about";
    let img = "my_img";
    let my_object = MyObject {username: name.to_owned(), favorite_language: "Rust".to_owned()};

    log!(serde_json::to_string_pretty(&my_object).unwrap());
    log!("My name is ", name);
    html! {
       <>
       <head></head>
        <body>
            <h1 class={title}><u>{"Raphael Nembhard Personal Website"}</u></h1>
            <p>
                <a href="https://opensea.io/assets/ethereum/0xd3605059c3ce9facf625fa72d727508b7b7f280f/1785">
                <img class={img}src="img/villagefarmer.JPG" alt="rust image"/>
                </a>
                    <font>{"My PFP is my beloved Pablo #1785"}</font>
            </p>
            <div class={about}>{"I build then I build more"}
                <br />
                {"I like to build blockchains and dapps. My favorite langauges are Rust and soldity"}
                <br />
                {"You can view my projects here on my "}
                <a href="https://github.com/DeluxeRaph">
                {"Github"}</a>
            </div>
        </body>
        <footer>
            <hr />
            <p>{"Author: Raphael"}</p>
            <p><a href="mailto:raphael@digitalinnovation.com">{"raphael@digitalinnovation.com"}</a></p>
        </footer>
       </> 
    }
}