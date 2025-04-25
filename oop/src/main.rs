fn main() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut healer = Healer::new();

    warrior.health_decrease(20);
    mage.health_decrease(20);
    healer.health_decrease(20);

    warrior.attack();
    mage.attack();

    healer.health_increase(5);

    println!("Show Warrior Health: {}", warrior.health());
    println!("Show Mage Health: {}", mage.health());
    println!("Show Healer Health: {}", healer.health());
}

trait Weapon {
    fn attack(&self);
}

struct Sword;
impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack: Slash!")
    }
}
struct Staff;
impl Weapon for Staff {
    fn attack(&self) {
        println!("Staff attack: Zap!")
    }
}

trait Character {
    fn job(&self) -> &'static str;
    fn health(&self) -> u8;
    fn health_increase(&mut self, heal: u8);
    fn health_decrease(&mut self, damage: u8);
    fn attack(&self);
}

struct Warrior {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Warrior {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 90,
            intelligence: 25,
            weapon: Box::new(Sword),
        }
    }
}

impl Character for Warrior {
    fn job(&self) -> &'static str {
        "Warrior"
    }

    fn health(&self) -> u8 {
        self.health
    }

    fn health_increase(&mut self, heal: u8) {
        self.health = self.health.checked_add(heal).unwrap_or(100).min(100);
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn attack(&self) {
        self.weapon.attack();
    }
}

struct Mage {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Mage {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 35,
            intelligence: 70,
            weapon: Box::new(Staff),
        }
    }
}

impl Character for Mage {
    fn job(&self) -> &'static str {
        "Mage"
    }

    fn health(&self) -> u8 {
        self.health
    }

    fn health_increase(&mut self, heal: u8) {
        self.health = self.health.checked_add(heal).unwrap_or(100).min(100);
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn attack(&self) {
        self.weapon.attack();
    }
}

struct Healer {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Healer {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 20,
            intelligence: 90,
            weapon: Box::new(Staff),
        }
    }
}

impl Character for Healer {
    fn job(&self) -> &'static str {
        "Healer"
    }

    fn health(&self) -> u8 {
        self.health
    }

    fn health_increase(&mut self, heal: u8) {
        self.health = self.health.checked_add(heal).unwrap_or(100).min(100);
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn attack(&self) {
        self.weapon.attack();
    }
}
