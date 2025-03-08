use macroquad::rand::RandGenerator;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_rand_generator() -> RandGenerator {
    // random number generator
    let rng = RandGenerator::new();
    // this is the seed of the number generator (the current time in milliseconds)
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("yooo, the time is all fucked up 'n shit")
        .as_millis();
    rng.srand(current_time as u64);
    rng
}
