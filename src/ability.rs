use bevy::{gltf::Gltf, prelude::Vec3};

pub struct Character {
    pub hitbox: Vec3,
    pub scale: Vec3,
    pub asset: Gltf,
    pub emotes: u8,
}

pub struct Ability {
    pub ability_start: MyPlayerEffect,
    pub ability_active: Projectile,
    pub ability_end: Impact,
}
pub struct MyPlayerEffect {
    pub animation: Gltf,
    pub sound: String,
}
pub struct Projectile {
    pub proj: Gltf,
    pub info: AbilityInfo,
    pub sound: String,
}
pub struct AbilityInfo {
    pub damage: u32,
    pub distance: u32,
    pub speed: u32,
}

pub struct Impact {
    pub animation: String,
    pub info: AbilityInfo,
    pub sound: String,
}
