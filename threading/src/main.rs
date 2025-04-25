use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let gollum_gold = Arc::new(Mutex::new(10));

    let loot_1 = thread::spawn({
        let gollum_gold_artifact = Arc::clone(&gollum_gold);
        move || {
            let mut gold = gollum_gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_2 = thread::spawn({
        let gollum_gold_artifact = Arc::clone(&gollum_gold);
        move || {
            let mut gold = gollum_gold_artifact.lock().unwrap();
            *gold += 200;
        }
    });

    let loot_3 = thread::spawn({
        let gollum_gold_artifact = Arc::clone(&gollum_gold);
        move || {
            let mut gold = gollum_gold_artifact.lock().unwrap();
            *gold += 300;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    loot_3.join().unwrap();

    println!("Gollum's gold: {}", gollum_gold.lock().unwrap());
}
