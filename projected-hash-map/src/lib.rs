use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// Wraps around `Edge` struct. The main feature of this struct, is that it's hashed by
/// `(Edge::key.0, Edge::key.1)` pair instead of `(Edge::key.0, Edge::key.1, Edge::nonce)`
/// triple.
#[derive(std::cmp::Eq)]
pub struct HashMapHelper<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    inner: V,
    pd: PhantomData<T>,
}

impl<T, V> Borrow<T> for HashMapHelper<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    fn borrow(&self) -> &T {
        self.inner.borrow()
    }
}

impl<T, V> PartialEq for HashMapHelper<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.borrow() == other.inner.borrow()
    }
}

impl<T, V> Hash for HashMapHelper<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state)
    }
}

pub(crate) struct ProjectedHashMap<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    repr: HashSet<HashMapHelper<T, V>>,
    pd: PhantomData<T>,
}

impl<T, V> Default for ProjectedHashMap<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    fn default() -> Self {
        Self { repr: HashSet::new(), pd: PhantomData }
    }
}

impl<T, V> ProjectedHashMap<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
        HashMapHelper<T, V>: Eq,
{
    /// Note: We need to remove the value first.
    /// The insert inside HashSet, will not remove existing element if it has the same key.
    pub(crate) fn insert(&mut self, edge: V) {
        self.repr.replace(HashMapHelper { inner: edge, pd: PhantomData });
    }

    pub(crate) fn get(&self, key: &T) -> Option<&V> {
        self.repr.get(key).map(|v| &v.inner)
    }

    pub(crate) fn remove(&mut self, key: &T) -> bool {
        self.repr.remove(key)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &V> + '_ {
        self.repr.iter().map(|it| &it.inner)
    }

    #[allow(unused)]
    pub(crate) fn len(&self) -> usize {
        self.repr.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::routing::projected_hash_map::ProjectedHashMap;
    use crate::routing::Edge;
    use near_primitives::network::PeerId;
    use crate::ProjectedHashMap;

    #[test]
    fn test_remove_key() {
        let p1 = PeerId::random();
        let p2 = PeerId::random();
        let p3 = PeerId::random();
        let p4 = PeerId::random();
        let e1 = Edge::make_fake_edge(p1, p2, 1);
        let e2 = Edge::make_fake_edge(p3, p4, 1);
        let mut se = ProjectedHashMap::default();
        se.insert(e2.clone());

        let key = e1.key().clone();
        se.insert(e1.clone());
        assert_eq!(se.get(&key).unwrap(), &e1);
        se.remove(e1.key());
        assert_eq!(se.get(&key), None);

        let key2 = e2.key().clone();
        assert_eq!(se.get(&key2).unwrap(), &e2);
    }

    #[test]
    fn test_hashset() {
        let p1 = PeerId::random();
        let p2 = PeerId::random();
        let p3 = PeerId::random();
        let e0 = Edge::make_fake_edge(p1.clone(), p3, 1);
        let e1 = Edge::make_fake_edge(p1.clone(), p2.clone(), 1);
        let e2 = Edge::make_fake_edge(p1.clone(), p2.clone(), 2);
        let e3 = Edge::make_fake_edge(p2, p1, 3);

        let mut se = ProjectedHashMap::default();
        se.insert(e0.clone());
        se.insert(e1);
        se.insert(e2);
        se.insert(e3.clone());

        let key3 = e3.key().clone();
        let key0 = e0.key().clone();

        assert_eq!(se.get(&key3).unwrap(), &e3);
        assert_eq!(se.get(&key0).unwrap(), &e0);
    }
}
