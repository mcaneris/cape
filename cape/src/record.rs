use std::marker::PhantomData;
use std::ops::Not;

use crate::id::Id;
use crate::inner::Inner;
use crate::persistence::{N, Persistence, S};
use crate::timestamp::Timestamp;

/// A record wraps a domain object `T` with metadata and relations.
///
/// - `R`: relations (another Record, or tuple, or Vec<Record<_>>)
/// - `S`: persistence state (N, S)
/// - `K`: key type (i64, Uuid, ...)
#[derive(Debug, Clone)]
pub struct Record<T, R = (), S: Persistence = N, K = i64>
where
    T: Inner,
    R: Inner,
    K: Clone,
{
    // None for N, Some(K) for S
    id: Option<Id<K>>,

    pub inner: T,

    pub relations: R,

    created_at: Option<Timestamp>,

    updated_at: Option<Timestamp>,

    _dirty: bool,
    _state: PhantomData<S>,
}

/// Common implementation for all records
impl<T, R, S, K> Record<T, R, S, K>
where
    T: Inner,
    R: Inner,
    S: Persistence,
    K: Clone,
{
    pub fn is_dirty(&self) -> bool {
        self._dirty
    }

    pub fn mark_dirty(&mut self) {
        self._dirty = true;
    }
}

/// Implementation for records with persistence state N (New)
impl<T, K> Record<T, (), N, K>
where
    T: Inner,
    K: Inner,
{
    pub fn new(inner: T) -> Self {
        Self {
            id: None,
            inner,
            relations: (),
            created_at: None,
            updated_at: None,
            _dirty: true,
            _state: PhantomData,
        }
    }

    pub fn with_relations<R>(inner: T, relations: R) -> Record<T, R, N>
    where
        R: Inner,
    {
        Record {
            id: None,
            inner,
            relations,
            created_at: None,
            updated_at: None,
            _dirty: true,
            _state: PhantomData,
        }
    }
}

/// Implementation for records with persistence state N (New)
impl<T, K, R> Record<T, R, N, K>
where
    T: Inner,
    R: Inner,
    K: Clone,
{
    pub fn store(self, id: K, created_at: Timestamp) -> Record<T, R, S, K> {
        Record {
            id: Some(Id::new(id)),
            inner: self.inner,
            relations: self.relations,
            created_at: Some(created_at),
            updated_at: Some(created_at),
            _dirty: false,
            _state: PhantomData,
        }
    }
}

/// Implementation for records with persistence state S (Stored)
impl<T, K, R> Record<T, R, S, K>
where
    T: Inner,
    R: Inner,
    K: Clone,
{
    pub fn id(&self) -> &K {
        self.id
            .as_ref()
            .and_then(|id| id.get())
            .expect("stored record must have an id")
    }

    pub fn created_at(&self) -> &Timestamp {
        self.created_at
            .as_ref()
            .expect("stored record must have a created_at timestamp")
    }

    pub fn updated_at(&self) -> &Timestamp {
        self.updated_at
            .as_ref()
            .expect("stored record must have an updated_at timestamp")
    }

    pub fn store(self, id: K, created_at: Timestamp) -> Record<T, R, S, K> {
        if self._dirty.not() {
            return self;
        }

        Record {
            id: Some(Id::new(id)),
            inner: self.inner,
            relations: self.relations,
            created_at: Some(created_at),
            updated_at: Some(created_at),
            _dirty: false,
            _state: PhantomData,
        }
    }
}
