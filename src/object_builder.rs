use value_trait::*;
use value_trait::Mutable;
use value_trait::Builder;
use simd_json::value::owned::{Value as OwnedValue};

pub type Object = simd_json::value::owned::Object;
use array_builder;

pub struct ObjectBuilder {
    pub object: Object,
    pub null: bool,
    pub skip: bool,
    pub root: Option<String>
}

impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder {
            object: halfbrown::HashMap::new(),
            null: false,
            skip: false,
            root: None
        }
    }

    pub fn from_json(object: OwnedValue) -> Option<ObjectBuilder> {
        let obj = object.as_object().unwrap().to_owned();

        Some(ObjectBuilder {
            object: obj,
            null: false,
            skip: false,
            root: None
        })
    }

    pub fn build<F>(builder: F) -> ObjectBuilder where F: FnOnce(&mut ObjectBuilder) {
        let mut bldr = ObjectBuilder::new();
        builder(&mut bldr);

        bldr
    }

    pub fn null(&mut self) {
        self.null = true;
    }

    pub fn skip(&mut self) {
        self.skip = true;
    }

    pub fn root(&mut self, root: &str) {
        self.root = Some(root.to_string());
    }

    pub fn has_root(&mut self) -> bool {
        self.root.is_some()
    }

    pub fn unwrap(self) -> OwnedValue {
        if self.root.is_some() {
            let mut obj = OwnedValue::object();
            let root = self.root.as_ref().unwrap().to_string();
            let self_json = self.unwrap_internal();
            obj.insert(root, self_json).unwrap();
            obj
        } else {
            self.unwrap_internal()
        }
    }

    #[inline]
    fn unwrap_internal(self) -> OwnedValue {
        if self.null {
            OwnedValue::object()
        } else {
            OwnedValue::from(self.object)
        }
    }
}

impl ObjectBuilder {
    pub fn set<N: Into<String>>(&mut self, name: N, value: OwnedValue) {
        self.set_json(name, value);
    }

    pub fn call<N: Into<String>>(&mut self, name: N, value: OwnedValue) {
        self.set(name, value);
    }
}

impl ObjectBuilder {
    pub fn set_json<N: Into<String>>(&mut self, name: N, value: OwnedValue) {
        self.object.insert(name.into(), value);
    }

    pub fn array<N: Into<String>, F>(&mut self, name: N, builder: F) where F: FnOnce(&mut array_builder::ArrayBuilder) {
        self.set(name.into(), array_builder::ArrayBuilder::build(builder).unwrap());
    }

    pub fn object<N: Into<String>, F>(&mut self, name: N, builder: F) where F: FnOnce(&mut ObjectBuilder) {
        self.set(name.into(), ObjectBuilder::build(builder).unwrap());
    }
}
