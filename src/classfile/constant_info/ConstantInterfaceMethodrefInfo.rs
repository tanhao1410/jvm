pub struct ConstantInterfaceMethodrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantInterfaceMethodrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantInterfaceMethodrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader) }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.member.class_name(cp)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        self.member.name_and_descriptor(cp)
    }
}