use crate::map::TileMap as TileMapObject;

/// The `TileMap` resource.
///
/// This is a tile map for the current area/chunk/whatever.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[repr(transparent)]
pub struct TileMap(pub Option<TileMapObject>);
