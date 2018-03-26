//#![feature(nll)]
//#![feature(conservative_impl_trait)]
#![feature(try_from)]
#![feature(btree_range)]
#![feature(const_fn)]
#![feature(inclusive_range)]
#![feature(inclusive_range_fields)]



#![feature(proc_macro, specialization)]
//#[cfg(feature = "python")]
#[macro_use]
extern crate pyo3;

extern crate uuid;

pub mod identifiers;
pub mod uri;
pub mod literal;
pub mod blank;
pub mod indexed_hash_map;
pub mod indexed_quad_set;
pub mod store;

//#[cfg(feature = "python")]
pub mod python;




