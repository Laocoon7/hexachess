use bevy::prelude::*;

use crate::resources::{HighlightedHexes, Map};

pub fn draw_highlights(
    mut commands: Commands,
    mut gizmos: Gizmos,
    map: Res<Map>,
    highlighted_hexes: Res<HighlightedHexes>,
) {
    highlighted_hexes.draw(&mut commands, &map);

    for coord in highlighted_hexes.path.iter() {
        for [start, end] in map.board.layout.all_edge_coordinates(*coord) {
            gizmos.line_2d(start, end, Color::WHITE);
        }
    }
}
