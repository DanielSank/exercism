// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                level: self.level,
                mana: match self.level {
                    0..=9 => None,
                    _ => Some(100),
                }
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if mana_cost > self.health { self.health = 0; }
                else { self.health -= mana_cost; }
                return 0
            },
            Some(mut m) => {
                if mana_cost > m {
                    return 0;
                } else {
                    m = m - mana_cost;
                    self.mana = Some(m);  // Why do we need to do this?
                    return 2 * mana_cost
                }
            }
        }
    }
}
