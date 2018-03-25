use std::fmt;

use util::read_line;

use item::InventoryItem;
use item::equipment::Weapon;

pub struct Player {
    name: String,
    health: u8,
    rank: String,
    inventory: Vec<InventoryItem>,
    weapon: Weapon,
}

impl Player {
    pub fn introduce() -> Self {
        let mut name: String;

        loop {
            println!("What is your name?");
            name = read_line();

            println!("So you name is {}? Y/n", name);
            let ans = read_line();

            if ans.starts_with('Y') || ans.starts_with('y') {
                break;
            }
        }

        Player {
            name: name,
            health: 100,
            rank: String::from("noob"),
            inventory: Vec::new(),
            weapon: Weapon { strength: 1 },
        }
    }

    pub fn use_item(&mut self, item_idx: usize) {
        if item_idx >= self.inventory.len() {
            return;
        }
        match self.inventory[item_idx] {
            _ => (),
        }
    }
}

impl fmt::Display for Player {
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
            "{} -- {}\n[{}]({}%)",
            self.name, self.rank, h, self.health
        )
    }
}
