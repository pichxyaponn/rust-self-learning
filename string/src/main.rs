fn main() {
    let eng_map = String::from("England map");

    // Read only
    let borrowed_map = eng_map.as_str();

    // Read/Write
    let mut thai_map = borrowed_map.to_string();
    thai_map.push_str(" to Thailand map");

    println!("{}", thai_map);

}
