pub struct Object {}

pub const NULL: Object = Object {};

impl Copy for Object {}

impl Clone for Object {
    fn clone(&self) -> Self {
        *self
    }
}

impl PartialEq for Object {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        (self as *const Self) == (other as *const Self)
    }
}

impl Object {
    fn new(class: &Class) -> Arc<Object> {
        unimplemented!()
    }
}

const OBJECT_CLASS_NAME:&str = "java/lang/Object";