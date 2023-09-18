use crate::types::SqlType;

use super::{u256, i256};

pub trait StatBuffer {
    type Buffer: AsMut<[u8]> + AsRef<[u8]> + Copy + Sync;
    fn buffer() -> Self::Buffer;
    fn sql_type() -> SqlType;
}

impl StatBuffer for u8 {
    type Buffer = [Self; 1];

    fn buffer() -> Self::Buffer {
        [0; 1]
    }

    fn sql_type() -> SqlType {
        SqlType::UInt8
    }
}

impl StatBuffer for u16 {
    type Buffer = [u8; 2];

    fn buffer() -> Self::Buffer {
        [0; 2]
    }

    fn sql_type() -> SqlType {
        SqlType::UInt16
    }
}

impl StatBuffer for u32 {
    type Buffer = [u8; 4];

    fn buffer() -> Self::Buffer {
        [0; 4]
    }

    fn sql_type() -> SqlType {
        SqlType::UInt32
    }
}

impl StatBuffer for u64 {
    type Buffer = [u8; 8];

    fn buffer() -> Self::Buffer {
        [0; 8]
    }

    fn sql_type() -> SqlType {
        SqlType::UInt64
    }
}

impl StatBuffer for u128 {
    type Buffer = [u8; 16];

    fn buffer() -> Self::Buffer {
        [0; 16]
    }

    fn sql_type() -> SqlType {
        SqlType::UInt128
    }
}

impl StatBuffer for u256 {
    type Buffer = [u8; 32];

    fn buffer() -> Self::Buffer {
        [0; 32]
    }

    fn sql_type() -> SqlType {
        SqlType::UInt256
    }
}

impl StatBuffer for i8 {
    type Buffer = [u8; 1];

    fn buffer() -> Self::Buffer {
        [0; 1]
    }

    fn sql_type() -> SqlType {
        SqlType::Int8
    }
}

impl StatBuffer for i16 {
    type Buffer = [u8; 2];

    fn buffer() -> Self::Buffer {
        [0; 2]
    }

    fn sql_type() -> SqlType {
        SqlType::Int16
    }
}

impl StatBuffer for i32 {
    type Buffer = [u8; 4];

    fn buffer() -> Self::Buffer {
        [0; 4]
    }

    fn sql_type() -> SqlType {
        SqlType::Int32
    }
}

impl StatBuffer for i64 {
    type Buffer = [u8; 8];

    fn buffer() -> Self::Buffer {
        [0; 8]
    }

    fn sql_type() -> SqlType {
        SqlType::Int64
    }
}

impl StatBuffer for i128 {
    type Buffer = [u8; 16];

    fn buffer() -> Self::Buffer {
        [0; 16]
    }

    fn sql_type() -> SqlType {
        SqlType::Int128
    }
}

impl StatBuffer for i256 {
    type Buffer = [u8; 32];

    fn buffer() -> Self::Buffer {
        [0; 32]
    }

    fn sql_type() -> SqlType {
        SqlType::Int256
    }
}

impl StatBuffer for f32 {
    type Buffer = [u8; 4];

    fn buffer() -> Self::Buffer {
        [0; 4]
    }

    fn sql_type() -> SqlType {
        SqlType::Float32
    }
}

impl StatBuffer for f64 {
    type Buffer = [u8; 8];

    fn buffer() -> Self::Buffer {
        [0; 8]
    }

    fn sql_type() -> SqlType {
        SqlType::Float64
    }
}

impl StatBuffer for bool {
    type Buffer = [u8; 1];

    fn buffer() -> Self::Buffer {
        [0; 1]
    }

    fn sql_type() -> SqlType {
        SqlType::Bool
    }
}
