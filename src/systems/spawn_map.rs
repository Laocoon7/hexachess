use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::Anchor,
    utils::HashMap,
};
use hexx::{shapes, Hex, HexLayout, PlaneMeshBuilder};

use crate::{
    data::GlinskiBoard,
    resources::{ChessTextures, Map, TileColor},
};

const HEX_SIZE: f32 = 50.0;

pub fn spawn_map(
    mut commands: Commands,
    chess_textures: Res<ChessTextures>,
    mut a_meshes: ResMut<Assets<Mesh>>,
    mut a_materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = GlinskiBoard::new(HEX_SIZE);

    let tile_black_material = a_materials.add(Color::Srgba(css::BLACK));
    let tile_grey_material = a_materials.add(Color::Srgba(css::GREY));
    let tile_white_material = a_materials.add(Color::Srgba(css::BLANCHED_ALMOND));

    let hovered_material = a_materials.add(Color::Srgba(css::BLUE));
    let path_material = a_materials.add(Color::Srgba(css::ORANGE));
    let attack_material = a_materials.add(Color::Srgba(css::RED));

    let mesh_info = PlaneMeshBuilder::new(&board.layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.98))
        .center_aligned()
        .build();

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices));

    let mesh_handle = a_meshes.add(mesh);

    let mut piece_entities = HashMap::new();
    for (hex, maybe_piece) in board.pieces.iter() {
        if let Some(piece) = maybe_piece {
            let position = board.layout.hex_to_world_pos(*hex);
            let id = commands
                .spawn((
                    *piece,
                    Sprite {
                        image: chess_textures.handle.clone_weak(),
                        texture_atlas: Some(TextureAtlas {
                            layout: chess_textures.layout.clone_weak(),
                            index: piece.image_index(),
                        }),
                        custom_size: Some(Vec2::splat(64.0)),
                        anchor: Anchor::Custom(Vec2::new(0.0, -0.2)),
                        ..Default::default()
                    },
                    Transform::from_xyz(position.x, position.y, 0.0),
                ))
                .id();
            piece_entities.insert(*hex, id);
        }
    }

    let tile_entities = shapes::hexagon(Hex::ZERO, 5)
        .map(|hex| {
            let color = match Map::tile_color(hex) {
                TileColor::Grey => tile_grey_material.clone_weak(),
                TileColor::Black => tile_black_material.clone_weak(),
                TileColor::White => tile_white_material.clone_weak(),
            };
            let pos = board.layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(color),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2d(GlinskiBoard::to_glinski_notation(hex).unwrap()),
                        TextColor(Color::Srgba(css::REBECCA_PURPLE)),
                        TextFont {
                            font_size: 14.0,
                            ..Default::default()
                        },
                        Transform::from_xyz(0.0, -32.0, 10.0),
                    ));
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(Map {
        board,
        tile_entities,
        piece_entities,
        tile_black_material,
        tile_grey_material,
        tile_white_material,
        hovered_material,
        path_material,
        attack_material,
        // default_material,
    });
}
