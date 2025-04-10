fn main() {
    let map1 = "Dragon's Lair Map";
    let map2 = "Secret Cave Map";

    let chosen_map = shortest_map(map1, map2);
    println!("Chosen map: {}", chosen_map);
}

fn shortest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() < map2.len() {
        map1
    } else {
        map2
    }
}