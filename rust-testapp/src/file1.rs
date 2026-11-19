// Port of the C++ testapp file1.cpp. C++ bitfields (StructD) have no Rust equivalent and are omitted.
// Every static is #[used] so it survives optimization even though nothing reads it.
#![allow(dead_code)]

#[repr(C, packed)]
pub struct StructA {
    pub struct_a_member_int: i32,
    pub struct_a_member_uint: u32,
    pub struct_a_member_float: f32,
    pub struct_a_member_double: f64,
    pub struct_a_member_bool: bool,
}

impl StructA {
    pub const fn new() -> Self {
        Self {
            struct_a_member_int: 0,
            struct_a_member_uint: 0,
            struct_a_member_float: 0.0,
            struct_a_member_double: 0.0,
            struct_a_member_bool: false,
        }
    }
}

#[repr(C, packed)]
pub struct StructB {
    pub struct_b_member_int: i32,
    pub struct_b_member_struct_a: StructA,
}

impl StructB {
    pub const fn new() -> Self {
        Self { struct_b_member_int: 0, struct_b_member_struct_a: StructA::new() }
    }
}

// Rust has no anonymous structs, so the nested C++ structs are named.
#[repr(C, packed)]
pub struct StructCNested2 {
    pub nested_struct_instance2_member_double: f64,
}

#[repr(C, packed)]
pub struct StructCNested {
    pub nested_struct_member_int: i32,
    pub nested_struct_member_float: f32,
    pub nested_struct_instance2: StructCNested2,
}

#[repr(C, packed)]
pub struct StructC {
    pub struct_c_member_int: i32,
    pub nested_struct_instance: StructCNested,
}

impl StructC {
    pub const fn new() -> Self {
        Self {
            struct_c_member_int: 0,
            nested_struct_instance: StructCNested {
                nested_struct_member_int: 0,
                nested_struct_member_float: 0.0,
                nested_struct_instance2: StructCNested2 { nested_struct_instance2_member_double: 0.0 },
            },
        }
    }
}

#[used]
pub static mut FILE1_GLOBAL_CHAR: i8 = 0;
#[used]
pub static mut FILE1_GLOBAL_INT: i32 = 0;
#[used]
pub static mut FILE1_GLOBAL_SHORT: i16 = 0;
#[used]
pub static mut FILE1_GLOBAL_LONG: i64 = 0;
#[used]
pub static mut FILE1_GLOBAL_UNSIGNED_CHAR: u8 = 0;
#[used]
pub static mut FILE1_GLOBAL_UNSIGNED_INT: u32 = 0;
#[used]
pub static mut FILE1_GLOBAL_UNSIGNED_SHORT: u16 = 0;
#[used]
pub static mut FILE1_GLOBAL_UNSIGNED_LONG: u64 = 0;
#[used]
pub static mut FILE1_GLOBAL_FLOAT: f32 = 0.0;
#[used]
pub static mut FILE1_GLOBAL_DOUBLE: f64 = 0.0;
#[used]
pub static mut FILE1_GLOBAL_BOOL: bool = false;

#[used]
static mut FILE1_STATIC_CHAR: i8 = 0;
#[used]
static mut FILE1_STATIC_INT: i32 = 0;
#[used]
static mut FILE1_STATIC_SHORT: i16 = 0;
#[used]
static mut FILE1_STATIC_LONG: i64 = 0;
#[used]
static mut FILE1_STATIC_UNSIGNED_CHAR: u8 = 0;
#[used]
static mut FILE1_STATIC_UNSIGNED_INT: u32 = 0;
#[used]
static mut FILE1_STATIC_UNSIGNED_SHORT: u16 = 0;
#[used]
static mut FILE1_STATIC_UNSIGNED_LONG: u64 = 0;
#[used]
static mut FILE1_STATIC_FLOAT: f32 = 0.0;
#[used]
static mut FILE1_STATIC_DOUBLE: f64 = 0.0;
#[used]
static mut FILE1_STATIC_BOOL: bool = false;

#[used]
static mut FILE1_STRUCT_A_STATIC_INSTANCE: StructA = StructA::new();

#[used]
pub static mut FILE1_STRUCT_A_INSTANCE: StructA = StructA::new();
#[used]
pub static mut FILE1_STRUCT_B_INSTANCE: StructB = StructB::new();
#[used]
pub static mut FILE1_STRUCT_C_INSTANCE: StructC = StructC::new();

