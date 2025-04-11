fn main() {
    let treasures = ["Gold", "Silver", "Emerald", "Diamond", "Ruby Gem", "Ancient Scroll"];
    let mut energy = 5;
    let target_treasure = "Ruby Gem";

    println!("Gollum has {} energy points to find the {}.", energy, target_treasure);

    for treasure in treasures.iter() {
        if energy <= 0 {
            println!("Game Over: Out of energy!");
            break;
        }

        energy -= 1;
        println!("Remaining energy: {}", energy);

        if treasure == &target_treasure {
            println!("\nGame Win: Gollum found the {}!", target_treasure);
            println!("Energy remaining: {}", energy);
            break;
        }
    }
}
