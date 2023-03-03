#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod items;
mod maps;

use eframe::egui;
use egui::{RichText, FontId};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mario Kart 8 Deluxe Randomizer",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "Mario Kart 8 Deluxe Randomizer", // hardcode it
            web_options,
            Box::new(|_cc| Box::new(MyApp::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}

struct MyApp {
  num_maps: u32,

  combo1: String,
  combo2: String,
  combo3: String,
  combo4: String,

  map_list: String,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      num_maps: 4,

      combo1 : format!("\n{}", items::get_combo_from_csv()),
      combo2 : format!("\n{}", items::get_combo_from_csv()),
      combo3 : format!("\n{}", items::get_combo_from_csv()),
      combo4 : format!("\n{}", items::get_combo_from_csv()),

      map_list: maps::get_maps(4),
    }
  }
}
impl eframe::App for MyApp{
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::new([true,true]).show(ui, |ui|{
          ui.heading(RichText::new("Mario Kart 8 Deluxe Randomizer").font(FontId::proportional(36.0)));
        ui.horizontal(|ui| {
          ui.vertical(|ui| {
            egui::Grid::new("combo grid")
              .num_columns(3)
              .min_col_width(260.0)
              .show(ui, |ui| {
                ui.vertical(|ui|{
                  ui.label(RichText::new("Player 1 Combo").font(FontId::proportional(18.0)));
                  ui.label(RichText::new(format!("{}", self.combo1)).font(FontId::monospace(14.0)));
                  ui.separator();
                });
                ui.vertical(|ui|{
                  ui.label(RichText::new("Player 2 Combo").font(FontId::proportional(18.0)));
                  ui.label(RichText::new(format!("{}", self.combo2)).font(FontId::monospace(14.0)));
                  ui.separator();
                });
                ui.end_row();

                ui.vertical(|ui|{
                  ui.label(RichText::new("Player 3 Combo").font(FontId::proportional(18.0)));
                  ui.label(RichText::new(format!("{}", self.combo3)).font(FontId::monospace(14.0)));
                  ui.separator();
                });
                ui.vertical(|ui|{
                  ui.label(RichText::new("Player 4 Combo").font(FontId::proportional(18.0)));
                  ui.label(RichText::new(format!("{}", self.combo4)).font(FontId::monospace(14.0)));
                  ui.separator();
                });
                ui.end_row();
              });
            if ui.button("Generate Setups").clicked() {
              self.combo1 = format!("\n{}", items::get_combo_from_csv());
              self.combo2 = format!("\n{}", items::get_combo_from_csv());
              self.combo3 = format!("\n{}", items::get_combo_from_csv());
              self.combo4 = format!("\n{}", items::get_combo_from_csv());
            }
            ui.add_space(10.0);
          });
          ui.separator();

          ui.vertical(|ui| {
            ui.add(egui::Label::new(
                RichText::new("Map Randomizer")
                .font(FontId::proportional(25.0)))
              .wrap(false));
            ui.horizontal(|ui| {
              ui.label(RichText::new("Number of Maps").font(FontId::proportional(18.0)));
              ui.add(egui::Slider::new(&mut self.num_maps, 4..=48));
            });
            if ui.button("Randomize Maps").clicked() {
              self.map_list = maps::get_maps(self.num_maps);
            }
            ui.separator();
            ui.label(RichText::new(format!("{}", self.map_list)).font(FontId::proportional(14.0)));
          });
        });
      });
    });
  }
}
