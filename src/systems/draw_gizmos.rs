use bevy::prelude::*;

use crate::resources::{HighlightedHexes, Map};

pub fn draw_gizmos(mut gizmos: Gizmos, highlighted_hexes: Res<HighlightedHexes>, map: Res<Map>) {
    for coord in highlighted_hexes.path.iter() {
        for [start, end] in map.layout.all_edge_coordinates(*coord) {
            gizmos.line_2d(start, end, Color::WHITE);
        }
    }
}
