use rand::Rng;
use rand::thread_rng;
use std::fmt::Debug;

trait CanHit {
    fn hit(&self, target: &mut dyn CanTakeHit);
}

trait CanTakeHit: Debug {
    fn take_hit(&mut self, damage: i32);
}

#[derive(Debug)]
struct Goblin {
    health: i32,
    damage: i32,
}

impl Goblin {
    fn new() -> Self {
        Self { health: 10, damage: 2 }
    }
}

impl CanHit for Goblin {
    fn hit(&self, target: &mut dyn CanTakeHit) {
        if self.health == 0 {
            println!("Goblin is already game over and cannot hit anything");
            return;
        }
        println!("Goblin hits");
        target.take_hit(self.damage);
    }
}

impl CanTakeHit for Goblin {
    fn take_hit(&mut self, damage: i32) {
        if self.health == 0 {
            println!("Goblin is already game over and cannot take any more hits");
            return;
        }
        self.health = std::cmp::max(0, self.health - damage);
        println!("Goblin got hit, health is now {}", self.health);
        if self.health == 0 {
            println!("Goblin game over");
        }
    }
}

#[derive(Debug)]
struct Orc {
    health: i32,
    damage: i32,
}

impl Orc {
    fn new() -> Self {
        Self { health: 6, damage: 3 }
    }
}

impl CanHit for Orc {
    fn hit(&self, target: &mut dyn CanTakeHit) {
        if self.health == 0 {
            println!("Orc is game over and cannot hit anything");
            return;
        }
        println!("Orc hits");
        target.take_hit(self.damage);
    }
}

impl CanTakeHit for Orc {
    fn take_hit(&mut self, damage: i32) {
        if self.health == 0 {
            println!("Orc is already game over and cannot take any more hits");
        }
        self.health = std::cmp::max(0, self.health - damage);
        println!("Orc got hit, health is now {}", self.health);
        if self.health == 0 {
            println!("Orc game over");
        }
    }
}

#[derive(Debug)]
struct Building {
    health: i32,
}

impl Building {
    fn new() -> Self {
        Self { health: 15 }
    }
}

impl CanTakeHit for Building {
    fn take_hit(&mut self, damage: i32) {
        if self.health == 0 {
            println!("Building has already been destroyed");
        }
        self.health = std::cmp::max(0, self.health - damage);
        println!("Building got hit, health is now {}", self.health);
        if self.health == 0 {
            println!("Building destroyed");
        }
    }
}

fn main() {
    let mut goblin = Goblin::new();
    let mut orc = Orc::new();
    let mut building = Building::new();

    println!("Fight commences");

    for i in 0..10 {
        println!("\nRound {}", i);
        let mut rng = thread_rng();
        let attacker = rng.gen_range(0..2);
        let defender = rng.gen_range(0..2);
        match attacker {
            0 => {
                match defender {
                    0 => goblin.hit(&mut orc),
                    1 => goblin.hit(&mut building),
                    _ => (),
                }
            }
            1 => {
                match defender {
                    0 => orc.hit(&mut goblin),
                    1 => orc.hit(&mut building),
                    _ => (),
                }
            }
            _ => (),
        }
    }

    println!("\nFight ends");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goblin_hit_orc() {
        let goblin = Goblin::new();
        let mut orc = Orc::new();
        goblin.hit(&mut orc);
        assert_eq!(orc.health, 4);
    }

    #[test]
    fn test_orc_hit_goblin() {
        let orc = Orc::new();
        let mut goblin = Goblin::new();
        orc.hit(&mut goblin);
        assert_eq!(goblin.health, 7);
    }

    #[test]
    fn test_goblin_hit_building() {
        let goblin = Goblin::new();
        let mut building = Building::new();
        goblin.hit(&mut building);
        assert_eq!(building.health, 13);
    }

    #[test]
    fn test_health_never_drops_below_zero() {
        let orc = Orc::new();
        let mut goblin = Goblin::new();
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        assert_eq!(goblin.health, 0);
    }

    #[test]
    fn test_dead_goblin_cannot_damage_building() {
        let orc = Orc::new();
        let mut goblin = Goblin::new();
        let mut building = Building::new();
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        orc.hit(&mut goblin);
        assert_eq!(goblin.health, 0);
        goblin.hit(&mut building);
        assert_eq!(building.health, 15);
    }
}
