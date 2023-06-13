#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::items::Statstick;

mod data;
mod items;
mod maps;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut map_list = maps::get_map_list(); 
    let map = map_list.pop().unwrap();
    let map_hook = use_state(cx, || map);

    let combo = items::get_combo_from_csv(); 
    let combo_hook = use_state(cx, || combo);

    cx.render(rsx! {
        section { class: "min-h-screen m-5 bg-[#121212]", 
            h1 { class: "text-neutral-100 lg:text-5xl text-4xl text-center", 
                "Mario Kart 8 Deluxe Randomizer" }
            div { 
                class: "flex flex-row justify-around content-evenly p-10",
                div { 
                    class: "flex flex-col grow-0",
                    Combo {
                        combo: combo_hook,
                    }
                }
            }
            div { class: "flex justify-around",
            button { class: "p-2 mb-5 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300",
            onclick: move |_| { 
                combo_hook.set( items::get_combo_from_csv())
            },
            "New Loadout"
            }
            }
            hr {}
            div { class: "flex flex-row justify-around p-5",
            Map{
                curr_map: &map_hook,
            }
            }
            div{ class: "flex flex-row justify-around ",
            button { class: "p-2 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300",
            onclick: move |_| {
                let newMap = map_list.pop().unwrap();
                map_hook.set(newMap);
            },
            "Next map"
            }
            }
        }
    })
}


#[inline_props]
fn Map<'a>(cx: Scope<'a>, curr_map: &'a str) -> Element<'a> {
    cx.render(rsx! {
        div{ class: "text-neutral-100 text-lg text-center", 
            "{curr_map}"
        }
    })
}

#[inline_props]
fn Combo<'a>(cx: Scope<'a>, combo: &'a Statstick) -> Element<'a> { 
    let combo_string = format!("{}", combo);
    cx.render(rsx! {
        combo_string.lines().map(|i| rsx! {
            div {
                class: "font-mono text-neutral-100 text-base/6 whitespace-pre",
                "{i}" 
            } 
        })
    })
}
