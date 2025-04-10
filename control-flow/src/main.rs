fn main() {
    const WEATHER_SUNNY: &str = "sunny";
    const WEATHER_RAINY: &str = "rainy";
    const WEATHER_STORMY: &str = "stormy";
    let weather = WEATHER_RAINY;
    if weather == WEATHER_SUNNY {
        println!("Gollum will cross the river by swimming!");
    } else if weather == WEATHER_RAINY {
        println!("Gollum will build a bridge to stay dry!");
    } else if weather == WEATHER_STORMY {
        println!("Gollum will wait for better weather!");
    } else {
        println!("Gollum is confused about the weather!");
    }
    
    const ENEMY_GOBLIN: &str = "goblin";
    const ENEMY_TROLL: &str = "troll";
    const ENEMY_DRAGON: &str = "dragon";
    let enemy = ENEMY_TROLL;
    match enemy {
        ENEMY_GOBLIN => println!("Gollum uses his rusty sword to attack!"),
        ENEMY_TROLL => println!("Gollum sets a trap!"),
        ENEMY_DRAGON => println!("Gollum runs for cover!"),
        _ => println!("Gollum is confused by the enemy!")
    };  

    let mut wood = 0;
    const WOOD_NEEDED: i32 = 10;
    loop {
        wood += 1;
        println!("Gollum gathered {} pieces of wood!", wood);

        if wood >= WOOD_NEEDED {
            println!("Gollum finished the boat!");
            break;
        }
        
        // ป้องกัน Infinite loops
        if wood > 100 {
            println!("Something went wrong with wood gathering!");
            break;
        }
    }
}
