use std::fmt;

use player::{Player, PlayerCondition};

/// Any event that could happen in the game.
pub trait Event {
    /// A description of the current state to inform the player of what's going on.
    fn describe(&self, &Player) -> String;

    /// Advance to the next state based on user input.
    fn next(self: Box<Self>, &mut Player, &str) -> GameState;

    /// Any event specific command information.
    fn help_text(&self) -> String;
}

/// A wrapper type to represent the game state.
pub type GameState = Box<Event>;

/// The player is just wandering somwhere.
pub struct Wandering {
    location: String,
}

impl Wandering {
    /// Create a new Wandering event with the given location.
    pub fn at<S: Into<String>>(location: S) -> GameState {
        Box::new(Wandering {
            location: location.into(),
        })
    }
}

impl Event for Wandering {
    fn describe(&self, _: &Player) -> String {
        format!("You are wandering {}", self.location)
    }

    fn next(self: Box<Self>, player: &mut Player, cmd: &str) -> GameState {
        match cmd {
            "move" => Wandering::at("elsewhere"),
            _ => {
                use util;
                util::invalid_command(cmd);
                self
            }
        }
    }

    fn help_text(&self) -> String {
        format!("Wandering help: type 'move' to move")
    }
}

pub struct Enemy {
    pub health: u32,
    pub strength: u32,
    pub name: String,
}

impl Enemy {
    pub fn new<S: Into<String>>(health: u32, strength: u32, name: S) -> Self {
        Enemy {
            health,
            strength,
            name: name.into()
        }
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut h = String::new();
        for i in 0..20 {
            if i * 5 < self.health {
                h.push('=');
            } else {
                h.push(' ');
            }
        }

        write!(
            f,
            "{} -- Strength: {}\n[{}]({}%)",
            self.name, self.strength, h, self.health
        )
    }
}

pub struct Battle {
    enemy: Enemy,
}

impl Battle {
    pub fn with(enemy: Enemy) -> GameState {
        Box::new(Battle { enemy })
    }
}

impl Event for Battle {
    fn describe(&self, player: &Player) -> String {
        format!("You are in a battle!\n{}\nVS\n{}", self.enemy, player)
    }

    /// Advance to the next state based on user input.
    fn next(mut self: Box<Self>, player: &mut Player, cmd: &str) -> GameState {
        /*
        attack
        */
        match cmd {
            // TODO: add more commands
            "attack" => {
                println!("You dealt {} damage", player.weapon.strength);
                if self.enemy.health <= player.weapon.strength {
                    self.enemy.health = 0;
                    println!("{}\nYou have vanquished {}!", self.enemy, self.enemy.name);
                    Wandering::at("at the zoo")
                } else {
                    self.enemy.health -= player.weapon.strength;
                    match player.damage(self.enemy.strength) {
                        PlayerCondition::Alive => self,
                        PlayerCondition::Dead => Wandering::at("in the afterlife"),
                    }
                }
            }
            _ => {
                use util;
                util::invalid_command(cmd);
                self
            }
        }
    }

    fn help_text(&self) -> String {
        format!(
            "Battle help:\n
        attack => Perform a simple attack
        "
        )
    }
}
