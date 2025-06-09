# mk8d-randomizer

This project came about because there is no random loadout generation built into MK8D, and me and my friends too often play with the exact same setups. It's also a good excuse to learn new things. 

Designed with mobile in mind.
You can use the app in the browser [here](https://ryankopp.github.io/mk8d-randomizer/).

Built using [dioxus](https://github.com/DioxusLabs/dioxus).

Build locally to test with `dioxus serve`.

You will need to run `rustup target add wasm32-unknown-unknown`.
To install dioxus-cli: `cargo install dioxus-cli --version 0.3.2 --locked`

## Features I may add in the future:

- Blacklists for map randomization
- Item randomization
