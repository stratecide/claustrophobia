use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //array_texture_loader: Res<ArrayTextureLoader>,
) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            if x.min(y) > 5 && x.max(y) < 12 {
                continue;
            }
            if x.min(y) > 8 && x.max(y) < 16 {
                continue;
            }
            let position = TilePos { x, y };
            let tile_entity = commands.spawn(TileBundle {
                position,
                texture_index: TileTextureIndex (0),
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            }).id();
            tile_storage.set(&position, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let mut transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.);
    transform.scale.x = 0.75;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform,
        //transform: Transform::from_xyz(0., 0., 0.),
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

