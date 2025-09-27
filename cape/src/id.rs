#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<K>(Option<K>);

impl<K> Id<K> {
    pub fn new(value: K) -> Self {
        Id(Some(value))
    }

    pub fn get(&self) -> Option<&K> {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Option<K> {
        self.0
    }
}
