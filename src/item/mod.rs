pub mod equipment {
    pub struct Weapon {
        pub strength: u32,
    }
}

pub enum InventoryItem {
    Weapon(equipment::Weapon),
}

pub struct Inventory;