pub mod namespace_in_file1 {
    pub mod namespace_in_file1_nested1 {
        #[used]
        pub static mut FILE1_GLOBAL_NESTED_VAR1: u64 = 0;
        #[used]
        pub(crate) static mut FILE1_STATIC_NESTED_VAR1: u64 = 0;
    }
}

// Test values copied from the C++ testapp, kept as-is.
#[allow(clippy::excessive_precision, clippy::approx_constant)]
pub fn file1_set_values() {
    unsafe {
        FILE1_STATIC_CHAR = 99;
        FILE1_STATIC_INT = 987654;
        FILE1_STATIC_SHORT = -666;
        FILE1_STATIC_LONG = -55555;
        FILE1_STATIC_UNSIGNED_CHAR = 44;
        FILE1_STATIC_UNSIGNED_INT = 3333;
        FILE1_STATIC_UNSIGNED_SHORT = 22222;
        FILE1_STATIC_UNSIGNED_LONG = 321321;
        FILE1_STATIC_FLOAT = 1.23456789;
        FILE1_STATIC_DOUBLE = 9.87654321;
        FILE1_STATIC_BOOL = true;

        FILE1_STRUCT_A_STATIC_INSTANCE.struct_a_member_int = -789;
        FILE1_STRUCT_A_STATIC_INSTANCE.struct_a_member_uint = 147258;
        FILE1_STRUCT_A_STATIC_INSTANCE.struct_a_member_float = 88.88;
        FILE1_STRUCT_A_STATIC_INSTANCE.struct_a_member_double = 99.99;
        FILE1_STRUCT_A_STATIC_INSTANCE.struct_a_member_bool = true;

        namespace_in_file1::namespace_in_file1_nested1::FILE1_STATIC_NESTED_VAR1 = 945612345;

        FILE1_GLOBAL_CHAR = -10;
        FILE1_GLOBAL_INT = -1000;
        FILE1_GLOBAL_SHORT = -999;
        FILE1_GLOBAL_LONG = -100000;
        FILE1_GLOBAL_UNSIGNED_CHAR = 55;
        FILE1_GLOBAL_UNSIGNED_INT = 100001;
        FILE1_GLOBAL_UNSIGNED_SHORT = 50000;
        FILE1_GLOBAL_UNSIGNED_LONG = 100002;
        FILE1_GLOBAL_FLOAT = 3.1415926;
        FILE1_GLOBAL_DOUBLE = 1.71;
        FILE1_GLOBAL_BOOL = true;

        FILE1_STRUCT_A_INSTANCE.struct_a_member_int = -654;
        FILE1_STRUCT_A_INSTANCE.struct_a_member_uint = 258147;
        FILE1_STRUCT_A_INSTANCE.struct_a_member_float = 77.77;
        FILE1_STRUCT_A_INSTANCE.struct_a_member_double = 66.66;
        FILE1_STRUCT_A_INSTANCE.struct_a_member_bool = false;

        FILE1_STRUCT_B_INSTANCE.struct_b_member_int = 55555;
        FILE1_STRUCT_B_INSTANCE.struct_b_member_struct_a.struct_a_member_int = -199999;
        FILE1_STRUCT_B_INSTANCE.struct_b_member_struct_a.struct_a_member_uint = 33333;
        FILE1_STRUCT_B_INSTANCE.struct_b_member_struct_a.struct_a_member_float = 33.33;
        FILE1_STRUCT_B_INSTANCE.struct_b_member_struct_a.struct_a_member_double = 22.22;
        FILE1_STRUCT_B_INSTANCE.struct_b_member_struct_a.struct_a_member_bool = true;

        FILE1_STRUCT_C_INSTANCE.struct_c_member_int = 888874;
        FILE1_STRUCT_C_INSTANCE.nested_struct_instance.nested_struct_member_int = 2298744;
        FILE1_STRUCT_C_INSTANCE.nested_struct_instance.nested_struct_member_float = -147.55;
        FILE1_STRUCT_C_INSTANCE.nested_struct_instance.nested_struct_instance2.nested_struct_instance2_member_double = 654.654;

        namespace_in_file1::namespace_in_file1_nested1::FILE1_GLOBAL_NESTED_VAR1 = 1111111111;
    }
}

pub fn func_in_file1(a: i32, b: i32) -> i32 {
    #[used]
    static mut STATIC_LONG_IN_FUNC_FILE1: i64 = -0x123456789abcdef;
    a + b
}
