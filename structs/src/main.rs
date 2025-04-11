fn main() {
    let mut gollum = Gollum {
        name: String::from("Gollum"),
        health: 100
    };

    gollum.take_damage(80);
    gollum.status();
    gollum.healing(80);
    gollum.healing(20);
    gollum.status();
    gollum.take_damage(100);
    gollum.status();
    gollum.take_damage(80);
}

struct Gollum {
    name: String,
    health: u8
}

impl Gollum {
    fn healing(&mut self, heal: u8) {
        if self.health >= 100 {
            println!("{} is already at full health!", self.name);
            return;
        }

        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health = self.health.saturating_add(heal);
        println!("{} is healing {}HP.", self.name, heal);
    }

    fn take_damage(&mut self, damage: u8) {
        if self.health <= 0 {
            println!("{} is already defeated!", self.name);
            return;
        }

        self.health = self.health.saturating_sub(damage);
        println!("{} was damaged {}HP by troll!", self.name, damage);
    }

    fn status(&mut self) {
        println!("{} health: {}HP", self.name, self.health);
    }
}
