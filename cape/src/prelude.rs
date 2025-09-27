//! A prelude module for Cape
//!
//! This module contains the most commonly used items from Cape.
//! It's designed to be imported with `use cape::prelude::*`.

pub use crate::id::Id;
pub use crate::inner::{Inner, InnerRecord};
pub use crate::persistence::{N, Persistence, S};
pub use crate::record::Record;
pub use crate::timestamp::Timestamp;
