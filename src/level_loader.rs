//! adapted from bevy/examples/asset/custom_asset.rs

use std::collections::HashMap;
use std::fmt::Display;
use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy_ecs_tilemap::prelude::TilemapSize;
use serde_json::Value;

#[derive(Debug, TypeUuid)]
#[uuid = "48a7b56f-f52a-4d04-a4a5-66839157aeaa"]
pub struct Level {
    pub size: TilemapSize,
    pub tiles: HashMap<[u32; 2], u8>,
    pub player_pos: Vec2,
    pub pills: Vec<Vec2>,
    pub couches: Vec<Vec2>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "f348ffa1-aaeb-4ee1-ae4a-bc16294e16c0"]
pub struct LevelList {
    pub levels: Vec<(String, Handle<Level>)>,
}

#[derive(Default)]
pub struct LevelLoader;

#[derive(Debug)]
pub struct LevelLoaderError;
impl Display for LevelLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error loading level")
    }
}
impl std::error::Error for LevelLoaderError {

}

impl AssetLoader for LevelLoader {
    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let json = if let Value::Object(json) = serde_json::from_slice(bytes)? {
                Ok(json)
            } else {
                Err(LevelLoaderError)
            }?;
            let mut levels = vec![];
            for json_level in json.get("levels").unwrap().as_array().unwrap() {
                let label = json_level.get("identifier").unwrap().as_str().unwrap();
                let mut level = Level {
                    size: TilemapSize { x: 0, y: 0 },
                    tiles: HashMap::new(),
                    player_pos: Vec2::new(0., 0.),
                    pills: vec![],
                    couches: vec![],
                };
                for json_layer in json_level.get("layerInstances").unwrap().as_array().unwrap() {
                    let grid_size = json_layer.get("__gridSize").unwrap().as_u64().unwrap() as u32;
                    for json_tile in json_layer.get("gridTiles").unwrap().as_array().unwrap() {
                        let pos_array = json_tile.get("px").unwrap().as_array().unwrap();
                        let x = pos_array.get(0).unwrap().as_u64().unwrap() as u32 / grid_size;
                        let y = pos_array.get(1).unwrap().as_u64().unwrap() as u32 / grid_size;
                        level.size.x = level.size.x.max(x + 1);
                        level.size.y = level.size.y.max(y + 1);
                        level.tiles.insert([x, y], 0);
                    }
                    for json_entity in json_layer.get("entityInstances").unwrap().as_array().unwrap() {
                        let pos_array = json_entity.get("px").unwrap().as_array().unwrap();
                        let x = pos_array.get(0).unwrap().as_u64().unwrap() as f32;
                        let y = pos_array.get(1).unwrap().as_u64().unwrap() as f32;
                        level.size.x = level.size.x.max(x as u32 / grid_size + 1);
                        level.size.y = level.size.y.max(y as u32 / grid_size + 1);
                        match json_entity.get("__identifier").unwrap().as_str().unwrap() {
                            "Player" => level.player_pos = Vec2::new(x, y),
                            "Pill" => level.pills.push(Vec2::new(x, y)),
                            "Couch" => level.couches.push(Vec2::new(x + 8., y)),
                            _ => (),
                        }
                    }
                }
                let handle = load_context.set_labeled_asset(label, LoadedAsset::new(level));
                levels.push((label.to_string(), handle));
            }
            load_context.set_default_asset(LoadedAsset::new(LevelList {
                levels
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}

