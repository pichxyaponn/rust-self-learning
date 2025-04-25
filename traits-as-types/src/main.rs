fn main() {
    let gollum_sword = Sword;
    let gollum_bow = Bow;
    let gollum_potion = Potion;

    use_gear(&gollum_sword);
    use_gear(&gollum_bow);
    use_gear(&gollum_potion);
}

trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing Sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Fire arrow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drink potion");
    }
}

fn use_gear(item: &impl Gear) {
    item.use_gear();
}
