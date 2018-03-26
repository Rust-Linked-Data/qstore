#![allow(unused_imports)]
#[macro_use]
use pyo3;
use pyo3::Python;
use pyo3::ObjectProtocol;
use pyo3::prelude::*;
use pyo3::PyObject;
//use std::convert::TryFrom;
use store::{StorageEngine, StoreNode};
use identifiers::InternalID;
use uri::RDFUri;
use literal::Literal;
use blank::BlankNode;
use std::borrow::Cow;
use std::iter as stditer;

pub static URI_REF_TYPE_FLAG: u8 = 1;
pub static LITERAL_TYPE_FLAG: u8 = 2;
pub static BLANK_TYPE_FLAG: u8 = 3;
pub static GRAPH_TYPE_FLAG: u8 = 4;
pub static IDENTIFIER_TYPE_FLAG: u8 = 5;

#[derive(Debug)]
enum PyQStoreNodeType {
    URIRef(PyObject),
    Literal(PyObject, Option<PyObject>, Option<PyObject>),
    Blank(PyObject),
}

#[py::class(name=_PyQStoreNode, subclass)]
#[derive(Debug)]
struct PyQStoreNode {
    inner: PyQStoreNodeType,
}

impl PyQStoreNode {
    fn borrow_inner<'a>(&'a self) -> &'a PyQStoreNodeType
    {
        &self.inner
    }

    pub fn to_native_store_node(&self, py: Python, store: &mut StorageEngine) -> StoreNode {
        match self.borrow_inner() {
            &PyQStoreNodeType::URIRef(ref obj) => {
                let s: &str = obj.extract(py).unwrap();
                StoreNode::URIRef(RDFUri::from_string(store, s))

            },
            &PyQStoreNodeType::Literal(ref obj, ref maybe_dataclass_obj, ref maybe_lang_obj) => {
                let s: &str = obj.extract(py).unwrap();
                let d: Option<&str> = if let &Some(ref d_obj) = maybe_dataclass_obj {
                    d_obj.extract(py).unwrap()
                } else { None };
                let l: Option<&str> = if let &Some(ref l_obj) = maybe_lang_obj {
                    l_obj.extract(py).unwrap()
                } else { None };
                StoreNode::Literal(Literal::new(store, s, d, l))
            }
            &PyQStoreNodeType::Blank(ref obj) => {
                let s: &str = obj.extract(py).unwrap();
                StoreNode::Blank(BlankNode::new(store, Some(s)))
            }
        }
    }
    pub fn to_native_store_node_if_exist(&self, py: Python, store: &StorageEngine) -> Result<StoreNode, ()> {
        match self.borrow_inner() {
            &PyQStoreNodeType::URIRef(ref obj) => {
                let s: &str = obj.extract(py).unwrap();
                let found_uri = if let Ok(u) = RDFUri::from_string_if_exist(store, s) { u } else {
                    return Result::Err(());
                };
                Ok(StoreNode::URIRef(found_uri))

            },
            &PyQStoreNodeType::Literal(ref obj, ref maybe_dataclass_obj, ref maybe_lang_obj) => {
                let s: &str = obj.extract(py).unwrap();
                let d: Option<&str> = if let &Some(ref d_obj) = maybe_dataclass_obj {
                    d_obj.extract(py).unwrap()
                } else { None };
                let l: Option<&str> = if let &Some(ref l_obj) = maybe_lang_obj {
                    l_obj.extract(py).unwrap()
                } else { None };
                println!("to_native_store_node_if_exist: Literal: {:?} {:?} {:?}", s, d, l);
                let found_literal = if let Ok(l) = Literal::construct_if_exist(store, s, d, l) { l }
                    else { return Result::Err(()); };
                Ok(StoreNode::Literal(found_literal))
            }
            &PyQStoreNodeType::Blank(ref obj) => {
                let s: &str = obj.extract(py).unwrap();
                let found_bnode = if let Ok(b) = BlankNode::find_by_idenfier_if_exist(store, Some(s)) { b }
                    else { return Result::Err(()); };
                Ok(StoreNode::Blank(found_bnode))
            }
        }
    }
    pub fn create_from_native_store_node_ref(native_store_node: &StoreNode, py: Python, store: &StorageEngine) -> PyQStoreNode {
        match native_store_node {
            &StoreNode::URIRef(ref rdfuri) => {
                let uri_string: Py<PyString> = PyString::new(py,&(rdfuri.to_string(store)));
                PyQStoreNode { inner: PyQStoreNodeType::URIRef(uri_string.into()) }
            },
            &StoreNode::Literal(ref lit) => {
                let lit_string: Py<PyString> = PyString::new(py,lit.borrow_lexical_form());
                let maybe_lang: Option<PyObject> = if let Some(ref lang_string) = lit.borrow_lang() {
                    Some(PyString::new(py, lang_string).into())
                } else { None };
                let maybe_datatype: Option<PyObject> = if maybe_lang.is_some() {
                    None
                } else {
                    Some(PyString::new(py, &lit.borrow_datatype_uri().to_string(store)).into())
                };
                PyQStoreNode { inner: PyQStoreNodeType::Literal(lit_string.into(), maybe_datatype, maybe_lang) }
            },
            &StoreNode::Blank(ref bl) => {
                let bl_string: Py<PyString> = PyString::new(py,bl.lookup_identifier(store));
                PyQStoreNode { inner: PyQStoreNodeType::Blank(bl_string.into()) }
            }
        }
    }
}


