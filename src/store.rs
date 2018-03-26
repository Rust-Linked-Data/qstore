
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::iter;
use identifiers::{InternalID, InternalUriID, ThirtyTwoBitID, SixtyFourBitID};
use uri::RDFUri;
use literal::Literal;
use blank::BlankNode;
use indexed_hash_map::{IndexedIDHashMap};
use indexed_quad_set::{test_me, IndexedQuadSet, SearchableIndex, IndexOrder, SPOG, GSPO, POSG, OSPG, SPOGIndex, GSPOIndex, POSGIndex, OSPGIndex};

#[derive(PartialEq, PartialOrd, Clone, Hash)]
pub enum StoreNode {
    URIRef(RDFUri),
    Literal(Literal),
    Blank(BlankNode)
}

impl StoreNode {
}

pub type SubjectID = InternalID;
pub type PredicateID = InternalID;
pub type ObjectID = InternalID;
pub type GraphID = InternalID;
pub type InternalQuad = (SubjectID, PredicateID, ObjectID, GraphID);

type PrefixMap = IndexedIDHashMap<String, ThirtyTwoBitID>;
type SuffixMap = IndexedIDHashMap<String, ThirtyTwoBitID>;
type ObjectMap = IndexedIDHashMap<StoreNode, SixtyFourBitID>;

pub static DEFAULT_GRAPH_URI: &'static str = "http://internal/graph";

pub struct StorageEngine {
    object_map: ObjectMap,
    prefix_map: PrefixMap,
    suffix_map: SuffixMap,
    spog_index: SPOGIndex,
    gspo_index: GSPOIndex,
    posg_index: POSGIndex,
    ospg_index: OSPGIndex
}

impl Default for StorageEngine {
    fn default() -> StorageEngine {
        let mut fresh = StorageEngine {
            object_map: ObjectMap::default(),
            prefix_map: PrefixMap::default(),
            suffix_map: SuffixMap::default(),
            spog_index: IndexedQuadSet::default(),
            gspo_index: IndexedQuadSet::default(),
            posg_index: IndexedQuadSet::default(),
            ospg_index: IndexedQuadSet::default(),

        };
        let default_graph_id = fresh.uri_str_to_internal_id(DEFAULT_GRAPH_URI).unwrap();
        if 0u64 != default_graph_id.0.into() { panic!("Default graph ID should always be 0."); }
        fresh
    }
}


impl StorageEngine {
    pub fn uri_to_internal_uri_id_if_exist(&self, uri: &str) -> Result<InternalUriID, String> {
        let prefix_str;
        let suffix_str;
        let hash_index = uri.rfind('#');
        if hash_index.is_some() {
            let split_str = uri.split_at(hash_index.unwrap()+1);
            prefix_str = split_str.0;
            suffix_str = split_str.1;
        } else {
            let slash_index = uri.rfind('/');
            if slash_index.is_some() {
                let split_str = uri.split_at(slash_index.unwrap()+1);
                prefix_str = split_str.0;
                suffix_str = split_str.1;
            } else {
                return Result::Err(
                    "Uri must contain either a /, or a # to separate prefix from suffix."
                        .to_string());
            }
        }
        let prefix_id: ThirtyTwoBitID = if let Some(pid) = self.prefix_map.get_id_by_key(prefix_str).cloned() { pid }
            else { return Result::Err("That URI prefix does not exist in the store.".to_string()); };
        let suffix_id: ThirtyTwoBitID = if let Some(sid) = self.suffix_map.get_id_by_key(suffix_str).cloned() { sid }
            else { return Result::Err("That URI suffix does not exist in the store.".to_string()); };
        return Ok(InternalUriID(prefix_id, suffix_id))
    }
    pub fn uri_to_internal_uri_id(&mut self, uri: &str) -> Result<InternalUriID, String> {
        let prefix_str;
        let suffix_str;
        let hash_index = uri.rfind('#');
        if hash_index.is_some() {
            let split_str = uri.split_at(hash_index.unwrap()+1);
            prefix_str = split_str.0;
            suffix_str = split_str.1;
        } else {
            let slash_index = uri.rfind('/');
            if slash_index.is_some() {
                let split_str = uri.split_at(slash_index.unwrap()+1);
                prefix_str = split_str.0;
                suffix_str = split_str.1;
            } else {
                return Result::Err(
                    "Uri must contain either a /, or a # to separate prefix from suffix."
                        .to_string());
            }
        }
        let prefix_id: ThirtyTwoBitID = self.prefix_map.get_id_by_key(prefix_str).cloned().unwrap_or_else(||
            self.prefix_map.insert_unchecked(prefix_str.to_owned()).unwrap()
        );

        let suffix_id: ThirtyTwoBitID = self.suffix_map.get_id_by_key(suffix_str).cloned().unwrap_or_else(||
            self.suffix_map.insert_unchecked(suffix_str.to_owned()).unwrap()
        );
        return Ok(InternalUriID(prefix_id, suffix_id))
    }
    pub fn find_internal_id(&self, node: &StoreNode) -> Result<InternalID, ()> {
        let internal_id: SixtyFourBitID = if let Some(i) = self.object_map.get_id_by_key(node).cloned() { i }
            else { return Result::Err(()); };
        return Ok(InternalID(internal_id))
    }
    pub fn find_or_add_internal_id(&mut self, node: StoreNode) -> Result<InternalID, String> {
        let internal_id: SixtyFourBitID = self.object_map.get_id_by_key(&node).cloned().unwrap_or_else(||
            self.object_map.insert_unchecked(node).unwrap()
        );
        return Ok(InternalID(internal_id))
    }
    pub fn uri_str_to_internal_id(&mut self, uri: &str) -> Result<InternalID, String> {
        let internal_uri_id = self.uri_to_internal_uri_id(uri)?;
        let rdfuri = RDFUri::from_iuid(internal_uri_id);
        let wrapped_uri_id = StoreNode::URIRef(rdfuri);
        self.find_or_add_internal_id(wrapped_uri_id)
    }
    pub fn internal_uri_id_to_uri(&self, internal_uri_id: &InternalUriID) -> Result<String, String> {
        let prefix_id: &ThirtyTwoBitID = &internal_uri_id.0;
        let suffix_id: &ThirtyTwoBitID = &internal_uri_id.1;
        let prefix_string = self.prefix_map.get_key_ref_by_id(prefix_id).unwrap();
        let suffix_string = self.suffix_map.get_key_ref_by_id(suffix_id).unwrap();
        let full_string = format!("{}{}", prefix_string, suffix_string);
        Ok(full_string)
    }

