Projected hash hash map over hash set.

# Overview
Often when storing objects inside a hashmap, their keys are already contained
inside them. It's more efficient to store every key exactly once..

# Implementation
`ProjectedHashMap` is implemented over `HashSet`.

It's defined for pair `<K, V`>, user of this crate needs to implement
`Borrow<K> for V`.
