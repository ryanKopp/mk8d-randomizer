#![allow(non_snake_case)]
use crate::items::Statstick;
use dioxus::prelude::*;
use fermi::*;

mod data;
mod items;
mod maps;

const NUM_MAPS: usize = 96;
static COUNT: Atom<usize> = |_| 0;
static MAPS: AtomRef<Vec<String>> = |_| maps::get_map_list();

fn main() {
    // launch the web app
    #[cfg(debug_assertions)]
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
            div { class: "flex flex-row justify-center",
                div { class: "flex flex-col justify-center grow-0 pt-5", Combo { combo: combo_hook } }
            }
            div { class: "flex justify-around",
                button {
                    class: "p-2 m-2 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300 touch-manipulation",
                    onclick: move |_| { combo_hook.set(items::get_combo_from_csv()) },
                    "New Loadout"
                }
            }
            hr {}
            div { class: "flex flex-col justify-around grow-0", Map {} }
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
        #[cfg(debug_assertions)]
        log::info!("Map list refreshed");
    }

    let binding = map_vec.read();
    let curr_map = binding.get(*count % (map_vec.read().len())).unwrap();

    cx.render(rsx! {
        div { class: "flex flex-row justify-around pt-2", img { src: "assets/maps/{curr_map}.webp" } }
        div { class: "text-neutral-100 text-lg text-center", "{curr_map}" }
        div { class: "flex flex-row justify-around",
            button {
                class: "p-2 mt-3 rounded-lg bg-neutral-800 text-neutral-100 border border-solid border-neutral-300 touch-manipulation",
                onclick: move |_| {
                    changeCount(count + 1);
                    #[cfg(debug_assertions)]
                    {
                        log::info!("Click #{}", count + 1);
                        log::info!("First map: {}", map_vec.read().get(0).unwrap());
                        log::info!("Last map: {}", map_vec.read().get(79).unwrap());
                    }
                },
                "Next map"
            }
        }
    })
}

#[inline_props]
fn Combo<'a>(cx: Scope<'a>, combo: &'a Statstick) -> Element<'a> {
    let char_name = combo.get_character().unwrap_or("error");
    let kart_name = combo.get_kart().unwrap_or("error");
    let tire_name = combo.get_tire().unwrap_or("error");
    let glider_name = combo.get_glider().unwrap_or("error");

    cx.render(rsx! {
        div { class: "flex flex-row justify-between text-neutral-100 pb-3",
            div { class: "flex flex-col pr-7",
                img { src: "assets/characters/{char_name}.webp" }
                div { class: "text-center text-neutral-100 text-lg", "{char_name}" }
            }
            Stats_Display { combo: combo.clone() }
        }

        div { class: "flex flex-row justify-center flex-grow-0 gap-4",
            img { src: "assets/karts/{kart_name}.webp" }
            img { src: "assets/tires/{tire_name}.webp" }
            img { src: "assets/gliders/{glider_name}.webp" }
        }

        div { class: "flex flex-row justify-center flex-grow-0 gap-4 min-h-[40px]",
            div { class: "w-[100px] text-neutral-100 text-center whitespace-normal text-base/5",
                "{kart_name}"
            }
            div { class: "w-[100px] text-neutral-100 text-center whitespace-normal text-base/5",
                "{tire_name}"
            }
            div { class: "w-[100px] text-neutral-100 text-center whitespace-normal text-base/5",
                "{glider_name}"
            }
        }
    })
}

#[inline_props]
fn Stats_Display<'a>(cx: Scope<'a>, combo: &'a Statstick) -> Element<'a> {
    cx.render(rsx! {
        div { class: "flex flex-col text-sm",
            "Speed"
            img { src: "assets/statBars/{combo.speed}.png" }
            "Acceleration"
            img { src: "assets/statBars/{combo.acceleration}.png" }
            "Weight"
            img { src: "assets/statBars/{combo.weight}.png" }
            "Handling"
            img { src: "assets/statBars/{combo.handling}.png" }
            "Traction"
            img { src: "assets/statBars/{combo.traction}.png" }
        }
    })
}
