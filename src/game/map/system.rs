use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::resource::LevelHandle;
use crate::level_loader::Level;

pub fn build_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<Level>>,
) {
    let level_data = level_assets.get(&level_handle.handle).unwrap();

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let map_size = level_data.size;
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            if let Some(_tile) = level_data.tiles.get(&[x, y]) {
                let mut texture_index = TileTextureIndex(0);
                if x == map_size.x - 1 || level_data.tiles.get(&[x + 1, y]).is_some() {
                    texture_index.0 += 1;
                }
                // above
                if y == 0 || level_data.tiles.get(&[x, y - 1]).is_some() {
                    texture_index.0 += 2;
                }
                // left
                if x == 0 || level_data.tiles.get(&[x - 1, y]).is_some() {
                    texture_index.0 += 4;
                }
                // below
                if y == map_size.y - 1 || level_data.tiles.get(&[x, y + 1]).is_some() {
                    texture_index.0 += 8;
                }
                let position = TilePos { x, y: map_size.y - 1 - y };
                let tile_entity = commands.spawn(TileBundle {
                    position,
                    texture_index,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                }).id();
                tile_storage.set(&position, tile_entity);
            }
        }
    }

    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    //let transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.);
    //transform.scale.x = 0.5;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        //transform,
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });

    println!("tilemap set up");

}


pub fn fix_tilemap_edges(
    mut tile_query: Query<(&mut TileTextureIndex, &TilePos)>,
    tile_storage_query: Query<&TileStorage>,
) {
    if let Ok(tile_storage) = tile_storage_query.get_single() {
        for (mut texture_index, tile_pos) in tile_query.iter_mut() {
            if texture_index.0 == 0 {

                let &TilePos { x, y } = tile_pos;

                // right
                if x == tile_storage.size.x - 1 || tile_storage.get(&TilePos { x: x + 1, y }).is_some() {
                    texture_index.0 += 1;
                }
                // above
                if y == tile_storage.size.y - 1 || tile_storage.get(&TilePos { x, y: y + 1 }).is_some() {
                    texture_index.0 += 2;
                }
                // left
                if x == 0 || tile_storage.get(&TilePos { x: x - 1, y }).is_some() {
                    texture_index.0 += 4;
                }
                // below
                if y == 0 || tile_storage.get(&TilePos { x, y: y - 1 }).is_some() {
                    texture_index.0 += 8;
                }
            }
        }
    }
    println!("fixed tilemap edges");
}

