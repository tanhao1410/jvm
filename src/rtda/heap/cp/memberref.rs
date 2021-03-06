pub struct MemberRef {
    sym: SymRef,
    name: String,
    descriptor: String,
}

impl MemberRef {
    pub fn new(ref_info: &classfile::ConstantMemberrefInfo, cp: Arc<RwLock<ConstantPool>>) -> MemberRef {
        let class_name = ref_info.class_name();
        let (name, descriptor) = ref_info.name_and_descriptor();
        let sym = SymRef::new(cp, class_name);

        let (name, descriptor) = (name.to_string(), descriptor.to_string());
        Self { sym, name, descriptor }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    fn cp(&self) -> Arc<RwLock<ConstantPool>> {
        self.sym.cp.clone()
    }

    fn resoved_class(&mut self) -> Arc<RwLock<Class>> {
        self.sym.resoved_class()
    }

    fn lookup_method_in_class(class: Arc<RwLock<Class>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for method in &class.read().unwrap().methods {
            if method.name() == name && method.descriptor() == descriptor {
                return Some(method.clone());
            }
        }
        match &class.read().unwrap().super_class {
            Some(c) => Self::lookup_method_in_class(c.clone(), name, descriptor),
            None => None,
        }
    }

    fn lookup_method_in_interfaces(ifaces: &Vec<Arc<RwLock<Class>>>, name: &str, descriptor: &str) -> Option<Arc<Method>> {
        for iface in ifaces {
            for method in &iface.read().unwrap().methods {
                if method.name() == name && method.descriptor() == descriptor {
                    return Some(method.clone());
                }
            }

            let method = Self::lookup_method_in_interfaces(&iface.read().unwrap().interfaces, name, descriptor);
            if method.is_some() {
                return method;
            }
        }

        None
    }
}