#[py::methods]
impl PyQStoreNode {
    #[new]
    pub fn __new__(obj: &PyRawObject, thing: &PyObjectRef, type_flag: u8) -> PyResult<()> {
        obj.init(|token| {
            println!("__new__ type flag {}", type_flag);
            match type_flag {
                1u8 => { /* URIRef Type */
                    let uri_string: &PyString = PyString::try_from(thing).unwrap();
                    println!("URIRef {}", uri_string.to_string_lossy());
                    let owned_pystring: PyObject = uri_string.into_object(token.py());
                    return PyQStoreNode {
                        inner: PyQStoreNodeType::URIRef(owned_pystring)
                    };
                },
                2u8 => { /* Literal Type */
                    let lang: Option<PyObject> = match thing.getattr("_language") {
                        Ok(pyobj_lang) => {
                            if pyobj_lang.is_none() {
                                None
                            } else {
                                let owned_pystring: PyObject = pyobj_lang.into_object(token.py());
                                Some(owned_pystring)
                            }
                        }
                        Err(_) => None
                    };
                    println!("Literal lang = {:?}", lang);
                    let datatype: Option<PyObject> = match thing.getattr("_datatype") {
                        Ok(pyobj_datatype) => {
                            if pyobj_datatype.is_none() {
                                None
                            } else {
                                let owned_pystring: PyObject = pyobj_datatype.into_object(token.py());
                                Some(owned_pystring)
                            }
                        }
                        Err(_) => None
                    };

                    println!("Literal datatype = {:?}", datatype);
                    let val = thing.getattr("_value").unwrap();
                    println!("Literal value = {}", val);
                    let literal_string_obj: PyObject = FromPyObject::extract(thing).unwrap();
                    println!("Literal {:?}", literal_string_obj);
                    let owned_pystring: PyObject = thing.str().unwrap().into_object(token.py());
                    println!("Literal str = {:?}", owned_pystring);
                    return PyQStoreNode {
                        inner: PyQStoreNodeType::Literal(owned_pystring, datatype, lang)
                    };
                },
                3u8 => { /* BNode type */
                    let blank_val: &PyString = thing.cast_as::<PyString>().unwrap();
                    let val_obj = blank_val.into_object(token.py());
                    println!("Blank value = {:?}", val_obj);
                    return PyQStoreNode {
                        inner: PyQStoreNodeType::Blank(val_obj)
                    };
                },
                4u8 => { /* Graph Type */
                    let identifier = thing.getattr("identifier").unwrap();
                    println!("Identifier value = {}", identifier);
                    let id_string = if ! identifier.is_none() {
                        identifier.str().unwrap().into_object(token.py())
                    } else {
                        let thing_val: &PyString = thing.cast_as::<PyString>().unwrap();
                        thing_val.into_object(token.py())
                    };
                    return PyQStoreNode {
                        inner: PyQStoreNodeType::Literal(id_string, None, None)
                    };
                },
                5u8 => { /* Identifier type */
                    let id_string = thing.cast_as::<PyString>().unwrap().into_object(token.py());
                    return PyQStoreNode {
                        inner: PyQStoreNodeType::Literal(id_string, None, None)
                    };
                },
                _ => { unimplemented!("Unknown type flag is not implemented.") }
            }

            println!("__new__ {:?}", thing);
        })
    }

