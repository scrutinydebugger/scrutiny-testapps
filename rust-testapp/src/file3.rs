// Port of the C++ testapp file3.cpp. The bitfield members (File3AnonymousBitfieldInUnion,
// field3_enum_bitfields) have no Rust equivalent and are omitted.
#![allow(dead_code)]

#[repr(C)]
pub union File3Union {
    pub u32_var: u32,
    pub u16_var: u16,
    pub u8_var: u8,
}

pub mod file_namespace {
    #[repr(u32)]
    #[derive(Clone, Copy)]
    pub enum File3EnumInClass {
        Aaa = 0,
        Bbb = 1,
        Ccc = 2,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Field3U16 {
        pub p0: u16, // LSB
        pub p1: u16,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Field3U8 {
        pub p0: u8, // LSB
        pub p1: u8,
        pub p2: u8,
        pub p3: u8,
    }

    #[repr(C)]
    pub union Field3 {
        pub field3_u32: u32,
        pub field3_u16: Field3U16,
        pub field3_u8: Field3U8,
    }

    #[repr(C)]
    pub struct File3ComplexStruct {
        pub field1: u32,
        pub field2: u32,
        pub field3: Field3,
    }

    #[repr(C)]
    pub struct File3TestClass {
        pub m_file3testclass_inclassenum: File3EnumInClass,
        pub m_file3_complex_struct: File3ComplexStruct,
    }

    impl File3TestClass {
        pub const fn new() -> Self {
            Self {
                m_file3testclass_inclassenum: File3EnumInClass::Aaa,
                m_file3_complex_struct: File3ComplexStruct { field1: 0, field2: 0, field3: Field3 { field3_u32: 0 } },
            }
        }
    }
}

#[used]
pub static mut FILE3_UNION: File3Union = File3Union { u32_var: 0 };
#[used]
pub static mut FILE3_TEST_CLASS: file_namespace::File3TestClass = file_namespace::File3TestClass::new();

pub fn file3_set_values() {
    unsafe {
        FILE3_UNION.u32_var = 0x12345678;
        FILE3_UNION.u16_var = 0xAABB;
        FILE3_UNION.u8_var = 0x99;
        // result should be 99AA3412 in memory (little endian)

        FILE3_TEST_CLASS.m_file3testclass_inclassenum = file_namespace::File3EnumInClass::Bbb;

        FILE3_TEST_CLASS.m_file3_complex_struct.field1 = 0x11223344;
        FILE3_TEST_CLASS.m_file3_complex_struct.field2 = 0x55667788;
        FILE3_TEST_CLASS.m_file3_complex_struct.field3.field3_u32 = 0x12345678;
        FILE3_TEST_CLASS.m_file3_complex_struct.field3.field3_u16.p0 = 0xBCDE;
        FILE3_TEST_CLASS.m_file3_complex_struct.field3.field3_u8.p3 = 0xAA;
        // Should read field3 = AA34BCDE
    }
}
