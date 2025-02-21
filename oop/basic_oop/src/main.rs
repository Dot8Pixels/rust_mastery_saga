trait Weapon {
    fn attack(&self);
}

struct Sword;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack");
    }
}

struct Staff;

impl Weapon for Staff {
    fn attack(&self) {
        println!("Staff attack");
    }
}

struct Warrior {
    health: u8,
    weapon: Box<dyn Weapon>,
}

impl Warrior {
    fn new() -> Self {
        Self {
            health: 100,
            weapon: Box::new(Sword),
        }
    }

    fn get_health(&self) -> u8 {
        self.health
    }

    fn get_weapon(&self) -> &dyn Weapon {
        &*self.weapon
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }

        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

struct Mage {
    health: u8,
    weapon: Box<dyn Weapon>,
}

impl Mage {
    fn new() -> Self {
        Self {
            health: 100,
            weapon: Box::new(Staff),
        }
    }

    fn get_health(&self) -> u8 {
        self.health
    }

    fn get_weapon(&self) -> &dyn Weapon {
        &*self.weapon
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }

        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

struct Healer {
    health: u8,
    weapon: Box<dyn Weapon>,
}

impl Healer {
    fn new() -> Self {
        Self {
            health: 100,
            weapon: Box::new(Staff),
        }
    }

    fn get_health(&self) -> u8 {
        self.health
    }

    fn get_weapon(&self) -> &dyn Weapon {
        &*self.weapon
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }

        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

fn special_attack(weapon: &dyn Weapon) {
    weapon.attack();
}

fn main() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut healer = Healer::new();

    warrior.health_decrease(10);
    mage.health_decrease(10);
    healer.health_decrease(10);

    println!("Warrior health: {}", warrior.get_health());
    println!("Mage health: {}", mage.get_health());
    println!("Healer health: {}", healer.get_health());

    warrior.health_increase(10);
    mage.health_increase(10);
    healer.health_increase(10);

    println!("Warrior health: {}", warrior.get_health());
    println!("Mage health: {}", mage.get_health());
    println!("Healer health: {}", healer.get_health());

    special_attack(warrior.get_weapon());
    special_attack(mage.get_weapon());
    special_attack(healer.get_weapon());
}
