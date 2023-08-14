use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};
use noise::{utils::*, BasicMulti, Perlin};
use crate::common::AppState;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
        app.add_systems(OnEnter(AppState::Setup), generate_world);
    }
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

// fn get_color(val: f64) -> Color {
//     let color_result = match val.abs() {
//         v if v < 0.1 => Color::hex("#0a7e0a"),
//         v if v < 0.2 => Color::hex("#0da50d"),
//         v if v < 0.3 => Color::hex("#10cb10"),
//         v if v < 0.4 => Color::hex("#18ed18"),
//         v if v < 0.5 => Color::hex("#3ff03f"),
//         v if v < 0.6 => Color::hex("#65f365"),
//         v if v < 0.7 => Color::hex("#8cf68c"),
//         v if v < 0.8 => Color::hex("#b2f9b2"),
//         v if v < 0.9 => Color::hex("#d9fcd9"),
//         v if v <= 1.0 => Color::hex("#ffffff"),
//         _ => panic!("unexpected value")
//     };
//     color_result.expect("Getting color from HEX error")
// }

fn get_index(val: f64) -> TileTextureIndex {
    let res = match val.abs() {
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
    };
    TileTextureIndex(res)
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

    // In total, there will be `(QUADRANT_SIDE_LENGTH * 2) * (QUADRANT_SIDE_LENGTH * 2)` tiles.
    let map_size = TilemapSize {
        x: grid_width as u32,
        y: grid_height as u32,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let position = TilePos { x, y };
                let texture_index = get_index(map.get_value(x as usize, y as usize));
                let tile_entity = parent
                    .spawn(TileBundle {
                        texture_index,
                        position,
                        tilemap_id,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&position, tile_entity);
            }
        }
    });

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