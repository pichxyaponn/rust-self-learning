use std::collections::HashMap;

fn main() {
    let mut treasures: HashMap<&str, i32> = HashMap::new();
    treasures.insert("Gold Coins", 10);
    treasures.insert("Diamond", 1);

    if let Some(diamond) = treasures.get_mut("Diamond") {
        *diamond += 1;
    }

    println!("Treasures: {:?}", treasures);
}
