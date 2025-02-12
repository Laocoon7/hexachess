use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{shapes, Hex, HexLayout, PlaneMeshBuilder};

use crate::resources::{Map, TileColor};

const HEX_SIZE: f32 = 20.0;

pub fn spawn_map(
    mut commands: Commands,
    mut a_meshes: ResMut<Assets<Mesh>>,
    mut a_materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout::flat().with_hex_size(HEX_SIZE);

    let tile_black_material = a_materials.add(Color::Srgba(css::BLACK));
    let tile_grey_material = a_materials.add(Color::Srgba(css::GREY));
    let tile_white_material = a_materials.add(Color::Srgba(css::BLANCHED_ALMOND));

    let hovered_material = a_materials.add(Color::Srgba(css::BLUE));
    let path_material = a_materials.add(Color::Srgba(css::ORANGE));
    let attack_material = a_materials.add(Color::Srgba(css::RED));
    // let default_material = a_materials.add(Color::Srgba(css::WHITE));

    let mesh_info = PlaneMeshBuilder::new(&layout)
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

    let entities = shapes::hexagon(Hex::ZERO, 5)
        .map(|hex| {
            let color = match Map::tile_color(hex) {
                TileColor::Grey => tile_grey_material.clone_weak(),
                TileColor::Black => tile_black_material.clone_weak(),
                TileColor::White => tile_white_material.clone_weak(),
            };
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(color),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text2d(format!("{},{},{}", hex.x, hex.y, hex.z())),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 7.0,
                            ..Default::default()
                        },
                        Transform::from_xyz(0.0, 0.0, 10.0),
                    ));
                })
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(Map {
        layout,
        entities,
        tile_black_material,
        tile_grey_material,
        tile_white_material,
        hovered_material,
        path_material,
        attack_material,
        // default_material,
    });
}
