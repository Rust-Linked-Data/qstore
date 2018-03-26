
use std::hash::{Hash, Hasher};

use uuid::Uuid;
use store::{StoreNode, StorageEngine};
use identifiers::InternalID;
use literal::Literal;

static BLANK_HASH_PREFIX: &'static str = "B:";
static BLANK_NODE_IDENTIFIER_TYPE: &'static str = "http://internal/blank";

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct BlankNode {
    id: InternalID
}

impl Hash for BlankNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        BLANK_HASH_PREFIX.hash(state);
        self.id.0.hash(state)
    }
}

impl BlankNode {
    pub fn new(store: &mut StorageEngine, identifier: Option<&str>) -> BlankNode {
        let lit = if let Some(ref i) = identifier {
            Literal::new(store, i, Some(BLANK_NODE_IDENTIFIER_TYPE), None)
        } else {
            let new_uuid = Uuid::new_v4().to_string();
            Literal::new(store, &new_uuid, Some(BLANK_NODE_IDENTIFIER_TYPE), None)
        };
        let internal_id = store.find_or_add_internal_id(StoreNode::Literal(lit)).unwrap();
        BlankNode { id: internal_id }
    }

    pub fn find_by_idenfier_if_exist(store: &StorageEngine, identifier: Option<&str>) -> Result<BlankNode, ()> {
        let lit = if let Some(ref i) = identifier {
            if let Ok(l) = Literal::construct_if_exist(store, i, Some(BLANK_NODE_IDENTIFIER_TYPE), None) { l }
                else { return Result::Err(()); }
        } else {
            return Result::Err(());
        };
        let internal_id = if let Ok(i) = store.find_internal_id(&StoreNode::Literal(lit)) { i }
            else { return Result::Err(()); };
        Ok(BlankNode { id: internal_id })
    }

    pub fn lookup_identifier<'a>(&self, store: &'a StorageEngine) -> &'a str {
        let maybe_node = store.lookup_node_by_iid(&self.id);
        if let Ok(node) = maybe_node {
            match node {
                &StoreNode::URIRef(_) => { unimplemented!("Blank node with uriref identifier is not yet supported.") },
                &StoreNode::Blank(_) => { panic!("Blank node with BNode identifier is impossible.") },
                &StoreNode::Literal(ref lit) => {
                    lit.borrow_lexical_form()
                },
            }
        } else {
            panic!("Identifier for that blank node does not exist!")
        }
    }
}