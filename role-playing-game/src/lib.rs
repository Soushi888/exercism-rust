pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(health: u32, mana: Option<u32>, level: u32) -> Self {
        Self { health, mana, level }
    }

    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 { return None; }

        if self.level < 10 {
            Some(Player::new(100, None, self.level))
        } else {
            Some(Player::new(100, Some(100), self.level))
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            match mana_cost {
                cost if cost > self.health => self.health = 0,
                _ => self.health -= mana_cost,
            };

            return 0;
        }

        if mana_cost > self.mana.unwrap() { return 0; }

        self.mana = Some(self.mana.unwrap() - mana_cost);
        mana_cost * 2
    }
}
