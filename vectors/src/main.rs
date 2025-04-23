fn main() {
    let mut inventory = vec!["Gold Coins", "Books", "Banana", "Bottle"];
    inventory.push("Sword");
    inventory.push("Shield");
    let removed_item = inventory.remove(0);
    println!("Removed '{}' from position 0", removed_item);

    // inventory.clear();
    if inventory.is_empty() {
        println!("Inventory is empty!");
        return;
    }

    println!("Inventory contents: {:?}", inventory);

    inventory.sort();
    println!("Sorted inventory: {:?}", inventory);

    println!("Items count: {:?}", inventory.len());
    println!("Inventory capacity: {:?}", inventory.capacity());
}
