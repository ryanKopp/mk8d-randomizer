#![allow(non_snake_case)]
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
        section { class: "min-h-full m-5 bg-[#121212]",
            h1 { class: "text-neutral-100 lg:text-5xl text-4xl text-center", "Mario Kart 8 Deluxe Randomizer" }
            div { class: "flex flex-row justify-around pt-5",
                div { class: "flex flex-col grow-0", Combo { combo: combo_hook } }
            }
            div { class: "flex justify-around",
            button {
                class: "p-2 m-2 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300 touch-manipulation",
                onclick: move |_| { 
                    combo_hook.set(items::get_combo_from_csv()) },
                    "New Loadout"
            }
            }
            hr {}
            div {class:"flex flex-col justify-around",
                Map {} 
            }
        }
    })
}

fn Map(cx: Scope) -> Element {
    let map_vec = use_atom_ref(cx, MAPS);

    let count = use_read(cx, COUNT);
    let changeCount = use_set(cx, COUNT);
    
    let binding = map_vec.read();
    let curr_map = binding.get(*count%(map_vec.read().len())).unwrap();
    let map_path = "maps/".to_owned() + curr_map + ".webp";
   
    //Reshuffle maps
    if *count >= NUM_MAPS {
        *map_vec.write() = maps::get_map_list();
        changeCount(0);
        log::info!("Map list refreshed");
    }
    cx.render(rsx! {
        div { class: "flex flex-row justify-around pt-2", 
            img {
                src: "{map_path}",
            }
        }
        div { class: "text-neutral-100 text-lg text-center",
            "{map_vec.read().get(*count%(map_vec.read().len())).unwrap()}"
        }
        div { class: "flex flex-row justify-around",
            button {
                class: "p-2 mt-3 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300 touch-manipulation",
                onclick: move |_| {
                    changeCount(count+1);
                    log::info!("Click #{}", count);
                },
                "Next map"
            }
        }
    })
}

#[inline_props]
fn Combo<'a>(cx: Scope<'a>, combo: &'a Statstick) -> Element<'a> { 
    let combo_string = format!("{}", combo);
    cx.render(rsx! {
        combo_string.lines().map(|i| rsx! {
            div {
            class: "font-mono text-neutral-100 text-base/5 md:text-base/6 whitespace-pre",
                "{i}" 
            } 
        }) 
    })
}
