use simd_json::value::owned::{Value as OwnedValue};
use object_builder;

pub trait Serializer {

    fn build(&self, &mut object_builder::ObjectBuilder);

    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, include_root: bool) -> OwnedValue {
        let mut bldr = object_builder::ObjectBuilder::new();
        let root = self.root();
        if include_root && root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(&mut bldr);

        bldr.unwrap()
    }
}

pub trait ObjectSerializer<T> {

    fn build(&self, &T, &mut object_builder::ObjectBuilder);

    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T, include_root: bool) -> OwnedValue {
        let mut bldr = object_builder::ObjectBuilder::new();
        let root = self.root();
        if include_root && root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, &mut bldr);
        bldr.unwrap()
    }
}

pub trait ObjectScopeSerializer<T, S> {

    fn build(&self, &T, &S, &mut object_builder::ObjectBuilder);

    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T, scope: &S, include_root: bool) -> OwnedValue {
        let mut bldr = object_builder::ObjectBuilder::new();
        let root = self.root();
        if include_root && root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, scope, &mut bldr);
        bldr.unwrap()
    }
}
