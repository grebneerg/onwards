mod util;
mod player;
mod item;
mod state;

use player::Player;
use util::read_line;
use state::{Event, GameState, Wandering};

fn display_help(state: &GameState) {
    println!("General commands:");
    println!("
        disp => Show player info
        exit => Exit game
        help => Display this information
    ");

    println!("{}", state.help_text());
}

fn main() {
    let mut state: GameState = Wandering::at("Somewhere");

    println!("Welcome");

    let mut player = Player::introduce();

    println!("{}", player);
    loop {
        println!("{}", state.describe());

        let cmd = read_line();

        state = match cmd.to_lowercase().trim() {
            "disp" => {
                println!("{}", player);
                state
            }
            "help" => {
                display_help(&state);
                state
            }
            "exit" => break,
            s => state.next(&mut player, &s),
        }
    }
}
