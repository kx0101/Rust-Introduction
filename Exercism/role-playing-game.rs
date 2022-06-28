// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}
impl Player {
    pub fn revive(&self) -> Option<Player> {
        (self.health <= 0).then(||
            Self {
                health: 100,
                mana: (self.level > 9).then(|| 100),
                level: self.level})
    }
    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            match mana >= mana_cost {
                true => {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                }
                false => 0,
            }
        } else {
            self.health = match self.health >= mana_cost {
                true => self.health - mana_cost,
                false => 0
            };
            0
        }
    }
}
