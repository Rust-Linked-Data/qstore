extern crate qstore;
use qstore::StorageEngine;
use qstore::identifiers::{InternalID};

fn main() {
    let mut s = StorageEngine::default();
    let t1 = "http://default.org/graph";

    let tid = s.uri_to_internal_ID(t1).unwrap();


    let t2 = "http://default.com/testtwo";
    let t3 = "http://default.com/another/test";
    let t4 = "http://default.com/third#test";
    let r1 = s.uri_to_internal_uri_ID(t1).unwrap();
    let r2 = s.uri_to_internal_uri_ID(t2).unwrap();
    let r3 = s.uri_to_internal_uri_ID(t3).unwrap();
    let r4 = s.uri_to_internal_uri_ID(t4).unwrap();
    let u1 = s.internal_uri_ID_to_uri(r1).unwrap();
    println!("{}", u1);
    let u2 = s.internal_uri_ID_to_uri(r2).unwrap();
    println!("{}", u2);
    let u3 = s.internal_uri_ID_to_uri(r3).unwrap();
    println!("{}", u3);
    let u4 = s.internal_uri_ID_to_uri(r4).unwrap();
    println!("{}", u4);

    //test_me();

    s.add_internal_quad(InternalID(1.into()), InternalID(2.into()), InternalID(3.into()), InternalID(4.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(7.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(8.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(9.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(10.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(11.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(12.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(13.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(14.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(15.into()));
    s.add_internal_quad(InternalID(4.into()), InternalID(5.into()), InternalID(6.into()), InternalID(16.into()));
    s.add_internal_quad(InternalID(99.into()), InternalID(100.into()), InternalID(101.into()), InternalID(102.into()));
    s.add_internal_quad(InternalID(105.into()), InternalID(106.into()), InternalID(107.into()), InternalID(108.into()));
    s.add_internal_quad(InternalID(7.into()), InternalID(8.into()), InternalID(9.into()), InternalID(10.into()));
    s.add_internal_quad(InternalID(10.into()), InternalID(11.into()), InternalID(12.into()), InternalID(13.into()));

    let mut res = s.search_engine_internal(Some(InternalID(99.into())), Some(InternalID(100.into())), Some(InternalID(101.into())), Some(InternalID(102.into())));
    let r1 = res.next();
    println!("{:?}", r1);
    let mut res2 = s.search_engine_internal(Some(InternalID(100.into())), Some(InternalID(100.into())), Some(InternalID(101.into())), Some(InternalID(102.into())));
    let r2 = res2.next();
    println!("{:?}", r2);
    assert!(r1.is_some());
    assert!(r2.is_none());
    let res3 = s.search_engine_internal(Some(InternalID(4.into())), Some(InternalID(5.into())), Some(InternalID(6.into())), None);
    for f in res3 {
        println!("{:?}", f);
    }
}
