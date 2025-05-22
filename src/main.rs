use crate::powers::Power;
use crate::hero::Hero;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashSet;

mod powers;
mod hero;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Initiative {
    value: u32,
    tiebreak: u8,
}

impl Ord for Initiative {
    fn cmp(&self, other: &Initiative) -> Ordering {
        let order = self.value.cmp(&other.value);
        if let Ordering::Equal = order {
            self.tiebreak.cmp(&other.tiebreak)
        } else {
            order
        }
    }
}

impl PartialOrd for Initiative {
    fn partial_cmp(&self, other: &Initiative) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Player(&'static str);


#[derive(Clone, Debug)]
struct Battle {
    round: u8,
    heroes: Vec<Hero>,
}

impl Battle {
    fn sort_by_initiative(&mut self) {
        self.heroes
            .sort_by_key(|hero| std::cmp::Reverse(hero.initiative));
    }

    fn apply_passives(&mut self) {
        for hero in self.heroes.iter_mut() {
            hero.energy += hero.unspent_coins as i64;
            if hero.unspent_coins > 0 {
                hero.initiative.value += hero.unspent_coins as u32;
            } else {
                hero.initiative.value -= hero.unspent_coins.abs() as u32;
            }
        }
    }

    fn teams_left(&self) -> usize {
        let teams: HashSet<Player> = self.heroes.iter().map(|hero| hero.team).collect();
        teams.len()
    }

    fn start_of_round(&mut self) {
        self.round += 1;
        println!("\nStart of Round {:?}", self.round);
    }

    fn most_energetic(&self) -> &Hero {
        self.heroes.iter().max_by(|hero0, hero1| {
            hero0.energy.cmp(&hero1.energy).then(hero0.initiative.cmp(&hero1.initiative))
        }).unwrap()
    }

    /// Returns the mutable matching hero to the provided hero's team and version, to
    /// lookup the current state of a hero in the battle with a stale copy
    fn matching(&mut self, hero: &Hero) -> &mut Hero {
        self.heroes.iter_mut().find(|h| h.team == hero.team && h.version == hero.version).unwrap()
    }

    fn during_round(&mut self) {
        println!("\nRound {:?}", self.round);
        for hero in self.heroes.clone() {
            if self.matching(&hero).dead {
                println!("{} died before taking their turn", hero.name());
                continue;
            }
            println!("{} is up", hero.name());
            for power in hero.powers.clone() {
                if self.matching(&hero).energy <= 0 {
                    println!("{} ran out of energy during their own turn", hero.name());
                    break;
                }
                match power {
                    Power::BasicAttack => {
                        for target in self.heroes.iter_mut().filter(|h| h.team != hero.team) {
                            let mut damage = hero.attack;
                            let target_already_dead = target.dead;
                            println!("{} attacked {} for {} damage", hero.name(), target.name(), damage);
                            if target.defence > 0 {
                                let blocked = std::cmp::max(0, damage - target.defence);
                                println!("{} blocked {} damage with their defence", target.name(), blocked);
                                target.defence -= blocked;
                                damage -= blocked;
                            }
                            target.energy -= damage;
                            println!("{} took {} damage (remaining defence {} & energy {})", target.name(), damage, target.defence, target.energy);
                            if !target_already_dead {
                                if target.check_is_dead() {
                                    println!("{} killed {}!", hero.name(), target.name());
                                    target.dead = true;
                                }
                            }
                        }
                    },
                    Power::Amoeba |
                    Power::Souleater |
                    Power::Crystallize |
                    Power::CosmicShield |
                    Power::TitaniumSkin |
                    Power::BigGnashyClaws => (),
                }
            }
            println!("");
        }
    }

    fn end_of_round(&mut self) {
        println!("\nEnd of Round {:?}", self.round);
        self.heroes.retain(|hero| {
            if hero.dead {
                println!("{} died this round", hero.name());
            }
            !hero.dead
        });
    }
}

fn main() {
    let mut battle = Battle {
        round: 0,
        heroes: vec![
            Hero {
                energy: 100,
                attack: 10,
                initiative: Initiative {
                    value: 0,
                    tiebreak: 0,
                },
                team: Player("Alice"),
                powers: vec![Power::BasicAttack, Power::Souleater, Power::TitaniumSkin],
                unspent_coins: 18,
                defence: 0,
                dead: false,
                version: 1,
            },
            Hero {
                energy: 100,
                attack: 10,
                initiative: Initiative {
                    value: 0,
                    tiebreak: 1,
                },
                team: Player("Bob"),
                powers: vec![
                    Power::BasicAttack,
                    Power::Crystallize,
                    Power::Amoeba,
                    Power::CosmicShield,
                ],
                unspent_coins: 10,
                defence: 0,
                dead: false,
                version: 1,
            },
            Hero {
                energy: 100,
                attack: 10,
                initiative: Initiative {
                    value: 0,
                    tiebreak: 2,
                },
                team: Player("Charlie"),
                powers: vec![
                    Power::BasicAttack,
                    Power::Crystallize,
                    Power::BigGnashyClaws,
                ],
                unspent_coins: 13,
                defence: 0,
                dead: false,
                version: 1,
            },
        ],
    };
    battle.apply_passives();
    battle.sort_by_initiative();
    println!("Battle: {:#?}", battle);
    loop {
        battle.start_of_round();
        battle.during_round();
        battle.end_of_round();
        let teams_left = battle.teams_left();
        if teams_left == 0 {
            println!("\n\nEveryone is dead");
            break;
        } else if teams_left == 1 {
            let winner = battle.heroes[0].team;
            println!("\n\n{} wins!", winner.0);
            break;
        }
        if battle.round == 30 {
            let winner = battle.most_energetic();
            println!("\n\n{} wins by having the most energy at the end of round 30", winner.name());
            break;
        }
    }
}