    pub fn add_internal_quad(&mut self, graph: GraphID, subject: SubjectID, predicate: PredicateID, object: ObjectID) {
        self.spog_index.add_entry(SPOG::build_from_ref_parts(&subject, &predicate, &object, &graph));
        self.gspo_index.add_entry(GSPO::build_from_ref_parts(&graph, &subject, &predicate, &object));
        self.posg_index.add_entry(POSG::build_from_ref_parts(&predicate, &object, &subject, &graph));
        self.ospg_index.add_entry(OSPG::build_from_ref_parts(&object, &subject, &predicate, &graph));
    }

    pub fn add_internal_triple(&mut self, subject: SubjectID, predicate: PredicateID, object: ObjectID) {
        let graph_id = InternalID(0.into());
        self.add_internal_quad(graph_id, subject, predicate, object)
    }

    pub fn remove_internal_quad(&mut self, graph: GraphID, subject: SubjectID, predicate: PredicateID, object: ObjectID) {
        self.spog_index.remove_entry(&SPOG::build_from_ref_parts(&subject, &predicate, &object, &graph));
        self.gspo_index.remove_entry(&GSPO::build_from_ref_parts(&graph, &subject, &predicate, &object));
        self.posg_index.remove_entry(&POSG::build_from_ref_parts(&predicate, &object, &subject, &graph));
        self.ospg_index.remove_entry(&OSPG::build_from_ref_parts(&object, &subject, &predicate, &graph));
    }

    pub fn remove_internal_triple(&mut self, subject: SubjectID, predicate: PredicateID, object: ObjectID) {
        let graph_id = InternalID(0.into());
        self.remove_internal_quad(graph_id, subject, predicate, object)
    }

    pub fn empty_iter() -> iter::Empty<(GraphID, SubjectID, PredicateID, ObjectID)>{
        iter::empty::<(GraphID, SubjectID, PredicateID, ObjectID)>()
    }

