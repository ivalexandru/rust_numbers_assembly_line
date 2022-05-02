// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]


// helper fn, does not need to be publ
fn success_rate(speed: u8) -> f64{
    match speed {
        1..=4 => 1.0, // 100% success rate
        5..=8 => 0.9,
        9..=10 => 0.77, // 77% success rate 
        _ => 0.0,  
    }
}

const MIN_CARS_PER_HOUR: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    f64::from(speed) * MIN_CARS_PER_HOUR * success_rate(speed)
}


pub fn working_items_per_minute(speed: u8) -> u32 {
   production_rate_per_hour(speed) as u32 / 60
}


