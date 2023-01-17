mod data;

use core::fmt;
use std::error::Error;
//use std::process;
//use std::fs::File;
use serde::Deserialize;
use rand::seq::IteratorRandom;
use std::ops;

#[derive(Debug, Deserialize)]
struct Statstick {
    name: String,
    speed : f32,
    acceleration: f32,
    weight : f32,
    traction: f32, 
    handling: f32,
}

impl ops::Add<Statstick> for Statstick {
    type Output = Statstick;

    fn add(mut self, _rhs: Statstick) -> Statstick {
        self.name = format!("{}, {}", self.name, _rhs.name); 
        self.speed += _rhs.speed;
        self.acceleration += _rhs.acceleration;
        self.weight += _rhs.weight;
        self.traction += _rhs.traction;
        self.handling += _rhs.handling;
        
        self
    }
}

impl fmt::Display for Statstick {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        write!(f, "Speed: {}\n{}\n", self.speed, generate_bar(self.speed))?;
        write!(f, "Acceleration: {}\n{}\n", self.acceleration, generate_bar(self.acceleration))?;
        write!(f, "Weight: {}\n{}\n", self.weight, generate_bar(self.weight))?;
        write!(f, "Traction: {}\n{}\n", self.traction, generate_bar(self.traction))?;
        write!(f, "Handling: {}\n{}\n", self.handling, generate_bar(self.handling))
    }
}

fn generate_bar(num: f32) -> String {
    let bar_width = 29;
    let num_pounds: u8 = (num * 4.0) as u8;
    let mut bar = String::from("[");

    for i in 1..bar_width{
        if i % 5 == 0 {
            bar += "|";
        }else if i <= num_pounds + i/5 {
            bar += "*";
        }
        else{
            bar += " ";
        }
    }
    bar += "]";

    bar
}


fn pick_item_from_csv(csv: &str) -> Result<Statstick, Box<dyn Error>> {

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());

    // Pick a random record to deserialize into struct
    let record: Statstick = rdr.deserialize()
        .choose(&mut rand::thread_rng())
        .unwrap().unwrap();
    Ok(record)
}

fn main() {
    let driver = pick_item_from_csv(data::DRIVER_DATA)
        .expect("error getting driver");
    let vehicle = pick_item_from_csv(data::VEHICLE_DATA)
        .expect("error getting kart");
    let tire = pick_item_from_csv(data::TIRE_DATA)
        .expect("error getting tire");
    let glider = pick_item_from_csv(data::GLIDER_DATA)
        .expect("error getting glider");

    let combo = driver + vehicle + tire + glider;
    println!("{}", combo);

}
