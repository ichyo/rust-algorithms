//! data structures for efficient operations
//!
//! Now, it contains these data structures
//! * BIT: [`BIT`], [`BITRange`]
//! * UnionFind: [`UnionFind`]
//!
mod bit;
mod union_find;

pub use self::bit::{BITRange, BIT};
pub use self::union_find::UnionFind;
