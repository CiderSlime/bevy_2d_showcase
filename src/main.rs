//! In this example we generate a new texture atlas (sprite sheet) from a folder containing
//! individual sprites.

use bevy::{asset::LoadState, prelude::*};
use bevy::log::{Level, LogPlugin};
use bevy::render::color::Color;

use bevy_ecs_tilemap::prelude::*;

use rand::{thread_rng, Rng};

use noise::{utils::*, BasicMulti, Perlin};

mod plugins;
pub mod common;

use crate::common::AppState;

// Side length of a colored quadrant (in "number of tiles").
const QUADRANT_SIDE_LENGTH: u32 = 32;


fn main() {
    App::new()
        .init_resource::<RpgSpriteHandles>()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin{
                level: Level::DEBUG,
                filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string()
            }),
            plugins::camera::CameraPlugin
        )) // prevents blurry sprites
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), generate_world)
        // .add_systems(OnEnter(AppState::Setup), load_textures)
        // .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .run();
}

#[derive(Resource, Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn generate_noise_map() -> NoiseMap {
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    let basicmulti = BasicMulti::<Perlin>::new(seed);

    PlaneMapBuilder::<_, 2>::new(&basicmulti)
        // .set_size(100, 100)
        // .set_x_bounds(-5.0, 5.0)
        // .set_y_bounds(-5.0, 5.0)
        .build()
}

fn get_color(val: f64) -> Color {
    let color_result = match val.abs() {
        v if v < 0.1 => Color::hex("#0a7e0a"),
        v if v < 0.2 => Color::hex("#0da50d"),
        v if v < 0.3 => Color::hex("#10cb10"),
        v if v < 0.4 => Color::hex("#18ed18"),
        v if v < 0.5 => Color::hex("#3ff03f"),
        v if v < 0.6 => Color::hex("#65f365"),
        v if v < 0.7 => Color::hex("#8cf68c"),
        v if v < 0.8 => Color::hex("#b2f9b2"),
        v if v < 0.9 => Color::hex("#d9fcd9"),
        v if v <= 1.0 => Color::hex("#ffffff"),
        _ => panic!("unexpected value")
    };
    color_result.expect("Getting color from HEX error")
}

fn get_index(val: f64) -> u32 {
    match val.abs() {
        v if v < 0.1 => 0,
        v if v < 0.2 => 1,
        v if v < 0.3 => 3,
        v if v < 0.4 => 5,
        v if v < 0.5 => 7,
        v if v < 0.6 => 9,
        v if v < 0.7 => 11,
        v if v < 0.8 => 17,
        v if v < 0.9 => 18,
        v if v <= 1.0 => 19,
        _ => panic!("unexpected value")
    }
}

fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>
) {
    let texture_handle: Handle<Image> = asset_server.load("grassland_tiles.png");

    let map = generate_noise_map();
    let (grid_width, grid_height) = map.size();
    debug!("Map size: {}x{}", grid_width, grid_height);

    // let tile_size = 64_f32;
    //
    // let start_x = -(grid_width as f32) * tile_size / 2.0;
    // let start_y = -(grid_height as f32) * tile_size / 2.0;

    // In total, there will be `(QUADRANT_SIDE_LENGTH * 2) * (QUADRANT_SIDE_LENGTH * 2)` tiles.
    let map_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH * grid_width as u32,
        y: QUADRANT_SIDE_LENGTH * grid_height as u32,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    for col_x in 0..grid_width {
        for col_y in 0..grid_height {
            // debug!("Building tile: {}x{}", col_x, col_y);
            let val = map.get_value(col_x, col_y);
            let index = get_index(val);

            fill_tilemap_rect(
                TileTextureIndex(index),
                TilePos { x: QUADRANT_SIDE_LENGTH * col_x as u32, y: QUADRANT_SIDE_LENGTH * col_y as u32 },
                quadrant_size,
                tilemap_id,
                &mut commands,
                &mut tile_storage,
            );
            // if val > 0.8_f64 {
                // debug!("Value for {}:{} = {}", col_x, col_y, val);
            // }
            // let x = start_x + col_x as f32 * tile_size;
            // let y = start_y + col_y as f32 * tile_size;
            //
            // commands.spawn(
            //     SpriteBundle {
            //         sprite: Sprite {
            //             color: get_color(val),
            //             custom_size: Some(Vec2::new(tile_size, tile_size)),
            //             ..default()
            //         },
            //         transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            //         ..default()
            //     }
            // );
        }
    }
    let tile_size = TilemapTileSize { x: 64.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    debug!("Inserting TilemapBundle");
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    debug!("Switching State to Finished");
    next_state.set(AppState::Finished);
}

fn setup(
    mut commands: Commands,
    // rpg_sprite_handles: Res<RpgSpriteHandles>,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut textures: ResMut<Assets<Image>>,
) {
    debug!("Entering setup");
    // Build a `TextureAtlas` using the individual sprites
    // let mut texture_atlas_builder = TextureAtlasBuilder::default();
    // for handle in &rpg_sprite_handles.handles {
    //     let handle = handle.typed_weak();
    //     let Some(texture) = textures.get(&handle) else {
    //         warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(handle));
    //         continue;
    //     };
    //
    //     texture_atlas_builder.add_texture(handle, texture);
    // }
    //
    // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    // let texture_atlas_texture = texture_atlas.texture.clone();
    // let vendor_handle = asset_server.get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png");
    // let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    // let atlas_handle = texture_atlases.add(texture_atlas);

    // set up a scene to display our texture atlas
    // commands.spawn(Camera2dBundle::default());
    // draw a sprite from the atlas
    // commands.spawn(SpriteSheetBundle {
    //     transform: Transform {
    //         translation: Vec3::new(150.0, 0.0, 0.0),
    //         scale: Vec3::splat(4.0),
    //         ..default()
    //     },
    //     sprite: TextureAtlasSprite::new(vendor_index),
    //     texture_atlas: atlas_handle,
    //     ..default()
    // });
    // draw the atlas itself
    // commands.spawn(SpriteBundle {
    //     texture: texture_atlas_texture,
    //     transform: Transform::from_xyz(-300.0, 0.0, 0.0),
    //     ..default()
    // });
}
