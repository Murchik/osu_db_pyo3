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
