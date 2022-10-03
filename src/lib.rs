// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::ptr::null;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            h if h <= 0 => Some(self.revived_player()),
            _ => None
        }
    }

    fn revived_player(&self) -> Player {
        Player {
            health: 100,
            mana: self.derive_mana(),
            level: self.level,
        }
    }

    fn derive_mana(&self) -> Option<u32> {
        match self.level {
            level if level >= 10 => Some(100),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let damage = match self.mana {
            Some(mana) if mana < mana_cost => 0,
            Some(mana) => {
                self.decrease_mana(mana_cost);
                mana_cost * 2
            }
            None => {
                self.decrease_health(mana_cost);
                return 0;
            }
        };
        damage
    }

    fn decrease_mana(&mut self, mana_cost: u32) {
        self.mana = self.mana.map(|mana| mana.saturating_sub(mana_cost));
    }

    fn decrease_health(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage)
    }
}
