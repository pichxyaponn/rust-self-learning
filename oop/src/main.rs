const MAX_HEALTH: u8 = 100;
fn main() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut healer = Healer::new();
    println!("Warrior Health: {}", warrior.health());
    println!("Mage Health: {}", mage.health());
    println!("Healer Health: {}", healer.health());
    println!("---");

    warrior.attack();
    mage.attack();
    println!("---");

    mage.take_damage(30);
    healer.take_damage(15);
    println!("---");

    healer.heal(20);
    println!("---");

    println!("Show Warrior Health: {}", warrior.health());
    println!("Show Mage Health: {}", mage.health());
    println!("Show Healer Health: {}", healer.health());
    println!("---");

    warrior.take_damage(150);
}

trait Weapon {
    fn use_weapon(&self);
}

struct Sword;
impl Weapon for Sword {
    fn use_weapon(&self) {
        println!("Sword attack: Slash!")
    }
}
struct Staff;
impl Weapon for Staff {
    fn use_weapon(&self) {
        println!("Staff attack: Zap!")
    }
}
struct Wand;
impl Weapon for Wand {
    fn use_weapon(&self) {
        println!("Wand heal: Wink!")
    }
}

struct Character {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Character {
    fn new(health: u8, strength: u8, intelligence: u8, weapon: Box<dyn Weapon>) -> Self {
        Self {
            health,
            strength,
            intelligence,
            weapon,
        }
    }

    fn health(&self) -> u8 {
        self.health
    }

    fn health_increase(&mut self, heal: u8) {
        self.health = self
            .health
            .checked_add(heal)
            .unwrap_or(MAX_HEALTH)
            .min(MAX_HEALTH);
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn use_weapon(&self) {
        self.weapon.use_weapon();
    }
}

struct Warrior {
    character: Character,
}

impl Warrior {
    fn new() -> Self {
        Self {
            character: Character::new(MAX_HEALTH, 90, 25, Box::new(Sword)),
        }
    }

    fn health(&self) -> u8 {
        self.character.health()
    }

    fn take_damage(&mut self, damage: u8) {
        self.character.health_decrease(damage);
        println!(
            "Warrior takes {} damage. Current health: {}",
            damage,
            self.health()
        );
    }

    fn attack(&self) {
        println!("Warrior prepares to attack...");
        self.character.use_weapon();
    }
}

struct Mage {
    character: Character,
}

impl Mage {
    fn new() -> Self {
        Self {
            character: Character::new(MAX_HEALTH, 35, 70, Box::new(Staff)),
        }
    }

    fn health(&self) -> u8 {
        self.character.health()
    }

    fn take_damage(&mut self, damage: u8) {
        self.character.health_decrease(damage);
        println!(
            "Mage takes {} damage. Current health: {}",
            damage,
            self.health()
        );
    }

    fn attack(&self) {
        println!("Mage gathers magical energy...");
        self.character.use_weapon();
    }
}

struct Healer {
    character: Character,
}

impl Healer {
    fn new() -> Self {
        Self {
            character: Character::new(MAX_HEALTH, 20, 90, Box::new(Wand)),
        }
    }

    fn health(&self) -> u8 {
        self.character.health()
    }

    fn take_damage(&mut self, damage: u8) {
        self.character.health_decrease(damage);
        println!(
            "Healer takes {} damage. Current health: {}",
            damage,
            self.health()
        );
    }

    fn heal(&mut self, heal: u8) {
        println!("Healer channels healing energy...");
        self.character.health_increase(heal);
        self.character.use_weapon();
        println!(
            "Healer healed for {}. Current health: {}",
            heal,
            self.health()
        );
    }
}
