const CONSTANT_CLASS______________: u8 = 7;
const CONSTANT_FIELDREF___________: u8 = 9;
const CONSTANT_METHODREF__________: u8 = 10;
const CONSTANT_INTERFACE_METHODREF: u8 = 11;
const CONSTANT_STRING_____________: u8 = 8;
const CONSTANT_INTGER_____________: u8 = 3;
const CONSTANT_FLOAT______________: u8 = 4;
const CONSTANT_LONG_______________: u8 = 5;
const CONSTANT_DOUBLE_____________: u8 = 6;
const CONSTANT_NAME_AND_TYPE______: u8 = 12;
const CONSTANT_UTF8_______________: u8 = 1;
const CONSTANT_METHOD_HANDLE______: u8 = 15;
const CONSTANT_METHOD_TYPE________: u8 = 16;
const CONSTANT_INOVKE_DYNAMIC_____: u8 = 18;

fn read_constant_info(reader: &mut ClassReader) -> ConstantInfo {
    let tag = reader.read_u8();
    new_constant_info(tag, reader)
}

fn new_constant_info(tag: u8, reader: &mut ClassReader) -> ConstantInfo {
    match tag {
        CONSTANT_UTF8_______________ => ConstantInfo::Utf8(tag, ConstantUtf8Info::new(reader)),
        CONSTANT_INTGER_____________ => ConstantInfo::Integer(tag, ConstantIntegerInfo::new(reader)),
        CONSTANT_FLOAT______________ => ConstantInfo::Float(tag, ConstantFloatInfo::new(reader)),
        CONSTANT_LONG_______________ => ConstantInfo::Long(tag, ConstantLongInfo::new(reader)),
        CONSTANT_DOUBLE_____________ => ConstantInfo::Double(tag, ConstantDoubleInfo::new(reader)),
        CONSTANT_CLASS______________ => ConstantInfo::Class(tag, ConstantClassInfo::new(reader)),
        CONSTANT_STRING_____________ => ConstantInfo::String(tag, ConstantStringInfo::new(reader)),
        CONSTANT_FIELDREF___________ => ConstantInfo::FieldRef(tag, ConstantFieldrefInfo::new(reader)),
        CONSTANT_METHODREF__________ => ConstantInfo::MethodRef(tag, ConstantMethodrefInfo::new(reader)),
        CONSTANT_INTERFACE_METHODREF => ConstantInfo::InterfaceMethodRef(tag, ConstantInterfaceMethodrefInfo::new(reader)),
        CONSTANT_NAME_AND_TYPE______ => ConstantInfo::NameAndType(tag, ConstantNameAndTypeInfo::new(reader)),
        CONSTANT_METHOD_HANDLE______ => ConstantInfo::MethodHandle(tag, ConstantMethodHandleInfo::new(reader)),
        CONSTANT_METHOD_TYPE________ => ConstantInfo::MethodType(tag, ConstantMethodTypeInfo::new(reader)),
        CONSTANT_INOVKE_DYNAMIC_____ => ConstantInfo::InvokeDynamic(tag, ConstantInvokeDynamicInfo::new(reader)),
        _ => panic!("java. lang. ClassFormatError: constant pool tag!"),
    }
}

enum ConstantInfo {
    Empty,
    Utf8(u8, ConstantUtf8Info),
    Integer(u8, ConstantIntegerInfo),
    Float(u8, ConstantFloatInfo),
    Long(u8, ConstantLongInfo),
    Double(u8, ConstantDoubleInfo),
    Class(u8, ConstantClassInfo),
    String(u8, ConstantStringInfo),
    FieldRef(u8, ConstantFieldrefInfo),
    MethodRef(u8, ConstantMethodrefInfo),
    InterfaceMethodRef(u8, ConstantInterfaceMethodrefInfo),
    NameAndType(u8, ConstantNameAndTypeInfo),
    MethodHandle(u8, ConstantMethodHandleInfo),
    MethodType(u8, ConstantMethodTypeInfo),
    InvokeDynamic(u8, ConstantInvokeDynamicInfo),
}


