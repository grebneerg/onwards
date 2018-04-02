mod util;
mod player;
mod item;
mod state;

use player::Player;
use util::read_line;
use state::{Event, GameState, Wandering, Battle, Enemy};

fn display_help(state: &GameState) {
    println!("General commands:");
    println!(
        "
        disp => Show player info
        exit => Exit game
        help => Display this information
    "
    );

    println!("{}", state.help_text());
}

fn main() {
    let mut state: GameState = Battle::with(Enemy {
        name: "Fred".to_string(),
        strength: 1,
        health: 10,
    }); //Wandering::at("Somewhere");

    println!("Welcome");

    let mut player = Player::introduce();
	
    println!("{}", player);
    loop {
        println!("{}", state.describe(&player));

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
