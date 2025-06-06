pub enum AmmoType {
    Small,
    Medium,
    Large,
}

pub struct Weapon {
    damage: u32,
    bullets_per_shot: u32,
    magazine_size: u32,
    reload_time: u32,
    spread: f32,
    ammo_type: AmmoType,
}
pub enum ItemKind {
    Weapon(Weapon),
    Armor,
    Consumable,
}

pub struct Item {
    name: String,
    texture: String,
    texture_equipped: Option<String>,
    kind: ItemKind,
}
