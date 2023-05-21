use rand::Rng;
use seq_macro::seq;

seq!(N in 1..=15 {
    #[repr(u8)]
    pub enum TileId {
        Air,
        Dirt,
        #(Rock~N,)*
    }
});

macro_rules! probable {
    ($($prob:literal => $expr:expr),+) => {{
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(1..=100);
        let mut ranges = {
            let mut temp = vec![];
            let mut count = 0;
            $(
                count += 1;
                temp.push(count..(count + $prob));
                count += $prob;
            )+
            temp
        };
        $(
            if ranges[${index()}].contains(&rand) {
                return $expr;
            }
        )+
        return TileId::Air;
    }}
}

pub fn surface_tile() -> TileId {
    probable!(
        15 => TileId::Air,
        15 => TileId::Rock1,
        70 => TileId::Dirt
    )
}
