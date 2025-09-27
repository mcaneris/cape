use crate::persistence::Persistence;
use crate::record::Record;

pub trait Inner: Clone {}
pub trait InnerRecord: Inner {}

impl<T: Clone> Inner for T {}
impl<T: Inner, R: Inner, S: Persistence, K: Clone> InnerRecord for Record<T, R, S, K> {}
