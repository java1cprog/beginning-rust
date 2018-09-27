// It does nothing.

fn main() {
    trait Searchable<Key> {
        fn contains(&self, key: Key) -> bool;
    }
    #[allow(dead_code)]
    fn is_present<Collection>(coll: &Collection, id: u32) -> bool
    where
        Collection: Searchable<u32>,
    {
        coll.contains(id)
    }
}
