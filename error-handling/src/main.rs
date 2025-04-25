fn main() {
    let chest_result = match open_chest(false) {
        Some(item) => item,
        None => "Empty".to_string(),
    };
    println!("The chest contains {}", chest_result);

    let door_result = match open_door(true) {
        Ok(result) => result,
        Err(error) => panic!("{}", error),
    };
    println!("The door is {}", door_result);
}

fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("Coins".to_string())
    }
}

fn open_door(is_locked: bool) -> Result<String, String> {
    if is_locked {
        Err("Locked".to_string())
    } else {
        Ok("Open".to_string())
    }
}
