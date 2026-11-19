// Port of the C++ testapp file2.cpp. Classes become structs, namespaces become modules,
// overloaded file2func1() becomes two functions that each own a local static of the same name.
#![allow(dead_code)]

#[repr(C, packed)]
pub struct ClassAFile2 {
    pub int_in_class_a: i32,
}

pub mod namespace_a_file2 {
    use super::ClassAFile2;

    #[repr(C, packed)]
    pub struct ClassBAFile2 {
        pub int_in_class_ba: i32,
        pub class_a_instance: ClassAFile2,
    }

    #[repr(C, packed)]
    pub struct ClassBFile2 {
        pub int_in_class_b: i32,
        pub nested_class_instance: ClassBAFile2,
    }

    impl ClassBFile2 {
        pub const fn new() -> Self {
            Self {
                int_in_class_b: 0,
                nested_class_instance: ClassBAFile2 { int_in_class_ba: 0, class_a_instance: ClassAFile2 { int_in_class_a: 0 } },
            }
        }
    }
}

#[used]
pub static mut FILE2_GLOBAL_CHAR: i8 = 0;
#[used]
pub static mut FILE2_GLOBAL_INT: i32 = 0;
#[used]
pub static mut FILE2_GLOBAL_SHORT: i16 = 0;
#[used]
pub static mut FILE2_GLOBAL_LONG: i64 = 0;
#[used]
pub static mut FILE2_GLOBAL_UNSIGNED_CHAR: u8 = 0;
#[used]
pub static mut FILE2_GLOBAL_UNSIGNED_INT: u32 = 0;
#[used]
pub static mut FILE2_GLOBAL_UNSIGNED_SHORT: u16 = 0;
#[used]
pub static mut FILE2_GLOBAL_UNSIGNED_LONG: u64 = 0;
#[used]
pub static mut FILE2_GLOBAL_FLOAT: f32 = 0.0;
#[used]
pub static mut FILE2_GLOBAL_DOUBLE: f64 = 0.0;
#[used]
pub static mut FILE2_GLOBAL_BOOL: bool = false;
#[used]
pub static mut FILE2_CLASS_B_INSTANCE: namespace_a_file2::ClassBFile2 = namespace_a_file2::ClassBFile2::new();
#[used]
pub static mut FILE2_GLOBAL_ARRAY1_INT5: [i32; 5] = [0; 5];
#[used]
pub static mut FILE2_GLOBAL_ARRAY2X2_FLOAT: [[f32; 2]; 2] = [[0.0; 2]; 2];

#[used]
static mut FILE2_STATIC_CHAR: i8 = 0;
#[used]
static mut FILE2_STATIC_INT: i32 = 0;
#[used]
static mut FILE2_STATIC_SHORT: i16 = 0;
#[used]
static mut FILE2_STATIC_LONG: i64 = 0;
#[used]
static mut FILE2_STATIC_UNSIGNED_CHAR: u8 = 0;
#[used]
static mut FILE2_STATIC_UNSIGNED_INT: u32 = 0;
#[used]
static mut FILE2_STATIC_UNSIGNED_SHORT: u16 = 0;
#[used]
static mut FILE2_STATIC_UNSIGNED_LONG: u64 = 0;
#[used]
static mut FILE2_STATIC_FLOAT: f32 = 0.0;
#[used]
static mut FILE2_STATIC_DOUBLE: f64 = 0.0;
#[used]
static mut FILE2_STATIC_BOOL: bool = false;
#[used]
static mut FILE2_CLASS_B_STATIC_INSTANCE: namespace_a_file2::ClassBFile2 = namespace_a_file2::ClassBFile2::new();

pub mod namespace_in_file2 {
    // C++ unscoped enum: 4 bytes wide, unsigned values.
    #[repr(u32)]
    #[derive(Clone, Copy)]
    pub enum EnumA {
        EVal1 = 0,
        EVal2 = 1,
        EVal3 = 100,
        EVal4 = 101,
    }

