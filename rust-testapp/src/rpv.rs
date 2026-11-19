// Runtime Published Values. Same IDs, types and initial values as the C++ testapp.
use scrutiny_embedded_rust::types::{AnyType, RuntimePublishedValue, VariableType};

pub static RPVS: [RuntimePublishedValue; 13] = [
    RuntimePublishedValue { id: 0x1000, var_type: VariableType::Sint8 },
    RuntimePublishedValue { id: 0x1001, var_type: VariableType::Sint16 },
    RuntimePublishedValue { id: 0x1002, var_type: VariableType::Sint32 },
    RuntimePublishedValue { id: 0x1003, var_type: VariableType::Sint64 },
    RuntimePublishedValue { id: 0x2000, var_type: VariableType::Uint8 },
    RuntimePublishedValue { id: 0x2001, var_type: VariableType::Uint16 },
    RuntimePublishedValue { id: 0x2002, var_type: VariableType::Uint32 },
    RuntimePublishedValue { id: 0x2003, var_type: VariableType::Uint64 },
    RuntimePublishedValue { id: 0x3000, var_type: VariableType::Float32 },
    RuntimePublishedValue { id: 0x3001, var_type: VariableType::Float64 },
    RuntimePublishedValue { id: 0x4000, var_type: VariableType::Boolean },
    RuntimePublishedValue { id: 0x5000, var_type: VariableType::Boolean },
    RuntimePublishedValue { id: 0x5001, var_type: VariableType::Uint16 },
];

pub struct RpvStorage {
    pub rpv_id_1000: i8,
    pub rpv_id_1001: i16,
    pub rpv_id_1002: i32,
    pub rpv_id_1003: i64,
    pub rpv_id_2000: u8,
    pub rpv_id_2001: u16,
    pub rpv_id_2002: u32,
    pub rpv_id_2003: u64,
    pub rpv_id_3000: f32,
    pub rpv_id_3001: f64,
    pub rpv_id_4000: bool,
    pub rpv_id_5000: bool,
    pub rpv_id_5001: u16,
}

pub static mut RPV_STORAGE: RpvStorage = RpvStorage {
    rpv_id_1000: 0,
    rpv_id_1001: 0,
    rpv_id_1002: 0,
    rpv_id_1003: 0,
    rpv_id_2000: 0,
    rpv_id_2001: 0,
    rpv_id_2002: 0,
    rpv_id_2003: 0,
    rpv_id_3000: 0.0,
    rpv_id_3001: 0.0,
    rpv_id_4000: false,
    rpv_id_5000: false,
    rpv_id_5001: 0,
};

// Test values copied from the C++ testapp, kept as-is.
#[allow(clippy::approx_constant)]
pub fn rpv_set_values() {
    unsafe {
        RPV_STORAGE.rpv_id_1000 = -10;
        RPV_STORAGE.rpv_id_1001 = -30000;
        RPV_STORAGE.rpv_id_1002 = 0xABCDEF;
        RPV_STORAGE.rpv_id_1003 = 0xAABBCCDDEEFF;
        RPV_STORAGE.rpv_id_2000 = 0xAB;
        RPV_STORAGE.rpv_id_2001 = 0x1234;
        RPV_STORAGE.rpv_id_2002 = 0x12345678;
        RPV_STORAGE.rpv_id_2003 = 0x111111111111;
        RPV_STORAGE.rpv_id_3000 = 3.1415926;
        RPV_STORAGE.rpv_id_3001 = 2.71828;
        RPV_STORAGE.rpv_id_4000 = true;
        RPV_STORAGE.rpv_id_5000 = false;
        RPV_STORAGE.rpv_id_5001 = 0;
    }
}

pub fn rpv_read(rpv: RuntimePublishedValue, out: &mut AnyType, _caller: Option<&'static str>) -> bool {
    unsafe {
        match rpv.id {
            0x1000 => out.sint8 = RPV_STORAGE.rpv_id_1000,
            0x1001 => out.sint16 = RPV_STORAGE.rpv_id_1001,
            0x1002 => out.sint32 = RPV_STORAGE.rpv_id_1002,
            0x1003 => out.sint64 = RPV_STORAGE.rpv_id_1003,
            0x2000 => out.uint8 = RPV_STORAGE.rpv_id_2000,
            0x2001 => out.uint16 = RPV_STORAGE.rpv_id_2001,
            0x2002 => out.uint32 = RPV_STORAGE.rpv_id_2002,
            0x2003 => out.uint64 = RPV_STORAGE.rpv_id_2003,
            0x3000 => out.float32 = RPV_STORAGE.rpv_id_3000,
            0x3001 => out.float64 = RPV_STORAGE.rpv_id_3001,
            0x4000 => out.boolean = RPV_STORAGE.rpv_id_4000,
            0x5000 => out.boolean = RPV_STORAGE.rpv_id_5000,
            0x5001 => out.uint16 = RPV_STORAGE.rpv_id_5001,
            _ => return false,
        }
    }
    true
}

pub fn rpv_write(rpv: RuntimePublishedValue, val: &AnyType) -> bool {
    unsafe {
        match rpv.id {
            0x1000 => RPV_STORAGE.rpv_id_1000 = val.sint8,
            0x1001 => RPV_STORAGE.rpv_id_1001 = val.sint16,
            0x1002 => RPV_STORAGE.rpv_id_1002 = val.sint32,
            0x1003 => RPV_STORAGE.rpv_id_1003 = val.sint64,
            0x2000 => RPV_STORAGE.rpv_id_2000 = val.uint8,
            0x2001 => RPV_STORAGE.rpv_id_2001 = val.uint16,
            0x2002 => RPV_STORAGE.rpv_id_2002 = val.uint32,
            0x2003 => RPV_STORAGE.rpv_id_2003 = val.uint64,
            0x3000 => RPV_STORAGE.rpv_id_3000 = val.float32,
            0x3001 => RPV_STORAGE.rpv_id_3001 = val.float64,
            0x4000 => RPV_STORAGE.rpv_id_4000 = val.boolean,
            0x5000 => RPV_STORAGE.rpv_id_5000 = val.boolean,
            0x5001 => RPV_STORAGE.rpv_id_5001 = val.uint16,
            _ => return false,
        }
    }
    true
}
