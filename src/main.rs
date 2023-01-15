use core::fmt;
use std::error::Error;
//use std::process;
use std::fs::File;
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


fn read_csv(fname: &str) -> Result<Statstick, Box<dyn Error>> {

    let file = File::open(fname)?;
    let mut rdr = csv::Reader::from_reader(file);

    // Pick a random record to deserialize into struct
    let record: Statstick = rdr.deserialize()
        .choose(&mut rand::thread_rng())
        .unwrap().unwrap();
    Ok(record)
}

fn main() {
    let driver = read_csv("csv/DRIVERS.csv").unwrap();
    let kart = read_csv("csv/VEHICLES.csv").unwrap();
    let tire = read_csv("csv/TIRES.csv").unwrap();
    let glider = read_csv("csv/GLIDERS.csv").unwrap();

    let combo = driver + kart + tire + glider;
    println!("{:?}", combo);

}