    pub fn lookup_node_by_iid<'a>(&'a self, iid: &InternalID) -> Result<&'a StoreNode, ()> {
        let indexed_id: SixtyFourBitID = iid.clone().into();
        if let Some(n) = self.object_map.get_key_ref_by_id(&indexed_id) {
            Ok(n)
        } else {
            Err(())
        }
    }

    pub fn search_nodes<'a>(&'a self, graph: Option<StoreNode>, subject: Option<StoreNode>, predicate: Option<StoreNode>, object: Option<StoreNode>) -> Result<Box<Iterator<Item=(&'a StoreNode, &'a StoreNode, &'a StoreNode, &'a StoreNode)>+'a>, String>
    {
        let gid = if let Some(g) = graph {
            if let Some(gi) = self.object_map.get_id_by_key(&g).cloned() { Some(InternalID(gi)) } else { return Err("That graph identifier does not exist in the store.".to_string()) }
        } else { None };
        let sid = if let Some(s) = subject {
            if let Some(si) = self.object_map.get_id_by_key(&s).cloned() { Some(InternalID(si)) } else { return Err("That subject identifier does not exist in the store.".to_string()) }
        } else { None };
        let pid = if let Some(p) = predicate {
            if let Some(pi) = self.object_map.get_id_by_key(&p).cloned() { Some(InternalID(pi)) } else { return Err("That predicate identifier does not exist in the store.".to_string()) }
        } else { None };
        let oid = if let Some(o) = object {
            if let Some(oi) = self.object_map.get_id_by_key(&o).cloned() { Some(InternalID(oi)) } else { return Err("That object identifier does not exist in the store.".to_string()) }
        } else { None };
        let internal_results = self.search_engine_internal(gid, sid, pid, oid);
        let node_results = internal_results.map(|res| {
            let (gid,sid,pid,oid) = res;
            let graphnode = self.object_map.get_key_ref_by_id(&gid.into()).unwrap();
            let subjnode = self.object_map.get_key_ref_by_id(&sid.into()).unwrap();
            let prednode = self.object_map.get_key_ref_by_id(&pid.into()).unwrap();
            let objnode = self.object_map.get_key_ref_by_id(&oid.into()).unwrap();
            (graphnode, subjnode, prednode, objnode)
        }).collect::<Vec<(&'a StoreNode, &'a StoreNode, &'a StoreNode, &'a StoreNode)>>();
        return Ok(Box::new(node_results.into_iter()))
    }

    pub fn search_engine_internal<'a>(&'a self, graph: Option<GraphID>, subject: Option<SubjectID>, predicate: Option<PredicateID>, object: Option<ObjectID>)
                                      -> Box<Iterator<Item=(GraphID, SubjectID, PredicateID, ObjectID)>+'a> {
        match (graph, subject, predicate, object) {
            (Some(g), Some(s), Some(p), Some(o)) => {
                if let Some(f) = self.gspo_index.find_exact_match(&g, &s, &p, &o) {
                    Box::new(iter::once((g, s, p, o)))
                } else {
                    Box::new(Self::empty_iter())
                }
            },
            (None, None, None, None) => {
                Box::new(self.gspo_index.full_range().map(|r: &GSPO| { r.clone().deconstruct() }))
            },
            (Some(g), Some(s), Some(p), None) => {
                Box::new(self.gspo_index.find_by_first_three(&g, &s, &p).map(|r: &GSPO| { r.clone().deconstruct() }))
            },
            (Some(g), Some(s), None, None) => {
                Box::new(self.gspo_index.find_by_first_two(&g, &s).map(|r: &GSPO| { r.clone().deconstruct() }))
            },
            (Some(g), None, None, None) => {
                Box::new(self.gspo_index.find_by_first_one(&g).map(|r: &GSPO| { r.clone().deconstruct() }))
            },
            (Some(g), Some(s), None, Some(o)) => {
                let o_copy = o.clone();
                Box::new(self.gspo_index.find_by_first_two(&g, &s)
                    .filter(move |&r| r.object_refs().3.eq(&o_copy))
                    .map(|r: &GSPO|  r.clone().deconstruct() ))
            },
            (Some(g), None, Some(p), Some(o)) => {
                let g_copy = g.clone();
                Box::new(self.posg_index.find_by_first_two(&p, &o)
                    .filter(move |&r| r.object_refs().3.eq(&g_copy))
                    .map(|r: &POSG|  r.clone().deconstruct() ))
            },
            (Some(g), None, Some(p), None) => {
                /* This is assuming there are more different p's in the graph than different g's */
                let g_copy = g.clone();
                Box::new(self.posg_index.find_by_first_one(&p)
                    .filter(move |&r| r.object_refs().3.eq(&g_copy))
                    .map(|r: &POSG|  r.clone().deconstruct() ))
            },
            (Some(g), None, None, Some(o)) => {
                /* This is assuming there are more different o's in the graph than different g's */
                let g_copy = g.clone();
                Box::new(self.ospg_index.find_by_first_one(&o)
                    .filter(move |&r| r.object_refs().3.eq(&g_copy))
                    .map(|r: &OSPG|  r.clone().deconstruct() ))
            },
            (None, Some(s), Some(p), Some(o)) => {
                Box::new(self.spog_index.find_by_first_three(&s, &p, &o).map(|r: &SPOG| { r.clone().deconstruct() }))
            },
            (None, Some(s), Some(p), None) => {
                Box::new(self.spog_index.find_by_first_two(&s, &p).map(|r: &SPOG| { r.clone().deconstruct() }))
            },
            (None, Some(s), None, None) => {
                Box::new(self.spog_index.find_by_first_one(&s).map(|r: &SPOG| { r.clone().deconstruct() }))
            },
            (None, None, Some(p), Some(o)) => {
                Box::new(self.posg_index.find_by_first_two(&p, &o).map(|r: &POSG| { r.clone().deconstruct() }))
            },
            (None, None, Some(p), None) => {
                Box::new(self.posg_index.find_by_first_one(&p).map(|r: &POSG| { r.clone().deconstruct() }))
            },
            (None, Some(s), None, Some(o)) => {
                Box::new(self.ospg_index.find_by_first_two(&o, &s).map(|r: &OSPG| { r.clone().deconstruct() }))
            },
            (None, None, None, Some(o)) => {
                Box::new(self.ospg_index.find_by_first_one(&o).map(|r: &OSPG| { r.clone().deconstruct() }))
            },
            //_ => unimplemented!()
        }
    }
}