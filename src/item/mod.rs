pub mod equipment {
    pub struct Weapon {
        pub strength: usize,
    }
}

pub enum InventoryItem {
    Weapon(equipment::Weapon),
}

pub struct Inventory;
