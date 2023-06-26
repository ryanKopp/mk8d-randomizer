#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use crate::items::Statstick;
use fermi::*;

mod data;
mod items;
mod maps;

const NUM_MAPS: usize = 80;
static COUNT: Atom<usize> = |_| 0;
static MAPS: AtomRef<Vec<String>> = |_| maps::get_map_list();

fn main() {
    // launch the web app
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}


fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let combo = items::get_combo_from_csv(); 
    let combo_hook = use_state(cx, || combo);

    cx.render(rsx! {
        section { class: "min-h-screen m-5 bg-[#121212]",
            h1 { class: "text-neutral-100 lg:text-5xl text-4xl text-center", "Mario Kart 8 Deluxe Randomizer" }
            div { class: "flex flex-row justify-around content-evenly p-10",
                div { class: "flex flex-col grow-0", Combo { combo: combo_hook } }
            }
            div { class: "flex justify-around",
                button {
                    class: "p-2 mb-5 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300",
                    onclick: move |_| { 
                        combo_hook.set(items::get_combo_from_csv()) },
                    "New Loadout"
                }
            }
            hr {}
            Map {} 
        }
    })
}


fn Map(cx: Scope) -> Element {
    let map_vec = use_atom_ref(cx, MAPS);

    let count = use_read(cx, COUNT);
    let changeCount = use_set(cx, COUNT);
   
    //Reshuffle maps
    if *count >= NUM_MAPS {
        *map_vec.write() = maps::get_map_list();
        changeCount(0);
        log::info!("Map list refreshed");
    }
/*
    let mut maps = String::new();
    for i in 0..80 {
        maps += map_vec.read().get(i).unwrap();
        maps += "\n";
    }
*/
    cx.render(rsx! {
//div { class: "flex flex-row justify-around text-neutral-100 whitespace-pre", "{maps}"
        
        div { class: "text-neutral-100 text-lg text-center p-5",
            "{map_vec.read().get(*count%(map_vec.read().len())).unwrap()}"
        }
        div { class: "flex flex-row justify-around",
        button {
            class: "p-2 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300",
            onclick: move |_| {
                changeCount(count+1);
                log::info!("Click #{}", count);
            },
            "Next map"
        }
        //}
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