    #[used]
    pub static mut INSTANCE_ENUM_A: EnumA = EnumA::EVal1;
    #[used]
    pub(crate) static mut STATIC_INSTANCE_ENUM_A: EnumA = EnumA::EVal1;
}

#[used]
pub static mut INSTANCE2_ENUM_A: namespace_in_file2::EnumA = namespace_in_file2::EnumA::EVal1;
#[used]
static mut STATIC_INSTANCE2_ENUM_A: namespace_in_file2::EnumA = namespace_in_file2::EnumA::EVal1;

pub fn file2_set_values() {
    use namespace_in_file2::EnumA;
    unsafe {
        FILE2_GLOBAL_CHAR = 20;
        FILE2_GLOBAL_INT = 2000;
        FILE2_GLOBAL_SHORT = 998;
        FILE2_GLOBAL_LONG = 555555;
        FILE2_GLOBAL_UNSIGNED_CHAR = 254;
        FILE2_GLOBAL_UNSIGNED_INT = 123456;
        FILE2_GLOBAL_UNSIGNED_SHORT = 12345;
        FILE2_GLOBAL_UNSIGNED_LONG = 1234567;
        FILE2_GLOBAL_FLOAT = 0.1;
        FILE2_GLOBAL_DOUBLE = 0.11111111111111;
        FILE2_GLOBAL_BOOL = false;

        FILE2_STATIC_CHAR = -66;
        FILE2_STATIC_INT = -8745;
        FILE2_STATIC_SHORT = -9876;
        FILE2_STATIC_LONG = -12345678;
        FILE2_STATIC_UNSIGNED_CHAR = 12;
        FILE2_STATIC_UNSIGNED_INT = 34;
        FILE2_STATIC_UNSIGNED_SHORT = 56;
        FILE2_STATIC_UNSIGNED_LONG = 78;
        FILE2_STATIC_FLOAT = 2.22222;
        FILE2_STATIC_DOUBLE = 3.3333;
        FILE2_STATIC_BOOL = true;

        namespace_in_file2::INSTANCE_ENUM_A = EnumA::EVal2;
        namespace_in_file2::STATIC_INSTANCE_ENUM_A = EnumA::EVal3;
        INSTANCE2_ENUM_A = EnumA::EVal4;
        STATIC_INSTANCE2_ENUM_A = EnumA::EVal1;

        FILE2_GLOBAL_ARRAY1_INT5[0] = 1111;
        FILE2_GLOBAL_ARRAY1_INT5[1] = 2222;
        FILE2_GLOBAL_ARRAY1_INT5[2] = 3333;
        FILE2_GLOBAL_ARRAY1_INT5[3] = 4444;
        FILE2_GLOBAL_ARRAY1_INT5[4] = 5555;

        FILE2_GLOBAL_ARRAY2X2_FLOAT[0][0] = 1.1;
        FILE2_GLOBAL_ARRAY2X2_FLOAT[0][1] = 2.2;
        FILE2_GLOBAL_ARRAY2X2_FLOAT[1][0] = 3.3;
        FILE2_GLOBAL_ARRAY2X2_FLOAT[1][1] = 4.4;

        FILE2_CLASS_B_INSTANCE.int_in_class_b = -11111;
        FILE2_CLASS_B_INSTANCE.nested_class_instance.int_in_class_ba = -22222;
        FILE2_CLASS_B_INSTANCE.nested_class_instance.class_a_instance.int_in_class_a = -33333;

        FILE2_CLASS_B_STATIC_INSTANCE.int_in_class_b = -44444;
        FILE2_CLASS_B_STATIC_INSTANCE.nested_class_instance.int_in_class_ba = -55555;
        FILE2_CLASS_B_STATIC_INSTANCE.nested_class_instance.class_a_instance.int_in_class_a = -66666;
    }
}

pub fn file2func1() {
    #[used]
    static mut FILE2FUNC1_VAR: i32 = -88778877;
}

pub fn file2func1_with_arg(_x: i32) {
    #[used]
    static mut FILE2FUNC1_VAR: f64 = 963258741.123;
}
