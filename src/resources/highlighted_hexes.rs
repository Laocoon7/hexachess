use bevy::prelude::*;
use hexx::Hex;

use crate::resources::Map;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct HighlightedHexes {
    pub hovered: Hex,
    pub path: Vec<Hex>,
    pub attack: Vec<Hex>,
}

impl HighlightedHexes {
    pub fn reset(&mut self, commands: &mut Commands, map: &Map) {
        // reset hovered
        if let Some(entity) = map.tile_entities.get(&self.hovered).copied() {
            let color = map.get_tile_color(self.hovered);
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
        self.hovered = Hex::ZERO;

        // reset path
        for (entity, hex) in self
            .path
            .iter()
            .filter_map(|hex| map.tile_entities.get(hex).map(|entity| (*entity, *hex)))
        {
            let color = map.get_tile_color(hex);
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
        self.path.clear();

        // reset attack
        for (entity, hex) in self
            .attack
            .iter()
            .filter_map(|hex| map.tile_entities.get(hex).map(|entity| (*entity, *hex)))
        {
            let color = map.get_tile_color(hex);
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
        self.attack.clear();
    }

    pub fn hover(&mut self, hex: Hex) {
        self.hovered = hex;
    }

    pub fn add_to_path(&mut self, hex: Hex) {
        self.path.push(hex);
    }

    pub fn add_to_attack(&mut self, hex: Hex) {
        self.attack.push(hex);
    }

    pub fn draw(&self, commands: &mut Commands, map: &Map) {
        if let Some(entity) = map.tile_entities.get(&self.hovered).copied() {
            let color = map.hovered_material.clone_weak();
            commands.entity(entity).insert(MeshMaterial2d(color));
        }

        for hex in self.path.iter() {
            if let Some(entity) = map.tile_entities.get(hex).copied() {
                let color = map.path_material.clone_weak();
                commands.entity(entity).insert(MeshMaterial2d(color));
            }
        }

        for hex in self.attack.iter() {
            if let Some(entity) = map.tile_entities.get(hex).copied() {
                let color = map.attack_material.clone_weak();
                commands.entity(entity).insert(MeshMaterial2d(color));
            }
        }
    }
}
