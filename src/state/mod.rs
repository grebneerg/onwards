use player::Player;

/// Any event that could happen in the game.
pub trait Event {
    /// A description of the current state to inform the player of what's going on.
    fn describe(&self) -> String;

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
    fn describe(&self) -> String {
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
