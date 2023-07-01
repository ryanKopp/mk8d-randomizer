use crate::data;

use core::fmt;
use rand::seq::IteratorRandom;
use serde::Deserialize;
use std::ops;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Statstick {
    pub name: String,
    speed: f32,
    acceleration: f32,
    weight: f32,
    handling: f32,
    traction: f32,
}

impl Statstick {
    fn new() -> Statstick {
        return Statstick {
            name: "".to_string(),
            speed: 0.0,
            acceleration: 0.0,
            weight: 0.0,
            handling: 0.0,
            traction: 0.0,
        };
    }
    #[allow(dead_code)]
    pub fn test_item() -> Statstick {
        return Statstick {
            name: "test".to_string(),
            speed: 6.0,
            acceleration: 6.0,
            weight: 6.0,
            handling: 6.0,
            traction: 6.0,
        };
    }
}

impl ops::Add<Statstick> for Statstick {
    type Output = Statstick;

    fn add(mut self, _rhs: Statstick) -> Statstick {
        if self.name != "" {
            self.name = format!("{}\n{}", self.name, _rhs.name);
        } else {
            self.name = _rhs.name;
        }
        self.speed += _rhs.speed;
        self.acceleration += _rhs.acceleration;
        self.weight += _rhs.weight;
        self.handling += _rhs.handling;
        self.traction += _rhs.traction;

        self
    }
}

impl fmt::Display for Statstick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n\n", self.name)?;
        write!(f, "Speed: {}\n{}\n", self.speed, generate_bar(self.speed))?;
        write!(
            f,
            "Acceleration: {}\n{}\n",
            self.acceleration,
            generate_bar(self.acceleration)
        )?;
        write!(
            f,
            "Weight: {}\n{}\n",
            self.weight,
            generate_bar(self.weight)
        )?;
        write!(
            f,
            "Handling: {}\n{}\n",
            self.handling,
            generate_bar(self.handling)
        )?;
        write!(
            f,
            "Traction: {}\n{}\n",
            self.traction,
            generate_bar(self.traction)
        )
    }
}

fn generate_bar(num: f32) -> String {
    let bar_width = 30;
    let num_pounds: u8 = (num * 4.0) as u8;
    let mut bar = String::from("[");

    for i in 1..bar_width {
        if i % 5 == 0 {
            bar += "|";
        } else if i <= num_pounds + i / 5 {
            bar += "*";
        } else {
            bar += " ";
        }
    }
    bar += "]";

    bar
}

fn pick_item_from_csv(csv: &str) -> Statstick {
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());

    // Pick a random record to deserialize into struct
    // data is hardcorded so unwrap is ok
    let record: Statstick = rdr
        .deserialize()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .unwrap();

    return record;
}

pub fn get_combo_from_csv() -> Statstick {
    let assets = vec![
        data::DRIVER_DATA,
        data::VEHICLE_DATA,
        data::TIRE_DATA,
        data::GLIDER_DATA,
    ];

    let mut combo = Statstick::new();

    for part_list in assets {
        combo = combo + pick_item_from_csv(part_list);
    }

    return combo;
}