    #[getter]
    pub fn inner_type_flag(&self) -> PyResult<u8> {
        match self.inner {
            PyQStoreNodeType::URIRef(_) => Ok(URI_REF_TYPE_FLAG),
            PyQStoreNodeType::Literal(_,_,_) => Ok(LITERAL_TYPE_FLAG),
            PyQStoreNodeType::Blank(_) => Ok(BLANK_TYPE_FLAG)
        }
    }

    pub fn unpack_as_uriref(&self, py: Python) -> PyResult<&PyString> {
        let inner_string =  if let PyQStoreNodeType::URIRef(ref pyobj) = self.inner {
            pyobj
        } else {
            return Err(PyErr::new::<PyString, String>("Cannot unpack that PyQStoreNode as a uriref!".to_owned()));
        };
        return Ok(inner_string.extract(py)?);
    }

    pub fn unpack_as_bnode(&self, py: Python) -> PyResult<&PyString> {
        let inner_string =  if let PyQStoreNodeType::Blank(ref pyobj) = self.inner {
            pyobj
        } else {
            return Err(PyErr::new::<PyString, String>("Cannot unpack that PyQStoreNode as a bnode!".to_owned()));
        };
        return Ok(inner_string.extract(py)?);
    }

    pub fn unpack_as_literal(&self, py: Python) -> PyResult<(&PyString, Option<PyObject>, Option<PyObject>)> {
        let (inner_string, objdata, objlang) = if let PyQStoreNodeType::Literal(ref pyobjstr, ref pyobjdata, ref pyobjlang) = self.inner {
            (pyobjstr, pyobjdata, pyobjlang)
        } else {
            return Err(PyErr::new::<PyString, String>("Cannot unpack that PyQStoreNode as a Literal!".to_owned()));
        };
        let obj_data_or_none: Option<PyObject> = if let &Some(ref d) = objdata {
            Some(d.to_object(py))
        } else { None };
        let obj_lang_or_none: Option<PyObject> = if let &Some(ref l) = objlang {
            Some(l.to_object(py))
        } else { None };
        return Ok((inner_string.extract(py)?, obj_data_or_none, obj_lang_or_none));
    }

    #[staticmethod]
    pub fn _URIRefTypeFlag() -> PyResult<u8> {
        Ok(URI_REF_TYPE_FLAG)
    }

    #[staticmethod]
    pub fn _LiteralTypeFlag() -> PyResult<u8> {
        Ok(LITERAL_TYPE_FLAG)
    }

    #[staticmethod]
    pub fn _BlankTypeFlag() -> PyResult<u8> {
        Ok(BLANK_TYPE_FLAG)
    }

    #[staticmethod]
    pub fn _GraphTypeFlag() -> PyResult<u8> {
        Ok(GRAPH_TYPE_FLAG)
    }

    #[staticmethod]
    pub fn _IdentifierTypeFlag() -> PyResult<u8> {
        Ok(IDENTIFIER_TYPE_FLAG)
    }
}

impl<'source> FromPyObject<'source> for &'source PyQStoreNode
{
    fn extract(obj: &'source PyObjectRef) -> PyResult<&'source PyQStoreNode>
    {
        Ok(<PyQStoreNode as PyTryFrom>::try_from(obj)?)
    }
}

impl IntoPyObject for PyQStoreNode {
    fn into_object(self, py: Python) -> PyObject {
        Py::new(py, |token| { self }).unwrap().into()
    }
}


//impl<'source> TryFrom<&'source PyObjectRef> for PyQStoreNode
//{
//    type Error = PyErr;
//    fn try_from(obj: &PyObjectRef) -> Result<Self, PyErr>
//    {
//        println!("TryFrom {:?}", obj);
//        let pystring: PyString = PyString{0:obj.str().unwrap().into_object(obj.py())};
//        Ok(PyQStoreNode{inner: PyQStoreNodeType::Literal(pystring, None, None)})
//
//    }
//}

#[py::class]
struct PyQStoreIterableResult {
    inner: Box<Iterator<Item=((PyQStoreNode,PyQStoreNode,PyQStoreNode),PyQStoreNode)>>,
    token: PyToken
}
impl PyQStoreIterableResult {

