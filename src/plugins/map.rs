use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::{thread_rng, Rng};
use noise::{utils::*, Perlin, Terrace};
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

    // let basicmulti = BasicMulti::<Perlin>::new(seed);

    let perlin = Perlin::new(1);

    let terrace_inverted = Terrace::new(perlin)
        .add_control_point(-1.0)
        .add_control_point(-0.5)
        .add_control_point(0.1)
        .add_control_point(1.0)
        .invert_terraces(true);

    PlaneMapBuilder::<_, 2>::new(&terrace_inverted)
        .set_size(10, 10)
        // .set_x_bounds(-5.0, 5.0)
        // .set_y_bounds(-5.0, 5.0)
        .build()
}

fn get_index(val: f64) -> TileTextureIndex {
    let mut rng = thread_rng();
    let res = match val.abs() {
        // v if v < 0.11 => rng.gen_range(9..=12),
        _ => rng.gen_range(0..=3),

    };
    TileTextureIndex(res)
}

fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>
) {
    let texture_handle: Handle<Image> = asset_server.load("grass_and_water.png");

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

    let tile_size = TilemapTileSize { x: 64.0, y: 48.0 };
    let grid_size = TilemapGridSize { x: 64.0, y: 40.0 };
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