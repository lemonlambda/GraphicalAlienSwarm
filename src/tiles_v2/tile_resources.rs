use bevy::{prelude::*, reflect::Tuple};

#[derive(Resource)]
struct Tiles;

/// Easy way to index tiles
pub struct TileId<T>(T);

impl<'a, T: PartialEq<&U>, U: PartialEq> PartialEq<U> for TileId<T> {
    fn eq(&'a self, other: &'a U) -> bool {
        self.0 == other
    }
}

// impl<'a, T: Tuple, U: PartialEq> PartialEq<U> for TileId<T> {
//     fn eq(&'a self, other: &'a U) -> bool {
//         for field in self.0.iter_fields() {
//             match field.as_any().downcast_ref::<U>() {
//                 Some(val) => match val == other {
//                     true => return true,
//                     false => continue,
//                 },
//                 None => continue,
//             }
//         }
//         false
//     }
// }

// #[test]
// fn tile_id_test() {
//     let tile_id = TileId((10, "Cool"));
//     assert_eq!(10, tile_id);
//     assert_eq!("Cool", tile_id);
//     assert!("Funny" != tile_id);
// }
