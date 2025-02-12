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
        if let Some(entity) = map.entities.get(&self.hovered).copied() {
            let color = map.get_tile_color(self.hovered);
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
        self.hovered = Hex::ZERO;

        // reset path
        for (entity, hex) in self
            .path
            .iter()
            .filter_map(|hex| map.entities.get(hex).map(|entity| (*entity, *hex)))
        {
            let color = map.get_tile_color(hex);
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
        self.path.clear();

        // reset attack
        for (entity, hex) in self
            .attack
            .iter()
            .filter_map(|hex| map.entities.get(hex).map(|entity| (*entity, *hex)))
        {
            let color = map.get_tile_color(hex);
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
        self.attack.clear();
    }

    pub fn hover(&mut self, commands: &mut Commands, map: &Map, hex: Hex) {
        self.hovered = hex;
        if let Some(entity) = map.entities.get(&hex).copied() {
            let color = map.hovered_material.clone_weak();
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
    }

    pub fn add_to_path(&mut self, commands: &mut Commands, map: &Map, hex: Hex) {
        self.path.push(hex);
        if let Some(entity) = map.entities.get(&hex).copied() {
            let color = map.path_material.clone_weak();
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
    }

    pub fn add_to_attack(&mut self, commands: &mut Commands, map: &Map, hex: Hex) {
        self.attack.push(hex);
        if let Some(entity) = map.entities.get(&hex).copied() {
            let color = map.attack_material.clone_weak();
            commands.entity(entity).insert(MeshMaterial2d(color));
        }
    }
}
