#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

mod data;
mod items;
mod maps;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
   // let mut map_list = maps::get_map_list(); 

    cx.render(rsx! {
        section { class: "min-h-screen m-5 bg-[#121212]", 
            h1 { class: "text-neutral-100 lg:text-5xl text-4xl text-center", 
                "Mario Kart 8 Deluxe Randomizer" }
        div { 
            class: "flex flex-row justify-around content-evenly p-10",
            div { 
                class: "flex flex-col grow-0",
                Combo {}
            }
        }
        hr {}
        div {
            Map{}
        }
        }
    })
}

fn Map(cx: Scope) -> Element {
    let mut map_list = maps::get_map_list();
    //let mut map = map_iter.next().unwrap();
    let mapHook = use_ref(cx, || map_list.pop().unwrap());
    cx.render(rsx! {
        div{ class: "text-neutral-100 text-lg text-center", 
            "{mapHook.read()}"
        }
    })
}

fn Combo(cx: Scope) -> Element { 
    //let test = items::get_combo_from_csv(); 
    let test = items::Statstick::test_item();
    let combo = format!("{}", test);
    cx.render(rsx! {
        combo.lines().map(|i| rsx! {
            div {
                class: "font-mono text-neutral-100 text-base/6 whitespace-pre",
                "{i}" 
            } 
        })
    })
}