struct ConstantIntegerInfo {
    integer32: i32,
}

impl ConstantIntegerInfo {
    fn new(reader: &mut ClassReader) -> ConstantIntegerInfo {
        Self { integer32: reader.read_u32() as i32 }
    }
}

struct ConstantFloatInfo {
    float32: f32,
}

impl ConstantFloatInfo {
    fn new(reader: &mut ClassReader) -> ConstantFloatInfo {
        Self { float32: f32::from_bits(reader.read_u32()) }
    }
}

struct ConstantUtf8Info {
    string: String,
}

impl ConstantUtf8Info {
    fn new(reader: &mut ClassReader) -> ConstantUtf8Info {
        let length = reader.read_u16();
        let bytes = reader.read_bytes(length as usize);
        let string = String::from_utf8(bytes).unwrap();
        Self { string }
    }
}

struct ConstantLongInfo {
    integer64: i64,
}

impl ConstantLongInfo {
    fn new(reader: &mut ClassReader) -> ConstantLongInfo {
        Self { integer64: reader.read_u64() as i64 }
    }
}

struct ConstantDoubleInfo {
    float64: f64,
}

impl ConstantDoubleInfo {
    fn new(reader: &mut ClassReader) -> ConstantDoubleInfo {
        Self { float64: f64::from_bits(reader.read_u64()) }
    }
}

struct ConstantClassInfo {
    string_info: ConstantStringInfo,
}

impl ConstantClassInfo {
    fn new(reader: &mut ClassReader) -> ConstantClassInfo {
        Self { string_info: ConstantStringInfo::new(reader) }
    }
    fn name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.string_info.string(cp)
    }
}

struct ConstantStringInfo {
    string_index: u16,
}

impl ConstantStringInfo {
    fn new(reader: &mut ClassReader) -> ConstantStringInfo {
        let string_index = reader.read_u16();
        Self { string_index }
    }
    fn string<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.get_utf8(self.string_index)
    }
}

struct ConstantMemberrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

impl ConstantMemberrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantMemberrefInfo {
        let class_index = reader.read_u16();
        let name_and_type_index = reader.read_u16();
        Self { class_index, name_and_type_index }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        cp.class_name(self.class_index)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        cp.name_and_type(self.name_and_type_index)
    }
}

struct ConstantFieldrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantFieldrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantFieldrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader) }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.member.class_name(cp)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        self.member.name_and_descriptor(cp)
    }
}

struct ConstantMethodrefInfo {
    member: ConstantMemberrefInfo,
}

impl ConstantMethodrefInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodrefInfo {
        Self { member: ConstantMemberrefInfo::new(reader) }
    }
    fn class_name<'a>(&'a self, cp: &'a ConstantPool) -> &'a str {
        self.member.class_name(cp)
    }
    fn name_and_descriptor<'a>(&'a self, cp: &'a ConstantPool) -> (&'a str, &'a str) {
        self.member.name_and_descriptor(cp)
    }
}

struct ConstantInterfaceMethodrefInfo {
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

struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantNameAndTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantNameAndTypeInfo {
        let name_index = reader.read_u16();
        let descriptor_index = reader.read_u16();
        Self { name_index, descriptor_index }
    }
}

struct ConstantMethodHandleInfo {}

impl ConstantMethodHandleInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodHandleInfo {
        unimplemented!()
    }
}

struct ConstantMethodTypeInfo {}

impl ConstantMethodTypeInfo {
    fn new(reader: &mut ClassReader) -> ConstantMethodTypeInfo {
        unimplemented!()
    }
}

struct ConstantInvokeDynamicInfo {}

impl ConstantInvokeDynamicInfo {
    fn new(reader: &mut ClassReader) -> ConstantInvokeDynamicInfo {
        unimplemented!()
    }
}
