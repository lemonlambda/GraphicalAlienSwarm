use graphical_alien_swarm_proc_macros::tileitem;
use serde::{Deserialize, Serialize};


/// First iteration for parsing the YAML file
/// Gets put into [[Tile]] and/or [[Item]] structs later on
#[tileitem]
#[derive(Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct TileItemRaw {
    name: String,
    category: String,
}

/// Internal data for variants
#[tileitem]
#[derive(Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct VariantInternal {
    name: String,
    default: Option<bool>,
    weight: f32,
    toughness: f32,
}

// Internal Data for Autotile
#[derive(Serialize, Deserialize)]
pub struct AutotileInternal {
    tile: String,
    default: String,
    nw: String,
    n: String,
    ne: String,
    w: String,
    e: String,
    sw: String,
    s: String,
    se: String,
}

/// Actual Tile struct for data
/// See [[TileItemRaw]] for the raw data
pub struct Tile {}

/// Actual Item struct for data
/// See [[TileItemRaw]] for the raw data
pub struct Item {}