    pub fn create_with_iter(py: Python, iter: Box<Iterator<Item=((PyQStoreNode,PyQStoreNode,PyQStoreNode),PyQStoreNode)>>) -> Py<Self> {
        Py::new(py, |token| {
            PyQStoreIterableResult { inner: iter, token }
        }).unwrap()
    }
    pub fn node_empty_iter() -> stditer::Empty<((PyQStoreNode,PyQStoreNode,PyQStoreNode),PyQStoreNode)> {
        stditer::empty()
    }
    pub fn py_node_empty_iter(py: Python) -> Py<Self> {
        PyQStoreIterableResult::create_with_iter(py, Box::new(Self::node_empty_iter()))
    }
    fn next(&mut self) -> Option<((PyQStoreNode,PyQStoreNode,PyQStoreNode),PyQStoreNode)> {
        self.inner.next()
    }
}

#[py::methods]
impl PyQStoreIterableResult {

}
#[py::proto]
impl PyIterProtocol for PyQStoreIterableResult {

    fn __iter__(&mut self) -> PyResult<PyObject> {
        Ok(self.into())
    }
    fn __next__(&mut self) -> PyResult<Option<PyObject>> {
        match self.next() {
            Some(n) => {
                let py = self.token.py();
                let ((s_py_node, p_py_node, o_py_node), g_py_node) = n;
                let triple_tup = PyTuple::new(py, &vec!(s_py_node.into_object(py), p_py_node.into_object(py), o_py_node.into_object(py)));
                let quad_tup = PyTuple::new(py, &vec![triple_tup.into_object(py), g_py_node.into_object(py)]);
                Ok(Some(quad_tup.into()))
            },
            None => Ok(None)
        }
    }
}

#[py::class(name=_PyQStore, subclass)]
struct PyQStore {
    _engine: StorageEngine,
    default_graph_combined: bool,
    debug: bool,
    token: PyToken,
}

impl PyQStore {
//    pub fn internal_empty_iter() -> stditer::Empty<(InternalID,InternalID,InternalID,InternalID)> {
//        stditer::empty()
//    }
    pub fn _triples(&self, py: Python, triple: (Option<&PyQStoreNode>, Option<&PyQStoreNode>, Option<&PyQStoreNode>), context: Option<&PyQStoreNode>) -> PyResult<Py<PyQStoreIterableResult>> {
        let (s_py_node, p_py_node, o_py_node) = triple;
        let s_native_node = if let Some(s) = s_py_node {
            if let Ok(s_n) = s.to_native_store_node_if_exist(py, &self._engine) { Some(s_n) }
                else { return Ok(PyQStoreIterableResult::py_node_empty_iter(py)); }
        } else { None };
        let p_native_node = if let Some(p) = p_py_node {
            if let Ok(p_n) = p.to_native_store_node_if_exist(py, &self._engine) { Some(p_n) }
                else { return Ok(PyQStoreIterableResult::py_node_empty_iter(py)); }
        } else { None };
        let o_native_node = if let Some(o) = o_py_node {
            if let Ok(o_n) = o.to_native_store_node_if_exist(py, &self._engine) { Some(o_n) }
                else { return Ok(PyQStoreIterableResult::py_node_empty_iter(py)); }
        } else { None };
        let g_native_node = if let Some(g) = context {
            if let Ok(g_n) = g.to_native_store_node_if_exist(py, &self._engine) { Some(g_n) }
                else { return Ok(PyQStoreIterableResult::py_node_empty_iter(py)); }
        } else { None };
        let result = self._engine.search_nodes(g_native_node, s_native_node, p_native_node, o_native_node);
        if let Ok(res) = result {
            return Ok(PyQStoreIterableResult::create_with_iter(py, Box::new(res.map(|r|{
                let (g_n, s_n, p_n, o_n) = r;
                let py_g_n = PyQStoreNode::create_from_native_store_node_ref(g_n, py, &self._engine);
                let py_s_n = PyQStoreNode::create_from_native_store_node_ref(s_n, py, &self._engine);
                let py_p_n = PyQStoreNode::create_from_native_store_node_ref(p_n, py, &self._engine);
                let py_o_n = PyQStoreNode::create_from_native_store_node_ref(o_n, py, &self._engine);
                ((py_s_n, py_p_n, py_o_n), py_g_n) /* different order here is intentional */
            }).collect::<Vec<((PyQStoreNode,PyQStoreNode,PyQStoreNode),PyQStoreNode)>>().into_iter())));
        } else {
            return Ok(PyQStoreIterableResult::py_node_empty_iter(py));
        }

    }
}

#[py::methods]
impl PyQStore {
    #[new]
    pub fn __new__(obj: &PyRawObject, default_graph_combined: Option<bool>, debug: Option<bool>) -> PyResult<()> {
        let is_default_graph_combined = default_graph_combined.unwrap_or(false);
        let is_debug = debug.unwrap_or(false);
        obj.init(|token| {
            PyQStore {
                _engine: StorageEngine::default(),
                default_graph_combined: is_default_graph_combined,
                debug: is_debug,
                token: token
            }
        })
    }

