fn main() {
    let gold = Inventory { item: 100 };
    gold.display();

    let rusty_sword = Inventory { item: "Rusty Sword" };
    rusty_sword.display();
}

struct Inventory<T> {
    item: T
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T> where
    T: std::fmt::Debug {
    fn display(&self) {
        println!("{:?}", self.item);
    }
}
