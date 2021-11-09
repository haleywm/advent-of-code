use std::cmp::max;

#[derive(Debug, Clone)]
struct Game {
    boss_health: i16,
    boss_attack: i16,
    player_health: i16,
    player_mana: i16,
    shield_turns: i16,
    poison_turns: i16,
    recharge_turns: i16,
    total_mana: i16
}

enum Outcome {
    Win(i16),
    Lose,
    Invalid,
    Continue,
}

impl Game {
    fn new(player_health: i16, player_mana: i16, boss_health: i16, boss_attack: i16) -> Game {
        Game { boss_health, boss_attack, player_health, player_mana, shield_turns: 0, poison_turns: 0, recharge_turns: 0, total_mana: 0 }
    }

    fn do_turn(&mut self, move_id: i16) -> Outcome {
        // Player turn
        // Applying effects
        if self.recharge_turns > 0 {
            self.player_mana += 101;
            self.recharge_turns -= 1;
        }
        if self.poison_turns > 0 {
            self.boss_health -= 3;
            self.poison_turns -= 1;
        }
        if self.shield_turns > 0 {
            self.shield_turns -= 1;
        }
        // Checking if poison killed
        if self.boss_health <= 0 {
            return Outcome::Win(self.total_mana);
        }
        // Then, player performs move
        match move_id {
            0 => {
                // Magic missile
                if self.player_mana < 53 {
                    // Can't
                    return Outcome::Invalid
                }
                else {
                    self.player_mana -= 53;
                    self.total_mana += 53;
                    self.boss_health -= 4;
                }
            }
            1 => {
                // Drain
                if self.player_mana < 73 {
                    return Outcome::Invalid;
                }
                else {
                    self.player_mana -= 73;
                    self.total_mana += 73;
                    self.player_health += 2;
                    self.boss_health -= 2;
                }
            }
            2 => {
                // Shield
                if self.player_mana < 113 || self.shield_turns > 0 {
                    return Outcome::Invalid;
                }
                else {
                    self.player_mana -= 113;
                    self.total_mana += 113;
                    self.shield_turns = 6;
                }
            }
            3 => {
                // Poison
                if self.player_mana < 173 || self.poison_turns > 0 {
                    return Outcome::Invalid;
                }
                else {
                    self.player_mana -= 173;
                    self.total_mana += 173;
                    self.poison_turns = 6;
                }
            }
            4 => {
                // Recharge turns
                if self.player_mana < 229 || self.recharge_turns > 0 {
                    return Outcome::Invalid;
                }
                else {
                    self.player_mana -= 229;
                    self.total_mana += 229;
                    self.recharge_turns = 5;
                }
            }
            _ => panic!("Invalid move")
        }
        // Boss turn
        // Applying effects
        if self.recharge_turns > 0 {
            self.player_mana += 101;
            self.recharge_turns -= 1;
        }
        if self.poison_turns > 0 {
            self.boss_health -= 3;
            self.poison_turns -= 1;
        }
        // Checking if boss died
        if self.boss_health <= 0 {
            return Outcome::Win(self.total_mana);
        }
        // Finally, boss takes move, processing shield at same time
        if self.shield_turns > 0 {
            self.player_health -= max(1, self.boss_attack - 7);
            self.shield_turns -= 1;
        }
        else {
            self.player_health -= self.boss_attack;
        }
        // Checking if player died
        if self.player_health <= 0 {
            return Outcome::Lose;
        }

        // Nothing exciting happened
        Outcome::Continue
    }

    fn damage_player(&mut self, amount: i16) -> bool {
        // Deal damage to the player and return if this killed them
        self.player_health -= amount;
        self.player_health <= 0
    }
}

fn main() {
    let game = Game::new(50, 500, 51, 9);

    let mut lowest_mana: Option<i16> = None;

    let mut games: Vec<Game> = vec![game];
    let mut hard_games = games.clone();

    while let Some(next) = games.pop() {
        for next_turn in 0..5 {
            let mut cur = next.clone();
            match cur.do_turn(next_turn) {
                Outcome::Win(mana) => {
                    // Win! See if mana usage is better than current best
                    if lowest_mana.is_none() || lowest_mana.unwrap() > mana {
                        lowest_mana = Some(mana);
                    }
                },
                Outcome::Continue => games.push(cur), // Games continuing, add it to the list
                Outcome::Lose | Outcome::Invalid => {} // Do nothing
            }
        }
    }

    match lowest_mana {
        Some(mana) => println!("Lowest needed: {}", mana),
        None => println!("No wins found"),
    }

    lowest_mana = None;

    while let Some(mut next) = hard_games.pop() {
        if !next.damage_player(1) {
            // Player took one damage but survived
            for next_turn in 0..5 {
                let mut cur = next.clone();
                match cur.do_turn(next_turn) {
                    Outcome::Win(mana) => {
                        // Win! See if mana usage is better than current best
                        if lowest_mana.is_none() || lowest_mana.unwrap() > mana {
                            lowest_mana = Some(mana);
                        }
                    },
                    Outcome::Continue => hard_games.push(cur), // Games continuing, add it to the list
                    Outcome::Lose | Outcome::Invalid => {} // Do nothing
                }
            }
        }
    }

    match lowest_mana {
        Some(mana) => println!("Lowest needed: {}", mana),
        None => println!("No wins found"),
    }
}
