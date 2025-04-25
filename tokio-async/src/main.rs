use tokio;

#[tokio::main]
async fn main() {
    let gather_herbs = tokio::spawn(gather_herbs());
    let collect_gold_coins = tokio::spawn(collect_gold_coins());
    let fight_monster = tokio::spawn(fight_monster());

    let _ = tokio::join!(gather_herbs, collect_gold_coins, fight_monster);
}

async fn gather_herbs() {
    println!("Gollum is gathering herbs...");
}

async fn collect_gold_coins() {
    println!("Gollum is collecting gold coins...");
}

async fn fight_monster() {
    println!("Gollum is fighting the monster!");
}
