trait CanHit {
    fn hit(&self, target: &mut dyn CanTakeHit);
}

trait CanTakeHit {
    fn take_hit(&mut self, damage: i32);
}

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
        target.take_hit(self.damage);
    }
}

impl CanTakeHit for Goblin {
    fn take_hit(&mut self, damage: i32) {
        self.health -= damage;
    }
}

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
        target.take_hit(self.damage);
    }
}

impl CanTakeHit for Orc {
    fn take_hit(&mut self, damage: i32) {
        self.health -= damage;
    }
}

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
        self.health -= damage;
    }
}

fn main() {
    println!("Fight commences");
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
}
