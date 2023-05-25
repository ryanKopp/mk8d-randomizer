#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

//mod data;
//mod items;
//mod maps;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "flex flex-wrap -mx-4 mb-14 items-center",
        div { class: "w-full xl:w-2/3 px-4 mb-4 xl:mb-0",
        p {
            "hello there"
        }
        }
        p { "hello from outside" }
        p { "hello from outside" }
        p { "hello from outside" }
        p { "hello from outside" }
        p { "hello from outside" }
        p { "hello from outside" }
        } 
    })
}
