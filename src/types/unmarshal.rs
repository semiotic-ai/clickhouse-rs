use super::{u256, i256};

pub trait Unmarshal<T: Copy> {
    fn unmarshal(scratch: &[u8]) -> T;
}

impl Unmarshal<u8> for u8 {
    fn unmarshal(scratch: &[u8]) -> Self {
        scratch[0]
    }
}

impl Unmarshal<u16> for u16 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0]) | Self::from(scratch[1]) << 8
    }
}

impl Unmarshal<u32> for u32 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
    }
}

impl Unmarshal<u64> for u64 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
            | Self::from(scratch[4]) << 32
            | Self::from(scratch[5]) << 40
            | Self::from(scratch[6]) << 48
            | Self::from(scratch[7]) << 56
    }
}

impl Unmarshal<u128> for u128 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
            | Self::from(scratch[4]) << 32
            | Self::from(scratch[5]) << 40
            | Self::from(scratch[6]) << 48
            | Self::from(scratch[7]) << 56
            | Self::from(scratch[8]) << 64
            | Self::from(scratch[9]) << 72
            | Self::from(scratch[10]) << 80
            | Self::from(scratch[11]) << 88
            | Self::from(scratch[12]) << 96
            | Self::from(scratch[13]) << 104
            | Self::from(scratch[14]) << 112
            | Self::from(scratch[15]) << 120
    }
}

impl Unmarshal<u256> for u256 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
            | Self::from(scratch[4]) << 32
            | Self::from(scratch[5]) << 40
            | Self::from(scratch[6]) << 48
            | Self::from(scratch[7]) << 56
            | Self::from(scratch[8]) << 64
            | Self::from(scratch[9]) << 72
            | Self::from(scratch[10]) << 80
            | Self::from(scratch[11]) << 88
            | Self::from(scratch[12]) << 96
            | Self::from(scratch[13]) << 104
            | Self::from(scratch[14]) << 112
            | Self::from(scratch[15]) << 120
            | Self::from(scratch[16]) << 128
            | Self::from(scratch[17]) << 136
            | Self::from(scratch[18]) << 144
            | Self::from(scratch[19]) << 152
            | Self::from(scratch[20]) << 160
            | Self::from(scratch[21]) << 168
            | Self::from(scratch[22]) << 176
            | Self::from(scratch[23]) << 184
            | Self::from(scratch[24]) << 192
            | Self::from(scratch[25]) << 200
            | Self::from(scratch[26]) << 208
            | Self::from(scratch[27]) << 216
            | Self::from(scratch[28]) << 224
            | Self::from(scratch[29]) << 232
            | Self::from(scratch[30]) << 240
            | Self::from(scratch[31]) << 248
    }
}

impl Unmarshal<i8> for i8 {
    fn unmarshal(scratch: &[u8]) -> Self {
        scratch[0] as Self
    }
}

impl Unmarshal<i16> for i16 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0]) | Self::from(scratch[1]) << 8
    }
}

impl Unmarshal<i32> for i32 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
    }
}

impl Unmarshal<i64> for i64 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
            | Self::from(scratch[4]) << 32
            | Self::from(scratch[5]) << 40
            | Self::from(scratch[6]) << 48
            | Self::from(scratch[7]) << 56
    }
}

impl Unmarshal<i128> for i128 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
            | Self::from(scratch[4]) << 32
            | Self::from(scratch[5]) << 40
            | Self::from(scratch[6]) << 48
            | Self::from(scratch[7]) << 56
            | Self::from(scratch[8]) << 64
            | Self::from(scratch[9]) << 72
            | Self::from(scratch[10]) << 80
            | Self::from(scratch[11]) << 88
            | Self::from(scratch[12]) << 96
            | Self::from(scratch[13]) << 104
            | Self::from(scratch[14]) << 112
            | Self::from(scratch[15]) << 120
    }
}

impl Unmarshal<i256> for i256 {
    fn unmarshal(scratch: &[u8]) -> Self {
        Self::from(scratch[0])
            | Self::from(scratch[1]) << 8
            | Self::from(scratch[2]) << 16
            | Self::from(scratch[3]) << 24
            | Self::from(scratch[4]) << 32
            | Self::from(scratch[5]) << 40
            | Self::from(scratch[6]) << 48
            | Self::from(scratch[7]) << 56
            | Self::from(scratch[8]) << 64
            | Self::from(scratch[9]) << 72
            | Self::from(scratch[10]) << 80
            | Self::from(scratch[11]) << 88
            | Self::from(scratch[12]) << 96
            | Self::from(scratch[13]) << 104
            | Self::from(scratch[14]) << 112
            | Self::from(scratch[15]) << 120
            | Self::from(scratch[16]) << 128
            | Self::from(scratch[17]) << 136
            | Self::from(scratch[18]) << 144
            | Self::from(scratch[19]) << 152
            | Self::from(scratch[20]) << 160
            | Self::from(scratch[21]) << 168
            | Self::from(scratch[22]) << 176
            | Self::from(scratch[23]) << 184
            | Self::from(scratch[24]) << 192
            | Self::from(scratch[25]) << 200
            | Self::from(scratch[26]) << 208
            | Self::from(scratch[27]) << 216
            | Self::from(scratch[28]) << 224
            | Self::from(scratch[29]) << 232
            | Self::from(scratch[30]) << 240
            | Self::from(scratch[31]) << 248
    }
}

impl Unmarshal<f32> for f32 {
    fn unmarshal(scratch: &[u8]) -> Self {
        let bits = u32::from(scratch[0])
            | u32::from(scratch[1]) << 8
            | u32::from(scratch[2]) << 16
            | u32::from(scratch[3]) << 24;
        Self::from_bits(bits)
    }
}

impl Unmarshal<f64> for f64 {
    fn unmarshal(scratch: &[u8]) -> Self {
        let bits = u64::from(scratch[0])
            | u64::from(scratch[1]) << 8
            | u64::from(scratch[2]) << 16
            | u64::from(scratch[3]) << 24
            | u64::from(scratch[4]) << 32
            | u64::from(scratch[5]) << 40
            | u64::from(scratch[6]) << 48
            | u64::from(scratch[7]) << 56;
        Self::from_bits(bits)
    }
}

impl Unmarshal<bool> for bool {
    fn unmarshal(scratch: &[u8]) -> Self {
        scratch[0] != 0
    }
}
