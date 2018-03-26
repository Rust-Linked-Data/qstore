use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::BTreeMap;
use std::borrow::Borrow;
use std::ops::Deref;

pub type HashResult = u64;

use identifiers::IndexedID;

pub fn make_hash<Q: ?Sized+Hash>(o: &Q) -> HashResult {
    let mut hasher = DefaultHasher::default();
    o.hash(&mut hasher);
    hasher.finish()
}


pub struct IndexedIDHashMap<K, V> {
    inner_map: BTreeMap<HashResult, V>,
    inner_index: Vec<K>,
    reuse_pool: Vec<V>,
}
impl<K, V> Default for IndexedIDHashMap<K, V> {
    fn default() -> IndexedIDHashMap<K, V> {
        IndexedIDHashMap {
            inner_map: BTreeMap::new(),
            inner_index: Vec::new(),
            reuse_pool: Vec::new(),
        }
    }
}

impl<K, V: IndexedID> IndexedIDHashMap<K, V> {
    #[inline]
    pub fn get_id_by_key<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where K: Borrow<Q>, Q: Hash {
        let key_hash = make_hash(key);
        self.inner_map.get(&key_hash)
    }

    fn remove_by_key_hash(&mut self, hash: HashResult) -> Result<(), String> {
        let remove_result = self.inner_map.remove(&hash);
        if let Some(previous_id) = remove_result {
            //self.inner_index[previous_id as usize] = 0;
            self.reuse_pool.push(previous_id);
            Ok(())
        } else {
            Err("That key is not stored in this hash map.".to_string())
        }
    }

    #[inline]
    pub fn remove_by_key<Q: ?Sized>(&mut self, key: &Q) -> Result<(), String>
        where K: Borrow<Q>, Q: Hash {
        let key_hash = make_hash(key);
        self.remove_by_key_hash(key_hash)
    }

    #[inline]
    pub fn get_key_ref_by_id(&self, id: &V) -> Option<&K> {
        let id_copy: V = id.clone();
        self.inner_index.get(Into::<usize>::into(id_copy))
    }

}


impl <K: Hash, V: IndexedID> IndexedIDHashMap<K, V> {

    pub fn remove_by_id(&mut self, id: &V) -> Result<(),String> {
        let key_hash = {
            let maybe_key_ref = self.get_key_ref_by_id(id.into());
            if let Some(key_ref) = maybe_key_ref {
                make_hash(key_ref)
            } else {
                return Err("Cannot find a key with that ID.".to_string())
            }
        };
        self.remove_by_key_hash(key_hash)
    }

    pub fn insert_unchecked(&mut self, key: K) -> Result<V, String> {
        let maybe_reuse = self.reuse_pool.pop();
        let id: V;
        let key_hash = make_hash(&key);
        if let Some(reuse_id) = maybe_reuse {
            id = reuse_id as V;
            let index_position: usize = id.clone().into();
            self.inner_index[index_position] = key;
            self.inner_map.insert(key_hash, id.clone());
        } else {
            let next_id = self.inner_index.len();
            if next_id > V::MAX {
                return Err("Overflow. Cannot store more than that many elements.".to_string());
            }
            id = next_id.into();
            self.inner_index.push(key);
            self.inner_map.insert(key_hash, id.clone());
        }
        Ok(id)
    }
}
