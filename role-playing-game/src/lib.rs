// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {

    fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: if level < 10 {None} else {Some(100)},
            level
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(self_mana) => {
                if mana_cost > self_mana {
                    0
                }  else {
                    self.mana = Some(self_mana - mana_cost);
                    2 * mana_cost
                }
            },
            None => {
                self.health -= min(self.health, mana_cost);
                0
            }
        }
    }
}
