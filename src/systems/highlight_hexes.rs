use bevy::{prelude::*, window::PrimaryWindow};
use hexx::Hex;

use crate::{
    components::ChessPiece,
    resources::{HighlightedHexes, Map},
};

pub fn highlight_hexes(
    mut commands: Commands,
    window: Option<Single<&Window, With<PrimaryWindow>>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    map: Res<Map>,
    mut highlighted_hexes: ResMut<HighlightedHexes>,
) {
    let Some(window) = window else {
        return;
    };
    let window = window.into_inner();
    let (camera, camera_transform) = camera.into_inner();

    if let Some(hex) = get_hovered_hex(window, camera, camera_transform, &map) {
        if hex == highlighted_hexes.hovered {
            return;
        }
        highlighted_hexes.reset(&mut commands, &map);

        for hex in Hex::ZERO.line_to(hex) {
            highlighted_hexes.add_to_path(hex);
        }

        highlighted_hexes.hover(hex);
    }
}

fn get_hovered_hex(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    map: &Map,
) -> Option<Hex> {
    if let Some(pos) = window.cursor_position().and_then(|viewport_position| {
        camera
            .viewport_to_world_2d(camera_transform, viewport_position)
            .ok()
    }) {
        let hex = map.board.layout.world_pos_to_hex(pos);
        if map.tile_entities.contains_key(&hex) {
            Some(hex)
        } else {
            None
        }
    } else {
        None
    }
}
