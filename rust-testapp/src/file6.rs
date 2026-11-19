// Rust-specific types with no C++ counterpart in the C++ testapp.
#![allow(dead_code)]

use core::num::NonZeroU16;

// Default repr: the compiler is free to reorder fields.
pub struct File6RustReprStruct {
    pub a_u8: u8,
    pub b_u32: u32,
    pub c_u16: u16,
    pub d_u64: u64,
}

#[repr(C)]
pub struct File6ReprCStruct {
    pub a_u8: u8,
    pub b_u32: u32,
    pub c_u16: u16,
    pub d_u64: u64,
}

pub struct File6TupleStruct(pub u16, pub i16);

// Rust enum with payloads (tagged union).
pub enum File6PayloadEnum {
    Empty,
    Int(i32),
    Pair { a: u16, b: u16 },
}

#[repr(u8)]
pub enum File6ReprU8Enum {
    First = 1,
    Second = 2,
    Third = 200,
}

pub struct File6Generic<T> {
    pub value: T,
    pub count: u8,
}

pub struct File6WithMethod;

impl File6WithMethod {
    pub fn method_with_static() {
        #[used]
        static mut STATIC_IN_METHOD: u32 = 0x600DF00D;
    }
}

pub mod file6_outer {
    pub mod file6_inner {
        #[used]
        pub static mut FILE6_DEEP_VAR: u16 = 0;
    }
}

// Referenced by FILE6_OPTION_REF; immutable so it lives in .rodata.
#[used]
pub static FILE6_CONST_U32: u32 = 0x13572468;

#[used]
pub static mut FILE6_CHAR: char = '\0';
#[used]
pub static mut FILE6_U128: u128 = 0;
#[used]
pub static mut FILE6_I128: i128 = 0;
#[used]
pub static mut FILE6_USIZE: usize = 0;
#[used]
pub static mut FILE6_ISIZE: isize = 0;
#[used]
pub static mut FILE6_BOOL_ARRAY: [bool; 4] = [false; 4];
#[used]
pub static mut FILE6_TUPLE: (u8, i32, f64) = (0, 0, 0.0);
#[used]
pub static mut FILE6_TUPLE_STRUCT: File6TupleStruct = File6TupleStruct(0, 0);
#[used]
pub static mut FILE6_RUST_REPR_STRUCT: File6RustReprStruct = File6RustReprStruct { a_u8: 0, b_u32: 0, c_u16: 0, d_u64: 0 };
#[used]
pub static mut FILE6_REPR_C_STRUCT: File6ReprCStruct = File6ReprCStruct { a_u8: 0, b_u32: 0, c_u16: 0, d_u64: 0 };
#[used]
pub static mut FILE6_PAYLOAD_ENUM_PAIR: File6PayloadEnum = File6PayloadEnum::Empty;
#[used]
pub static mut FILE6_PAYLOAD_ENUM_INT: File6PayloadEnum = File6PayloadEnum::Empty;
#[used]
pub static mut FILE6_REPR_U8_ENUM: File6ReprU8Enum = File6ReprU8Enum::First;
#[used]
pub static mut FILE6_OPTION_U32_SOME: Option<u32> = None;
#[used]
pub static mut FILE6_OPTION_U32_NONE: Option<u32> = None;
#[used]
pub static mut FILE6_OPTION_NONZERO: Option<NonZeroU16> = None;
#[used]
pub static mut FILE6_OPTION_REF: Option<&'static u32> = None;
#[used]
pub static mut FILE6_STR: &str = "";
#[used]
pub static mut FILE6_GENERIC_U32: File6Generic<u32> = File6Generic { value: 0, count: 0 };
#[used]
pub static mut FILE6_GENERIC_F32: File6Generic<f32> = File6Generic { value: 0.0, count: 0 };

// Unmangled symbol name, as exported to C.
#[used]
#[unsafe(no_mangle)]
pub static mut file6_no_mangle_u32: u32 = 0;

pub fn file6_set_values() {
    unsafe {
        FILE6_CHAR = 'é';
        FILE6_U128 = 0x0123456789ABCDEF_FEDCBA9876543210;
        FILE6_I128 = -0x0123456789ABCDEF_0EDCBA9876543210;
        FILE6_USIZE = 0x1122334455667788;
        FILE6_ISIZE = -0x1122334455667788;
        FILE6_BOOL_ARRAY = [true, false, true, true];
        FILE6_TUPLE = (0xAB, -123456, 2.5);
        FILE6_TUPLE_STRUCT = File6TupleStruct(0xBEEF, -1234);
        FILE6_RUST_REPR_STRUCT = File6RustReprStruct { a_u8: 0x11, b_u32: 0x22222222, c_u16: 0x3333, d_u64: 0x4444444444444444 };
        FILE6_REPR_C_STRUCT = File6ReprCStruct { a_u8: 0x55, b_u32: 0x66666666, c_u16: 0x7777, d_u64: 0x8888888888888888 };
        FILE6_PAYLOAD_ENUM_PAIR = File6PayloadEnum::Pair { a: 0x1234, b: 0x5678 };
        FILE6_PAYLOAD_ENUM_INT = File6PayloadEnum::Int(-98765);
        FILE6_REPR_U8_ENUM = File6ReprU8Enum::Third;
        FILE6_OPTION_U32_SOME = Some(0xCAFEBABE);
        FILE6_OPTION_U32_NONE = None;
        FILE6_OPTION_NONZERO = NonZeroU16::new(0x4321);
        FILE6_OPTION_REF = Some(&FILE6_CONST_U32);
        FILE6_STR = "Hello Scrutiny ✓";
        FILE6_GENERIC_U32 = File6Generic { value: 0x12121212, count: 3 };
        FILE6_GENERIC_F32 = File6Generic { value: -1.5, count: 7 };
        file6_outer::file6_inner::FILE6_DEEP_VAR = 0xD00D;
        file6_no_mangle_u32 = 0x5A5A5A5A;
    }
}
