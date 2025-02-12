use bevy::prelude::*;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ChessTextures {
    pub handle: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl ChessTextures {
    pub fn sprite(&self) {}
}

impl FromWorld for ChessTextures {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let handle = asset_server.load("chess.png");
        let layout = asset_server.add(TextureAtlasLayout::from_grid(
            UVec2::splat(64),
            6,
            2,
            None,
            None,
        ));

        Self { handle, layout }
    }
}
