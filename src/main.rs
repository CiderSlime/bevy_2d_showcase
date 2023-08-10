//! In this example we generate a new texture atlas (sprite sheet) from a folder containing
//! individual sprites.

use bevy::{asset::LoadState, prelude::*};
use bevy::log::{Level, LogPlugin};
use bevy::render::color::Color;

use rand::{thread_rng, Rng};

use noise::{utils::*, BasicMulti, Perlin};


fn main() {
    App::new()
        .init_resource::<RpgSpriteHandles>()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin{
                level: Level::DEBUG,
                filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string()
            })
        ) // prevents blurry sprites
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), generate_world)
        // .add_systems(OnEnter(AppState::Setup), load_textures)
        // .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

// fn load_textures(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder

    // rpg_sprite_handles.handles = asset_server.load_folder("textures/rpg").unwrap();
// }

// fn check_textures(
//     mut next_state: ResMut<NextState<AppState>>,
//     rpg_sprite_handles: ResMut<RpgSpriteHandles>,
//     asset_server: Res<AssetServer>,
// ) {
//     // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
//     if let LoadState::Loaded = asset_server
//         .get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id()))
//     {
//         next_state.set(AppState::Finished);
//     }
// }

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

fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>
) {
    let map = generate_noise_map();
    let (grid_width, grid_height) = map.size();
    debug!("Map size: {}x{}", grid_width, grid_height);

    let tile_size = 32_f32;

    for col_x in 0..grid_width {
        for col_y in 0..grid_height {
            let val = map.get_value(col_x, col_y);
            // if val > 0.8_f64 {
                // debug!("Value for {}:{} = {}", col_x, col_y, val);
            // }
            let x = col_x as f32 * tile_size;
            let y = col_y as f32 * tile_size;

            commands.spawn(
                SpriteBundle {
                    sprite: Sprite {
                        color: get_color(val),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                }
            );
        }
    }
     next_state.set(AppState::Finished);
}

fn setup(
    mut commands: Commands,
    // rpg_sprite_handles: Res<RpgSpriteHandles>,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut textures: ResMut<Assets<Image>>,
) {
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
    commands.spawn(Camera2dBundle::default());
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
