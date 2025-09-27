/// Marker trait for persistence state of a Record
pub trait Persistence: Clone {}

/// An unstored record
#[derive(Clone)]
pub struct N;

/// A stored record
#[derive(Clone)]
pub struct S;

impl Persistence for N {}
impl Persistence for S {}
