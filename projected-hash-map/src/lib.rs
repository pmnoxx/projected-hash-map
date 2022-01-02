use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

///
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
        self.inner.borrow().hash(state);
    }
}

pub struct ProjectedHashMap<T, V>
    where
        V: Borrow<T>,
        T: Hash + PartialEq + Eq,
{
    pd: PhantomData<T>,
    repr: HashSet<HashMapHelper<T, V>>,
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
    /// Gets element
    pub fn get(&self, key: &T) -> Option<&V> {
        self.repr.get(key).map(|v| &v.inner)
    }

    /// The insert inside `HashSet`, will not remove existing element if it has the same key.
    pub fn insert(&mut self, edge: V) {
        self.repr.replace(HashMapHelper { inner: edge, pd: PhantomData });
    }

    /// Checks if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    /// Gets iterator
    pub fn iter(&self) -> impl Iterator<Item = &V> + '_ {
        self.repr.iter().map(|it| &it.inner)
    }

    /// Gets length
    #[must_use]
    pub fn len(&self) -> usize {
        self.repr.len()
    }

    /// Removes element
    pub fn remove(&mut self, key: &T) -> bool {
        self.repr.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct User {
        height: usize,
        name: String,
        weight: usize,
    }

    impl User {
        fn new(name: String) -> Self {
            Self {
                name,
                height: 12,
                weight: 12,
            }
        }
    }

    impl Borrow<String> for User {
        fn borrow(&self) -> &String {
            &self.name
        }
    }

    #[test]
    fn test_hashset() {
        let p1 = "p1".to_string();
        let p2 = "p2".to_string();
        let p3 = "p3".to_string();
        let e0 = User::new(p1.clone());
        let e1 = User::new(p2);
        let e2 = User::new(p3);
        let e3 = User::new(p1);

        let mut se = ProjectedHashMap::default();
        se.insert(e0.clone());
        se.insert(e1);
        se.insert(e2);
        se.insert(e3.clone());

        let key3 = e3.name.clone();
        let key0 = e0.name.clone();

        assert_eq!(se.get(&key3).unwrap(), &e3);
        assert_eq!(se.get(&key0).unwrap(), &e0);
    }

    #[test]
    fn test_remove_key() {
        let p1 = "p1".to_string();
        let p2 = "p2".to_string();
        let e1 = User::new(p1);
        let e2 = User::new(p2);
        let mut se = ProjectedHashMap::default();
        se.insert(e2.clone());

        let key = e1.name.clone();
        se.insert(e1.clone());
        assert_eq!(se.get(&key).unwrap(), &e1);
        se.remove(&e1.name);
        assert_eq!(se.get(&key), None);

        let key2 = e2.name.clone();
        assert_eq!(se.get(&key2).unwrap(), &e2);
    }
}

