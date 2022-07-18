/* * Data Types
 * To ease the description of the format of each .db file,
 * the following names for data types will be used.
 * Unless otherwise specified, all numerical types are stored little-endian.
 * Integer values, including bytes, are all unsigned.
 * UTF-8 characters are stored in their canonical form,
 * with the higher-order byte first. */

pub use u16 as Short;
pub use u32 as Int;
pub use u64 as Long;
pub use u8 as Byte;

pub use f32 as Single;
pub use f64 as Double;

pub use u8 as Bool;

pub use u64 as DateTime;

pub struct IntDoublePair {
    pub flag: Byte,
    pub int: Int,
    pub flag2: Byte,
    pub double: Double,
}

pub struct TimingPoint {
    pub bpm: Double,
    pub offset: Double,     // ms
    pub is_inherited: Bool, // if false, then this timing point is inherited. See .osu (file format) for more information regarding timing points.
}
