use crate::Initiative;
use crate::Player;
use crate::powers::Power;

#[derive(Clone, Debug)]
pub struct Hero {
    pub energy: i64,
    pub attack: i64,
    pub initiative: Initiative,
    pub team: Player,
    pub powers: Vec<Power>,
    pub unspent_coins: i32,
    pub defence: i64,
    pub dead: bool,
    pub version: u8,
}

impl Hero {
    pub fn name(&self) -> String {
        if self.version == 1 {
            format!("{}", self.team.0)
        } else {
            format!("{} {}", self.team.0, self.version)
        }
    }

    pub fn check_is_dead(&self) -> bool {
        self.energy <= 0
    }
}
