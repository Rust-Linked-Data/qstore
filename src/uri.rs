use identifiers::InternalUriID;
use store::StorageEngine;
use std::hash::{Hash, Hasher};

static RDFURI_HASH_PREFIX: &'static str = "U:";

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct RDFUri {
    id: InternalUriID
}

impl Hash for RDFUri {
    fn hash<H: Hasher>(&self, state: &mut H) {
        RDFURI_HASH_PREFIX.hash(state);
        self.id.hash(state);
    }
}

impl RDFUri {
    pub fn from_string(store: &mut StorageEngine, uri_string: &str) -> RDFUri {
        let iid = store.uri_to_internal_uri_id(uri_string).unwrap();
        RDFUri {id: iid}
    }
    pub fn from_string_if_exist(store: &StorageEngine, uri_string: &str) -> Result<RDFUri, ()> {
        let iid = match store.uri_to_internal_uri_id_if_exist(uri_string) {
            Ok(i) => i,
            Err(s) => { println!("{}", s); return Result::Err(()) }
        };
        Ok(RDFUri {id: iid})
    }

    pub fn from_iuid(iid: InternalUriID) -> RDFUri {
        RDFUri {id: iid}
    }

    pub fn to_string(&self, store: &StorageEngine) -> String {
        if let Ok(uristr) = store.internal_uri_id_to_uri(&self.id) { uristr }
            else {
            panic!(format!("Cannot extract string representation of RDFUri with internal uri id {:?}.", self.id))
        }
    }
}