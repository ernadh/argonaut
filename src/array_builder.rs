use value_trait::Mutable;
use value_trait::Builder;
use simd_json::value::owned::{Value as OwnedValue};

pub type JsonArray = Vec<OwnedValue>;

use object_builder;

pub struct ArrayBuilder {
    pub array: JsonArray,
    pub null: bool,
    pub skip: bool,
    pub root: Option<String>
}

impl ArrayBuilder {

    pub fn new() -> ArrayBuilder {
        ArrayBuilder {
            array: vec![],
            null: false,
            skip: false,
            root: None
        }
    }

    pub fn from_json(array: OwnedValue) -> Option<ArrayBuilder> {
        match array {
            OwnedValue::Array(array) => Some(ArrayBuilder {
                array: array,
                null: false,
                skip: false,
                root: None
            }),
            _ => None
        }
    }

    pub fn build<F>(builder: F) -> ArrayBuilder where F: FnOnce(&mut ArrayBuilder) {
        let mut bldr = ArrayBuilder::new();
        builder(&mut bldr);

        bldr
    }

    pub fn push_json(&mut self, value: OwnedValue) {
        self.array.push(value);
    }

    pub fn array<F>(&mut self, builder: F) where F: FnOnce(&mut ArrayBuilder) {
        self.push(ArrayBuilder::build(builder).unwrap());
    }

    pub fn object<F>(&mut self, builder: F) where F: FnOnce(&mut object_builder::ObjectBuilder) {
        self.push(object_builder::ObjectBuilder::build(builder).unwrap());
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
            //OwnedValue::Null
            // FIXME: ???
            OwnedValue::object()
        } else {
            OwnedValue::Array(self.array)
        }
    }
}

impl ArrayBuilder {
    pub fn push(&mut self, value: OwnedValue)
    {
        self.push_json(OwnedValue::from(value));
    }
}

impl ArrayBuilder {

    pub fn objects<A, T: Iterator<Item=A>, F>(&mut self, iter: T, func: F) where F: Fn(A, &mut object_builder::ObjectBuilder) {
        for a in iter {
            let mut bldr = object_builder::ObjectBuilder::new();
            func(a, &mut bldr);
            if !bldr.skip {
                self.push(bldr.unwrap())
            }
        }
    }

    pub fn arrays<A, T: Iterator<Item=A>, F>(&mut self, iter: T, func: F) where F: Fn(A, &mut ArrayBuilder) {
        for a in iter {
            let mut bldr = ArrayBuilder::new();
            func(a, &mut bldr);
            if !bldr.skip {
                self.push(bldr.unwrap())
            }
        }
    }

    pub fn map<A, T: Iterator<Item=A>, F>(&mut self, iter: T, func: F) where F: Fn(A) -> OwnedValue {
        for a in iter {
            self.push(func(a))
        }
    }
}