    pub fn add(&mut self, py: Python, triple: (&PyQStoreNode, &PyQStoreNode, &PyQStoreNode), context: Option<&PyQStoreNode>, quoted: Option<bool>) -> PyResult<()> {
        let is_quoted = quoted.unwrap_or(false);
        assert_ne!(is_quoted, true);
        let (s_py_node, p_py_node, o_py_node) = triple;
        println!("Adding quad : {:?} {:?} {:?} {:?}", context, s_py_node, p_py_node, o_py_node);
        println!("Adding quad inner: {:?} {:?} {:?}", s_py_node.borrow_inner(), p_py_node.borrow_inner(), o_py_node.borrow_inner());
        let s_native_node = s_py_node.to_native_store_node(py, &mut self._engine);
        let p_native_node = p_py_node.to_native_store_node(py, &mut self._engine);
        let o_native_node = o_py_node.to_native_store_node(py, &mut self._engine);
        let sid = self._engine.find_or_add_internal_id(s_native_node).unwrap();
        let pid = self._engine.find_or_add_internal_id(p_native_node).unwrap();
        let oid = self._engine.find_or_add_internal_id(o_native_node).unwrap();

        if let Some(g_py_node) = context {
            let g_native_node = g_py_node.to_native_store_node(py, &mut self._engine);
            let gid = self._engine.find_or_add_internal_id(g_native_node).unwrap();
            self._engine.add_internal_quad(gid, sid, pid, oid);
        } else {
            self._engine.add_internal_triple(sid, pid, oid);
        }
        Ok(())
    }

    pub fn remove(&mut self, py: Python, triple: (Option<&PyQStoreNode>, Option<&PyQStoreNode>, Option<&PyQStoreNode>), context: Option<&PyQStoreNode>) -> PyResult<()> {
        Ok(())
    }

    pub fn triples(&self, py: Python, triple: (Option<&PyQStoreNode>, Option<&PyQStoreNode>, Option<&PyQStoreNode>), context: Option<&PyQStoreNode>) -> PyResult<Py<PyQStoreIterableResult>> {
        self._triples(py, triple, context)
    }


    #[staticmethod]
    pub fn empty_iter(py: Python) -> PyResult<Py<PyQStoreIterableResult>> {
        Ok(PyQStoreIterableResult::py_node_empty_iter(py))
    }

    #[cfg(feature = "context-aware")]
    #[staticmethod]
    pub fn context_aware() -> PyResult<bool> {
        Ok(true)
    }

    #[cfg(not(feature = "context-aware"))]
    #[staticmethod]
    pub fn context_aware() -> PyResult<bool> {
        Ok(false)
    }
    #[staticmethod]
    pub fn graph_aware() -> PyResult<bool> {
        Ok(true)
    }
    #[staticmethod]
    pub fn formula_aware() -> PyResult<bool> {
        Ok(false)
    }
}

// add bindings to the generated python module
// N.B: names: "_qstore" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[py::modinit(_qstore)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyQStore>()?;
    m.add_class::<PyQStoreNode>()?;
//    #[pyfn(m, "sum_as_string")]
//    // pyo3 aware function. All of our python interface could be declared in a separate module.
//    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
//    // Python objects to Rust values; and the Rust return value back into a Python object.
//    fn sum_as_string_py(a:i64, b:i64) -> PyResult<String> {
//        let out = sum_as_string(a, b);
//        Ok(out)
//    }

    Ok(())
}

// logic implemented as a normal rust function
fn sum_as_string(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}
