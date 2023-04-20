const ROW_2: u32 = 4*3;
const ROW_3: u32 = 4*3*2;
const ROW_4: u32 = 4*3*3;

pub const JUST_DIRT: u32 = 0;
pub const WORM1: u32 = 0;
pub const WORM2: u32 = 0;
pub const WORM3: u32 = 0;
pub const LARGE_ROCKS: [u32; 4] = [4, 5, ROW_2 + 4, ROW_2 + 5];
pub const MEDIUM_ROCKS: [u32; 4] = [6, 7, ROW_2 + 6, ROW_2 + 7];
pub const SMALL_ROCKS: [u32; 4] = [ROW_3 + 4, ROW_3 + 5, ROW_4 + 4, ROW_4 + 5];
