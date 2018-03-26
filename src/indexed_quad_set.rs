use std::ops::{RangeInclusive, Range};
use std::marker::PhantomData;
use std::collections::BTreeSet;
use std::collections::btree_set::Range as BTreeSetRange;
use std::borrow::Borrow;

use identifiers::{InternalID};
use store::{InternalQuad, SubjectID, PredicateID, ObjectID, GraphID};

pub trait IndexOrder<A, B, C, D>: Eq+Ord+Clone+Sized {
    fn make_full_range() -> RangeInclusive<Self>;
    fn make_one_part_range(part1: &A) -> RangeInclusive<Self>;
    fn make_two_part_range(part1: &A, part2: &B) -> RangeInclusive<Self>;
    fn make_three_part_range(part1: &A, part2: &B, part3: &C) -> RangeInclusive<Self>;
    fn make_four_part_range(part1: &A, part2: &B, part3: &C, part4: &D) -> RangeInclusive<Self>;
    fn build_from_ref_parts(part1: &A, part2: &B, part3: &C, part4: &D) -> Self;
    fn deconstruct(self) -> (GraphID, SubjectID, PredicateID, ObjectID);
    fn object_refs<'a>(&'a self) -> (&GraphID, &SubjectID, &PredicateID, &ObjectID);
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct SPOG (SubjectID, PredicateID, ObjectID, GraphID);
impl IndexOrder<SubjectID, PredicateID, ObjectID, GraphID> for SPOG {
    fn make_full_range() -> RangeInclusive<SPOG> {
        let min_spog = SPOG(SubjectID::MIN, PredicateID::MIN, ObjectID::MIN, GraphID::MIN);
        let max_spog = SPOG(SubjectID::MAX, PredicateID::MAX, ObjectID::MAX, GraphID::MAX);
        RangeInclusive { start: min_spog, end: max_spog }
    }
    fn make_one_part_range(part1: &SubjectID) -> RangeInclusive<SPOG> {
        let min_spog = SPOG(part1.clone(), PredicateID::MIN, ObjectID::MIN, GraphID::MIN);
        let max_spog = SPOG(part1.clone(), PredicateID::MAX, ObjectID::MAX, GraphID::MAX);
        RangeInclusive { start: min_spog, end: max_spog }
    }
    fn make_two_part_range(part1: &SubjectID, part2: &ObjectID) -> RangeInclusive<SPOG> {
        let min_spog = SPOG(part1.clone(), part2.clone(), ObjectID::MIN, GraphID::MIN);
        let max_spog = SPOG(part1.clone(), part2.clone(), ObjectID::MAX, GraphID::MAX);
        RangeInclusive { start: min_spog, end: max_spog }
    }
    fn make_three_part_range(part1: &SubjectID, part2: &ObjectID, part3: &PredicateID) -> RangeInclusive<SPOG> {
        let min_spog = SPOG(part1.clone(), part2.clone(), part3.clone(), GraphID::MIN);
        let max_spog = SPOG(part1.clone(), part2.clone(), part3.clone(), GraphID::MAX);
        RangeInclusive { start: min_spog, end: max_spog }
    }
    fn make_four_part_range(part1: &SubjectID, part2: &ObjectID, part3: &PredicateID, part4: &GraphID) -> RangeInclusive<SPOG> {
        let min_spog = SPOG(part1.clone(), part2.clone(), part3.clone(), part4.clone());
        let max_spog = min_spog.clone();
        RangeInclusive { start: min_spog, end: max_spog }
    }
    fn build_from_ref_parts(part1: &SubjectID, part2: &ObjectID, part3: &PredicateID, part4: &GraphID) -> SPOG {
        SPOG(part1.clone(), part2.clone(), part3.clone(), part4.clone())
    }
    fn object_refs<'a>(&'a self) -> (&GraphID, &SubjectID, &PredicateID, &ObjectID) {
        (&self.3, &self.0, &self.1, &self.2)
    }
    fn deconstruct(self) -> (GraphID, SubjectID, PredicateID, ObjectID) {
        (self.3, self.0, self.1, self.2)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct GSPO (GraphID, SubjectID, PredicateID, ObjectID);
impl IndexOrder<GraphID, SubjectID, PredicateID, ObjectID> for GSPO {
    fn make_full_range() -> RangeInclusive<GSPO> {
        let min = GSPO(GraphID::MIN, SubjectID::MIN, PredicateID::MIN, ObjectID::MIN);
        let max = GSPO(GraphID::MAX, SubjectID::MAX, PredicateID::MAX, ObjectID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_one_part_range(part1: &GraphID) -> RangeInclusive<GSPO> {
        let min = GSPO(part1.clone(), SubjectID::MIN, PredicateID::MIN, ObjectID::MIN);
        let max = GSPO(part1.clone(), SubjectID::MAX, PredicateID::MAX, ObjectID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_two_part_range(part1: &GraphID, part2: &SubjectID) -> RangeInclusive<GSPO> {
        let min = GSPO(part1.clone(), part2.clone(), PredicateID::MIN, ObjectID::MIN);
        let max = GSPO(part1.clone(), part2.clone(), PredicateID::MAX, ObjectID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_three_part_range(part1: &GraphID, part2: &SubjectID, part3: &PredicateID) -> RangeInclusive<GSPO> {
        let min = GSPO(part1.clone(), part2.clone(), part3.clone(), ObjectID::MIN);
        let max = GSPO(part1.clone(), part2.clone(), part3.clone(), ObjectID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_four_part_range(part1: &GraphID, part2: &SubjectID, part3: &PredicateID, part4: &ObjectID) -> RangeInclusive<GSPO> {
        let min = GSPO(part1.clone(), part2.clone(), part3.clone(), part4.clone());
        let max = min.clone();
        RangeInclusive { start: min, end: max }
    }
    fn build_from_ref_parts(part1: &GraphID, part2: &SubjectID, part3: &PredicateID, part4: &ObjectID) -> GSPO {
        GSPO(part1.clone(), part2.clone(), part3.clone(), part4.clone())
    }
    fn object_refs<'a>(&'a self) -> (&GraphID, &SubjectID, &PredicateID, &ObjectID) {
        (&self.0, &self.1, &self.2, &self.3)
    }
    fn deconstruct(self) -> (GraphID, SubjectID, PredicateID, ObjectID) {
        (self.0, self.1, self.2, self.3)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct POSG (PredicateID, ObjectID, SubjectID, GraphID);
impl IndexOrder<PredicateID, ObjectID, SubjectID, GraphID> for POSG {
    fn make_full_range() -> RangeInclusive<POSG> {
        let min = POSG(PredicateID::MIN, ObjectID::MIN, SubjectID::MIN, GraphID::MIN);
        let max = POSG(PredicateID::MAX, ObjectID::MAX, SubjectID::MAX, GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_one_part_range(part1: &PredicateID) -> RangeInclusive<POSG> {
        let min = POSG(part1.clone(), ObjectID::MIN, SubjectID::MIN, GraphID::MIN);
        let max = POSG(part1.clone(), ObjectID::MAX, SubjectID::MAX, GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_two_part_range(part1: &PredicateID, part2: &ObjectID) -> RangeInclusive<POSG> {
        let min = POSG(part1.clone(), part2.clone(), SubjectID::MIN, GraphID::MIN);
        let max = POSG(part1.clone(), part2.clone(), SubjectID::MAX, GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_three_part_range(part1: &PredicateID, part2: &ObjectID, part3: &SubjectID) -> RangeInclusive<POSG> {
        let min = POSG(part1.clone(), part2.clone(), part3.clone(), GraphID::MIN);
        let max = POSG(part1.clone(), part2.clone(), part3.clone(), GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_four_part_range(part1: &PredicateID, part2: &ObjectID, part3: &SubjectID, part4: &GraphID) -> RangeInclusive<POSG> {
        let min = POSG(part1.clone(), part2.clone(), part3.clone(), part4.clone());
        let max = min.clone();
        RangeInclusive { start: min, end: max }
    }
    fn build_from_ref_parts(part1: &PredicateID, part2: &ObjectID, part3: &SubjectID, part4: &GraphID) -> POSG {
        POSG(part1.clone(), part2.clone(), part3.clone(), part4.clone())
    }
    fn deconstruct(self) -> (GraphID, SubjectID, PredicateID, ObjectID) {
        (self.3, self.2, self.0, self.1)
    }
    fn object_refs<'a>(&'a self) -> (&GraphID, &SubjectID, &PredicateID, &ObjectID) {
        (&self.3, &self.2, &self.0, &self.1)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct OSPG (ObjectID, SubjectID, PredicateID, GraphID);
impl IndexOrder<ObjectID, SubjectID, PredicateID, GraphID> for OSPG {
    fn make_full_range() -> RangeInclusive<OSPG> {
        let min = OSPG(ObjectID::MIN, SubjectID::MIN, PredicateID::MIN, GraphID::MIN);
        let max = OSPG(ObjectID::MAX, SubjectID::MAX, PredicateID::MAX, GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_one_part_range(part1: &ObjectID) -> RangeInclusive<OSPG> {
        let min = OSPG(part1.clone(), SubjectID::MIN, PredicateID::MIN, GraphID::MIN);
        let max = OSPG(part1.clone(), SubjectID::MAX, PredicateID::MAX, GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_two_part_range(part1: &ObjectID, part2: &SubjectID) -> RangeInclusive<OSPG> {
        let min = OSPG(part1.clone(), part2.clone(), PredicateID::MIN, GraphID::MIN);
        let max = OSPG(part1.clone(), part2.clone(), PredicateID::MAX, GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_three_part_range(part1: &ObjectID, part2: &SubjectID, part3: &PredicateID) -> RangeInclusive<OSPG> {
        let min = OSPG(part1.clone(), part2.clone(), part3.clone(), GraphID::MIN);
        let max = OSPG(part1.clone(), part2.clone(), part3.clone(), GraphID::MAX);
        RangeInclusive { start: min, end: max }
    }
    fn make_four_part_range(part1: &ObjectID, part2: &SubjectID, part3: &PredicateID, part4: &GraphID) -> RangeInclusive<OSPG> {
        let min = OSPG(part1.clone(), part2.clone(), part3.clone(), part4.clone());
        let max = min.clone();
        RangeInclusive { start: min, end: max }
    }
    fn build_from_ref_parts(part1: &ObjectID, part2: &SubjectID, part3: &PredicateID, part4: &GraphID) -> OSPG {
        OSPG(part1.clone(), part2.clone(), part3.clone(), part4.clone())
    }
    fn object_refs<'a>(&'a self) -> (&GraphID, &SubjectID, &PredicateID, &ObjectID) {
        (&self.3, &self.1, &self.2, &self.0)
    }
    fn deconstruct(self) -> (GraphID, SubjectID, PredicateID, ObjectID) {
        (self.3, self.1, self.2, self.0)
    }
}


pub trait SearchableIndex<A, B, C, D, Q: IndexOrder<A, B, C, D>> {
    fn find_exact_struct(&self, struct_param: Q) -> Option<Q>;
    fn find_exact_match(&self, param1: &A, param2: &B, param3: &C, param4: &D) -> Option<Q>;
    fn full_range(&self) -> BTreeSetRange<Q>;
    fn find_by_first_one(&self, param1: &A) -> BTreeSetRange<Q>;
    fn find_by_first_two(&self, param1: &A, param2: &B) -> BTreeSetRange<Q>;
    fn find_by_first_three(&self, param1: &A, param2: &B, param3: &C) -> BTreeSetRange<Q>;
}

pub struct IndexedQuadSet<A, B, C, D, Q: IndexOrder<A, B, C, D>> {
    inner_map: BTreeSet<Q>,
    first_part: PhantomData<A>,
    second_part: PhantomData<B>,
    third_part: PhantomData<C>,
    fourth_part: PhantomData<D>

}
impl<A, B, C, D, Q: IndexOrder<A, B, C, D>> Default for IndexedQuadSet<A, B, C, D, Q> {
    fn default() -> IndexedQuadSet<A, B, C, D, Q> {
        IndexedQuadSet {
            inner_map: BTreeSet::new(),
            first_part: PhantomData,
            second_part: PhantomData,
            third_part: PhantomData,
            fourth_part: PhantomData
        }
    }
}


impl<A, B, C, D, Q: IndexOrder<A, B, C, D>> IndexedQuadSet<A, B, C, D, Q> {
    pub fn add_entry(&mut self, entry: Q) -> bool {
        self.inner_map.insert(entry)
    }
    pub fn remove_entry(&mut self, entry: &Q) -> bool {
        self.inner_map.remove(entry)
    }
}

impl<A, B, C, D, Q:IndexOrder<A, B, C, D>> SearchableIndex<A, B, C, D, Q> for IndexedQuadSet<A, B, C, D, Q> {
    fn find_exact_struct(&self, struct_param: Q)  -> Option<Q> {
        let range: RangeInclusive<Q> = RangeInclusive{ start: struct_param.clone(), end: struct_param.clone() };
        return if let Some(s) = self.inner_map.range(range).next() {
            Some(struct_param)
        } else { None }
    }
    fn find_exact_match(&self, param1: &A, param2: &B, param3: &C, param4: &D) -> Option<Q> {
        let index_struct = Q::build_from_ref_parts(param1, param2, param3, param4);
        self.find_exact_struct(index_struct)
    }
    fn full_range(&self) -> BTreeSetRange<Q> {
        self.inner_map.range(Q::make_full_range())
    }
    fn find_by_first_one(&self, param1: &A) -> BTreeSetRange<Q> {
        self.inner_map.range(Q::make_one_part_range(param1))
    }
    fn find_by_first_two(&self, param1: &A, param2: &B) -> BTreeSetRange<Q> {
        self.inner_map.range(Q::make_two_part_range(param1, param2))
    }
    fn find_by_first_three(&self, param1: &A, param2: &B, param3: &C) -> BTreeSetRange<Q> {
        self.inner_map.range(Q::make_three_part_range(param1, param2, param3))
    }
}


pub type SPOGIndex = IndexedQuadSet<SubjectID, PredicateID, ObjectID, GraphID, SPOG>;
pub type GSPOIndex = IndexedQuadSet<GraphID, SubjectID, PredicateID, ObjectID, GSPO>;
pub type POSGIndex = IndexedQuadSet<PredicateID, ObjectID, SubjectID, GraphID, POSG>;
pub type OSPGIndex = IndexedQuadSet<ObjectID, SubjectID, PredicateID, GraphID, OSPG>;

pub fn test_me() -> () {
    let mut SPOGSet = SPOGIndex::default();

    SPOGSet.add_entry(SPOG(InternalID(1.into()), InternalID(2.into()), InternalID(3.into()), InternalID(4.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(7.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(8.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(9.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(10.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(11.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(12.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(13.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(14.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(15.into())));
    SPOGSet.add_entry(SPOG(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(16.into())));
    SPOGSet.add_entry(SPOG(InternalID(99.into()), InternalID(100.into()), InternalID(101.into()), InternalID(102.into())));
    SPOGSet.add_entry(SPOG(InternalID(105.into()), InternalID(106.into()), InternalID(107.into()), InternalID(108.into())));
    SPOGSet.add_entry(SPOG(InternalID(7.into()), InternalID(8.into()), InternalID(9.into()), InternalID(10.into())));
    SPOGSet.add_entry(SPOG(InternalID(10.into()), InternalID(11.into()), InternalID(12.into()), InternalID(13.into())));

    let res = SPOGSet.find_exact_match(&InternalID(99.into()), &InternalID(100.into()), &InternalID(101.into()), &InternalID(102.into()));
    println!("{:?}", res);
    let res2 = SPOGSet.find_exact_match(&InternalID(100.into()), &InternalID(100.into()), &InternalID(101.into()), &InternalID(102.into()));
    println!("{:?}", res2);
    assert!(res.is_some());
    assert!(res2.is_none());
    let res3 = SPOGSet.find_by_first_three(&InternalID(4.into()), &InternalID(5.into()), &InternalID(6.into()));
    for f in res3 {
        println!("{:?}", f);
    }

}
