use log_time::log_time;

macro_rules! magic_spelling {
    (fire) => {
        println!("FIRE!");
    };
    (water) => {
        println!("WATER!");
    };
}

#[log_time]
fn main() {
    magic_spelling!(fire);
    magic_spelling!(water);
}
