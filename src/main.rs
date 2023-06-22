use std::cmp::{Ordering, Ord, PartialOrd};

#[derive(Clone, Debug)]
enum Power {
    BasicAttack,
}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Player(&'static str);

#[derive(Clone, Debug)]
struct Hero {
    energy: i64,
    attack: i64,
    initiative: Initiative,
    team: Player,
    powers: Vec<Power>,
}

#[derive(Clone, Debug)]
struct Battle {
    round: u8,
    heroes: Vec<Hero>,
}

impl Battle {
    fn sort_by_initiative(&mut self) {
        self.heroes.sort_by_key(|hero| std::cmp::Reverse(hero.initiative));
    }
}

fn main() {
    let mut battle = Battle {
        round: 0,
        heroes: vec![
            Hero {
                energy: 100,
                attack: 10,
                initiative: Initiative { value: 0, tiebreak: 0 },
                team: Player("Alice"),
                powers: vec![ Power::BasicAttack ],
            },
            Hero {
                energy: 100,
                attack: 10,
                initiative: Initiative { value: 0, tiebreak: 1 },
                team: Player("Bob"),
                powers: vec![ Power::BasicAttack ],
            },
            Hero {
                energy: 100,
                attack: 10,
                initiative: Initiative { value: 0, tiebreak: 2 },
                team: Player("Charlie"),
                powers: vec![ Power::BasicAttack ],
            }
        ]
    };
    battle.sort_by_initiative();
    println!("Battle: {:#?}", battle);
}
