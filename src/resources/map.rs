use bevy::{prelude::*, utils::HashMap};
use hexx::{Hex, HexLayout};

use crate::data::GlinskiBoard;

#[derive(Default, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    Black,
    Grey,
    #[default]
    White,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Map {
    pub board: GlinskiBoard,
    pub tile_entities: HashMap<Hex, Entity>,
    pub piece_entities: HashMap<Hex, Entity>,

    pub tile_black_material: Handle<ColorMaterial>,
    pub tile_grey_material: Handle<ColorMaterial>,
    pub tile_white_material: Handle<ColorMaterial>,

    pub hovered_material: Handle<ColorMaterial>,
    pub path_material: Handle<ColorMaterial>,
    pub attack_material: Handle<ColorMaterial>,
    // pub default_material: Handle<ColorMaterial>,
}

impl Map {
    pub fn get_tile_color(&self, hex: Hex) -> Handle<ColorMaterial> {
        match Self::tile_color(hex) {
            TileColor::Grey => self.tile_grey_material.clone_weak(),
            TileColor::Black => self.tile_black_material.clone_weak(),
            TileColor::White => self.tile_white_material.clone_weak(),
        }
    }

    pub fn tile_color(hex: Hex) -> TileColor {
        match (((hex.x() - hex.y()) % 3) + 3) % 3 {
            0 => TileColor::Grey,
            1 => TileColor::Black,
            2 => TileColor::White,
            _ => unreachable!(),
        }
    }
}
