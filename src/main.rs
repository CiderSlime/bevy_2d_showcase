//! In this example we generate a new texture atlas (sprite sheet) from a folder containing
//! individual sprites.

use bevy::{
    // asset::LoadState,
    prelude::*
};
use bevy::log::{Level, LogPlugin};


mod plugins;
pub mod common;

use crate::common::AppState;


fn main() {
    App::new()
        // .init_resource::<RpgSpriteHandles>()
        .add_plugins((DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin{
                level: Level::DEBUG,
                filter: "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string()
            }),
            plugins::map::MapPlugin,
            plugins::camera::CameraPlugin
        )) // prevents blurry sprites
        .add_state::<AppState>()
        // .add_systems(OnEnter(AppState::Setup), load_textures)
        // .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .run();
}

// #[derive(Resource, Default)]
// struct RpgSpriteHandles {
//     handles: Vec<HandleUntyped>,
// }


fn setup(
    // mut commands: Commands,
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
