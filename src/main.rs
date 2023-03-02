mod data;
mod items;
mod maps;

use std::env;

fn main() {

    let mut maps = maps::get_map_list();
    maps::shuffle_maps(&mut maps);
   
    let mut args = env::args();
    args.next();
    
    let num_maps = match args.next(){
        Some(num) => num.parse::<i32>().unwrap(),
        None => {println!("Outputting zero maps"); 0},
    };

    for map in maps.into_iter().take(num_maps.try_into().unwrap()){
        println!("{}", map);
    }

    let combo = items::get_combo_from_csv();
    println!("{}", combo);
}
