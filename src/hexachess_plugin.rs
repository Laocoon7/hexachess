use bevy::prelude::*;

use crate::{
    data::ChessPiece,
    resources::{ChessTextures, HighlightedHexes},
    systems::{draw_gizmos, highlight_hexes, spawn_camera, spawn_map},
};

pub struct HexachessPlugin;
impl Plugin for HexachessPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChessPiece>();
        app.init_resource::<ChessTextures>();
        app.init_resource::<HighlightedHexes>();

        app.add_systems(Startup, (spawn_camera, spawn_map));
        app.add_systems(Update, (highlight_hexes, draw_gizmos).chain());
    }
}
