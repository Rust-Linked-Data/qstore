use std::hash::{Hash, Hasher};
use store::StorageEngine;
use uri::RDFUri;

pub static STRING_URI: &'static str = "http://www.w3.org/2001/XMLSchema#string";
pub static LANG_STRING_URI: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
static LITERAL_HASH_PREFIX: &'static str = "L:";

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Literal {
    lexical_form: String,
    data_type: RDFUri,
    lang: Option<String>,
}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        LITERAL_HASH_PREFIX.hash(state);
        self.lexical_form.hash(state);
        self.data_type.hash(state);
        if let Some(ref l) = self.lang {
            l.hash(state);
        }
    }
}

impl Literal {
    pub fn new(store: &mut StorageEngine, lexical_form: &str, data_type: Option<&str>, lang: Option<&str>) -> Literal {
        let (determined_data_type, determined_lang): (&str, Option<String>) =
        match (data_type, lang) {
            (None, None) => (&STRING_URI, None),
            (Some(dt), None) => (dt, None),
            (None, Some(l)) => (&LANG_STRING_URI, Some(l.to_owned())),
            (Some(dt), Some(l)) => {
                if !(dt == LANG_STRING_URI) {
                    panic!(format!("data_type must be None or \"{}\" when using lang arg.", LANG_STRING_URI))
                }
                (&LANG_STRING_URI, Some(l.to_owned()))
            }
        };
        let data_type_uri = RDFUri::from_string(store, determined_data_type);
        let l = Literal { lexical_form: lexical_form.to_owned(), data_type: data_type_uri, lang: determined_lang };
        println!("new literal: {:?}",l);
        l
    }

    pub fn construct_if_exist(store: &StorageEngine, lexical_form: &str, data_type: Option<&str>, lang: Option<&str>) -> Result<Literal, ()> {
        let (determined_data_type, determined_lang): (&str, Option<String>) =
            match (data_type, lang) {
                (None, None) => (&STRING_URI, None),
                (Some(dt), None) => (dt, None),
                (None, Some(l)) => (&LANG_STRING_URI, Some(l.to_owned())),
                (Some(dt), Some(l)) => {
                    if !(dt == LANG_STRING_URI) {
                        panic!(format!("data_type must be None or {} when using lang arg.", LANG_STRING_URI))
                    }
                    (&LANG_STRING_URI, Some(l.to_owned()))
                }
            };
        let data_type_uri = if let Ok(u) = RDFUri::from_string_if_exist(store, determined_data_type) { u } else {
            return Result::Err(());
        };
        let l = Literal { lexical_form: lexical_form.to_owned(), data_type: data_type_uri, lang: determined_lang };
        println!("constructed literal: {:?}",l);
        Ok(l)
    }

    pub fn from_string(store: &mut StorageEngine, lexical_form: &str) -> Literal {
        Self::new(store, lexical_form, None, None)
    }

    pub fn from_string_if_exist(store: &StorageEngine, lexical_form: &str) -> Result<Literal, ()> {
        Self::construct_if_exist(store, lexical_form, None, None)
    }


    pub fn with_lang(store: &mut StorageEngine, lexical_form: &str, lang: &str) -> Literal {
        Self::new(store, lexical_form, None, Some(lang))
    }

    pub fn with_lang_if_exist(store: &StorageEngine, lexical_form: &str, lang: &str) -> Result<Literal, ()> {
        Self::construct_if_exist(store, lexical_form, None, Some(lang))
    }

    pub fn parse_raw_literal(store: &mut StorageEngine, raw_literal: &str) -> Literal {
        unimplemented!("raw literal parsing is not yet implemented!")
    }

    pub fn borrow_lexical_form<'a>(&'a self) -> &'a str {
        &self.lexical_form
    }

    pub fn borrow_lang<'a>(&'a self) -> Option<&'a str> {
        if let Some(ref l) = self.lang {
            Some(l)
        } else { None }
    }

    pub fn borrow_datatype_uri<'a>(&'a self) -> &'a RDFUri {
        &self.data_type
    }

}