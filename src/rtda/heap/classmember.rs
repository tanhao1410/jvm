struct ClassMember<'a> {
    access_flags: u16,
    name: &'a str,
    descriptor: &'a str,
    class: Arc<Class<'a>>,
}

impl<'a> ClassMember<'a> {
    fn new(class: Arc<Class<'a>>, cf_method: &'a MemberInfo) -> ClassMember<'a> {
        let access_flags = cf_method.access_flgs();
        let name = cf_method.name();
        let descriptor = cf_method.descriptor();
        Self { access_flags, name, descriptor, class }
    }

    fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }
    fn is_private(&self) -> bool {
        self.access_flags & ACC_PRIVATE != 0
    }
    fn is_protected(&self) -> bool {
        self.access_flags & ACC_PROTEDTED != 0
    }
    fn is_static(&self) -> bool {
        self.access_flags & ACC_STATIC != 0
    }
    fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    fn name(&self) -> &str {
        self.name
    }
    fn descriptor(&self) -> &str {
        self.descriptor
    }
    fn class(&'a self) -> &'a Class<'a> {
        self.class.borrow()
    }

    fn is_accessible_to(&'a self, d: &'a Class<'a>) -> bool {
        if self.is_public() {
            true
        } else {
            let c = self.class.borrow();
            if self.is_protected() {
                d == c || d.is_sub_class_of(c) || c.package_name() == d.package_name()
            } else if !self.is_private() {
                c.package_name() == d.package_name()
            } else {
                d == c
            }
        }
    }
}