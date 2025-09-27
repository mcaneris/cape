# 🦸 Cape &emsp;

> **⚠️ Work in Progress**: This project is currently under active development and not ready for production use.

**Cape** is an unintrusive and lightweight wrapper ORM for Rust.

[![Build Status]][actions] [![Docs]][docs.rs] [![Latest Version]][crates.io]

[Docs]: https://img.shields.io/docsrs/cape?style=for-the-badge&logo=docsdotrs&color=informational
[docs.rs]: https://docs.rs/cape
[Build Status]: https://img.shields.io/github/actions/workflow/status/mcaneris/cape/ci.yml?branch=main&style=for-the-badge&logo=github
[actions]: https://github.com/mcaneris/cape/actions
[Latest Version]: https://img.shields.io/crates/v/cape?style=for-the-badge&logo=rust&color=blueviolet
[crates.io]: https://crates.io/crates/cape

---

## ✨ Why Cape?

Most ORMs force core domain structs to mirror database schemas — IDs, foreign keys, and boilerplate that sullies the core domain model.

Cape flips this around by wrapping the domain model, just like a superhero’s cape. Core types stay clean and focused, while persistence details live in the wrapper.

---

## 🧩 Core Concepts

Cape centers on a single generic type:

```rust
Record<T, R, S, K>
```

* `T` → **inner entity** (core struct)
* `R` → **relations** (another `Record`, a tuple, or a `Vec<Record>`)
* `S` → **persistence state** (`N`, `S`)
* `K` → **key type** (`i64`, `Uuid`, etc.)

### Example

```rust
use cape::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub title: String,
    pub body: String,
}

fn main() {
    // New user, not yet persisted
    let new_user: Record<User, (), N, Uuid> = Record::new(User {
        username: "alice".into(),
        email: "alice@example.com".into(),
    });

    assert!(new_user.is_dirty(), true);

    // Stored user, persisted with a UUID
    let stored_user = new_user.store(Uuid::new_v4(), chrono::Utc::now().naive_utc());

    // New post with a stored author relation
    let new_post: Record<Post, Record<User, (), S, Uuid>, N, i64> =
        Record::with_relations(
            Post { title: "Hello".into(), body: "Cape world!" },
            stored_user,
        );

    // Move post into stored state
    let stored_post = new_post.store(42, chrono::Utc::now().naive_utc());

    assert!(new_user.is_dirty(), false);

    println!("Post '{}' by {}", 
        stored_post.inner.title,
        stored_post.relations.inner.username
    );
}
```

---

## 📜 License

<sup>
  Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>

