// Port of the C++ testapp file5.cpp: raw pointers to scalars and structs.
#![allow(dead_code)]

pub mod file5_namespace_a {
    use core::ptr::null_mut;

    #[repr(C, packed)]
    pub struct File5StructA {
        pub i32: i32,
        pub i32_ptr: *mut i32,
        pub u16_array: [u16; 5],
    }

    #[repr(C, packed)]
    pub struct File5StructB {
        pub u32: u32,
        pub struct_a_ptr: *mut File5StructA,
    }

    impl File5StructB {
        pub const fn new() -> Self {
            Self { u32: 0, struct_a_ptr: null_mut() }
        }
    }

    #[used]
    pub static mut FILE5_I64: i64 = 0;
    #[used]
    pub static mut FILE5_I32: i32 = 0;
    #[used]
    pub static mut FILE5_I16: i16 = 0;
    #[used]
    pub static mut FILE5_I8: i8 = 0;

    #[used]
    pub static mut FILE5_U64: u64 = 0;
    #[used]
    pub static mut FILE5_U32: u32 = 0;
    #[used]
    pub static mut FILE5_U16: u16 = 0;
    #[used]
    pub static mut FILE5_U8: u8 = 0;

    #[used]
    pub static mut FILE5_STRUCT_A: File5StructA = File5StructA { i32: 0, i32_ptr: null_mut(), u16_array: [0; 5] };
    #[used]
    pub static mut FILE5_STRUCT_B: File5StructB = File5StructB::new();
    #[used]
    pub static mut FILE5_STRUCTB_ARRAY: [File5StructB; 2] = [File5StructB::new(), File5StructB::new()];
}

pub mod file5_namespace_b {
    use super::file5_namespace_a::{File5StructA, File5StructB};
    use core::ptr::null_mut;

    #[used]
    pub static mut FILE5_I64_PTR: *mut i64 = null_mut();
    #[used]
    pub static mut FILE5_I32_PTR: *mut i32 = null_mut();
    #[used]
    pub static mut FILE5_I16_PTR: *mut i16 = null_mut();
    #[used]
    pub static mut FILE5_I8_PTR: *mut i8 = null_mut();

    #[used]
    pub static mut FILE5_U64_PTR: *mut u64 = null_mut();
    #[used]
    pub static mut FILE5_U32_PTR: *mut u32 = null_mut();
    #[used]
    pub static mut FILE5_U16_PTR: *mut u16 = null_mut();
    #[used]
    pub static mut FILE5_U8_PTR: *mut u8 = null_mut();

    #[used]
    pub static mut FILE5_STRUCT_A_PTR: *mut File5StructA = null_mut();
    #[used]
    pub static mut FILE5_STRUCT_B_PTR: *mut File5StructB = null_mut();
    #[used]
    pub static mut FILE5_STRUCT_B_ARRAY_PTR: *mut File5StructB = null_mut();
}

pub fn file5_set_values() {
    use file5_namespace_a as a;
    use file5_namespace_b as b;
    unsafe {
        a::FILE5_I64 = -0x123456789abcdef;
        a::FILE5_I32 = -0x77553311;
        a::FILE5_I16 = -0x2A3C;
        a::FILE5_I8 = -0x35;
        a::FILE5_U64 = 0x98765432123456;
        a::FILE5_U32 = 0xaabbccdd;
        a::FILE5_U16 = 0xFDCE;
        a::FILE5_U8 = 0xBD;

        b::FILE5_I64_PTR = &raw mut a::FILE5_I64;
        b::FILE5_I32_PTR = &raw mut a::FILE5_I32;
        b::FILE5_I16_PTR = &raw mut a::FILE5_I16;
        b::FILE5_I8_PTR = &raw mut a::FILE5_I8;

        b::FILE5_U64_PTR = &raw mut a::FILE5_U64;
        b::FILE5_U32_PTR = &raw mut a::FILE5_U32;
        b::FILE5_U16_PTR = &raw mut a::FILE5_U16;
        b::FILE5_U8_PTR = &raw mut a::FILE5_U8;

        b::FILE5_STRUCT_A_PTR = &raw mut a::FILE5_STRUCT_A;
        b::FILE5_STRUCT_B_PTR = &raw mut a::FILE5_STRUCT_B;
        b::FILE5_STRUCT_B_ARRAY_PTR = (&raw mut a::FILE5_STRUCTB_ARRAY).cast::<a::File5StructB>();

        a::FILE5_STRUCT_A.i32 = 31415926;
        a::FILE5_STRUCT_A.i32_ptr = &raw mut a::FILE5_I32;
        a::FILE5_STRUCT_A.u16_array = [0x121, 0x122, 0x123, 0x124, 0x125];

        a::FILE5_STRUCT_B.u32 = 0x97461346;
        a::FILE5_STRUCT_B.struct_a_ptr = &raw mut a::FILE5_STRUCT_A;

        a::FILE5_STRUCTB_ARRAY[0].u32 = 0x1195735;
        a::FILE5_STRUCTB_ARRAY[0].struct_a_ptr = &raw mut a::FILE5_STRUCT_A;

        a::FILE5_STRUCTB_ARRAY[1].u32 = 0x1284629;
        a::FILE5_STRUCTB_ARRAY[1].struct_a_ptr = &raw mut a::FILE5_STRUCT_A;
    }
}
