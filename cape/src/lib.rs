//! Cape: Type-safe record and persistence state management
//!
//! Cape provides a type-safe way to manage domain objects with persistence
//! state tracking. Records wrap domain objects with metadata, relations, and
//! track their persistence state at the type level to prevent common database
//! synchronization errors.
//!
//! ## Core Concepts
//!
//! - **Record<T, R, S, K>**: Wraps a domain object `T` with relations `R`,
//!   persistence state `S`, and key type `K`
//! - **Persistence States**: Type-level tracking of whether records are new
//!   (`N`) or stored (`S`)
//! - **Inner Trait**: Marker trait for types that can be stored in records
//! - **`Id<K>`**: Optional identifier wrapper that enforces correct usage
//!   patterns
//! - **Timestamps**: Creation and update time tracking (optional chrono
//!   integration)
//!
//! ## Example
//!
//! ```rust
//! use cape::prelude::*;
//!
//! #[derive(Clone)]
//! struct User {
//!     name: String,
//!     email: String,
//! }
//!
//! // Create a new record (type: Record<User, (), N, i64>)
//! let new_user: Record<User, (), N, i64> = Record::new(User {
//!     name: "Alice".to_string(),
//!     email: "alice@example.com".to_string(),
//! });
//!
//! // After saving to database, convert to stored state
//! // let stored_user = new_user.store(123, timestamp);
//! // let user_id = stored_user.id(); // Only available for stored records
//! ```

pub(crate) mod id;
pub(crate) mod inner;
pub(crate) mod persistence;
pub mod prelude;
pub(crate) mod record;
pub(crate) mod timestamp;

pub use id::Id;
pub use inner::{Inner, InnerRecord};
pub use persistence::{N, Persistence, S};
pub use record::Record;
pub use timestamp::Timestamp;
