// Port of the C++ testapp file4.cpp. C++ inheritance (File4ClassA2 : File4ClassA1) becomes a `base` member.
#![allow(dead_code)]

pub mod file4_namespace_a1 {
    #[repr(u16)]
    #[derive(Clone, Copy)]
    pub enum File4EnumA {
        Xxx = 123,
        Yyy = 456,
    }

    pub mod file4_namespace_a2 {
        use super::File4EnumA;

        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct File4ClassA1 {
            pub xxx: u32,
            pub yyy: [[i32; 3]; 2],
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct File4ClassA2 {
            pub base: File4ClassA1,
            pub zzz: [[[u16; 6]; 5]; 4],
            pub a2enum: File4EnumA,
        }

        impl File4ClassA2 {
            pub const fn new() -> Self {
                Self { base: File4ClassA1 { xxx: 0, yyy: [[0; 3]; 2] }, zzz: [[[0; 6]; 5]; 4], a2enum: File4EnumA::Xxx }
            }
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        pub union File4ClassA3Union {
            pub u32: u32,
            pub u8: [u8; 4],
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct File4ClassA3 {
            pub the_union: File4ClassA3Union,
        }

        impl File4ClassA3 {
            pub const fn new() -> Self {
                Self { the_union: File4ClassA3Union { u32: 0 } }
            }
        }
    }
}

pub mod file4_namespace_b {
    use super::file4_namespace_a1::file4_namespace_a2::File4ClassA2;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct File4ClassB {
        pub some_bool: bool,
        pub array_of_a2: [[File4ClassA2; 2]; 3],
    }

    impl File4ClassB {
        pub const fn new() -> Self {
            Self { some_bool: false, array_of_a2: [[File4ClassA2::new(); 2]; 3] }
        }
    }
}

use file4_namespace_a1::file4_namespace_a2::File4ClassA3;
use file4_namespace_a1::File4EnumA;
use file4_namespace_b::File4ClassB;

#[used]
pub static mut FILE4_CLASS_B: File4ClassB = File4ClassB::new();
#[used]
pub static mut FILE4_CLASS_B_ARRAY: [File4ClassB; 3] = [File4ClassB::new(); 3];
#[used]
pub static mut FILE4_CLASS_A3_ARRAY: [[File4ClassA3; 2]; 2] = [[File4ClassA3::new(); 2]; 2];

// Sets the same pattern as the C++ testapp in a File4ClassB, shifted by `offset`.
unsafe fn set_class_b_values(target: *mut File4ClassB, some_bool: bool, offset: u32, a2enum: File4EnumA) {
    let a2 = &mut (*target).array_of_a2[2][0];
    (*target).some_bool = some_bool;
    a2.base.xxx = 0xdeadbeef + offset;
    a2.base.yyy[0][0] = (0x10001111 + offset) as i32;
    a2.base.yyy[0][1] = (0x20002222 + offset) as i32;
    a2.base.yyy[0][2] = (0x30003333 + offset) as i32;
    a2.base.yyy[1][2] = (0x40004444 + offset) as i32;
    a2.zzz[0][0][0] = (0x5566 + offset) as u16;
    a2.zzz[1][2][3] = (0x6789 + offset) as u16;
    a2.a2enum = a2enum;
}

pub fn file4_set_values() {
    unsafe {
        set_class_b_values(&raw mut FILE4_CLASS_B, true, 0, File4EnumA::Yyy);
        set_class_b_values(&raw mut FILE4_CLASS_B_ARRAY[0], true, 1, File4EnumA::Xxx);
        set_class_b_values(&raw mut FILE4_CLASS_B_ARRAY[1], true, 2, File4EnumA::Xxx);
        set_class_b_values(&raw mut FILE4_CLASS_B_ARRAY[2], false, 3, File4EnumA::Yyy);

        FILE4_CLASS_A3_ARRAY[0][0].the_union.u32 = 0x12345678;
        FILE4_CLASS_A3_ARRAY[0][1].the_union.u32 = 0xAABBCCDD;
        FILE4_CLASS_A3_ARRAY[1][0].the_union.u32 = 0x11223344;
        FILE4_CLASS_A3_ARRAY[1][1].the_union.u32 = 0x55667788;
    }
}
