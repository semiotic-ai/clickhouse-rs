use super::{i256, u256};

pub trait Marshal {
    fn marshal(&self, scratch: &mut [u8]);
}

impl Marshal for u8 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self;
    }
}

impl Marshal for u16 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
    }
}

impl Marshal for u32 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
        scratch[2] = (self >> 16) as u8;
        scratch[3] = (self >> 24) as u8;
    }
}

impl Marshal for u64 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
        scratch[2] = (self >> 16) as u8;
        scratch[3] = (self >> 24) as u8;

        scratch[4] = (self >> 32) as u8;
        scratch[5] = (self >> 40) as u8;
        scratch[6] = (self >> 48) as u8;
        scratch[7] = (self >> 56) as u8;
    }
}

impl Marshal for u128 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
        scratch[2] = (self >> 16) as u8;
        scratch[3] = (self >> 24) as u8;

        scratch[4] = (self >> 32) as u8;
        scratch[5] = (self >> 40) as u8;
        scratch[6] = (self >> 48) as u8;
        scratch[7] = (self >> 56) as u8;

        scratch[8] = (self >> 64) as u8;
        scratch[9] = (self >> 72) as u8;
        scratch[10] = (self >> 80) as u8;
        scratch[11] = (self >> 88) as u8;

        scratch[12] = (self >> 96) as u8;
        scratch[13] = (self >> 104) as u8;
        scratch[14] = (self >> 112) as u8;
        scratch[15] = (self >> 120) as u8;
    }
}

impl Marshal for u256 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = self.0[0] as u8;
        scratch[1] = (self.0[0] >> 8) as u8;
        scratch[2] = (self.0[0] >> 16) as u8;
        scratch[3] = (self.0[0] >> 24) as u8;

        scratch[4] = (self.0[0] >> 32) as u8;
        scratch[5] = (self.0[0] >> 40) as u8;
        scratch[6] = (self.0[0] >> 48) as u8;
        scratch[7] = (self.0[0] >> 56) as u8;

        scratch[8] = (self.0[0] >> 64) as u8;
        scratch[9] = (self.0[0] >> 72) as u8;
        scratch[10] = (self.0[0] >> 80) as u8;
        scratch[11] = (self.0[0] >> 88) as u8;

        scratch[12] = (self.0[0] >> 96) as u8;
        scratch[13] = (self.0[0] >> 104) as u8;
        scratch[14] = (self.0[0] >> 112) as u8;
        scratch[15] = (self.0[0] >> 120) as u8;

        scratch[16] = self.0[1] as u8;
        scratch[17] = (self.0[1] >> 8) as u8;
        scratch[18] = (self.0[1] >> 16) as u8;
        scratch[19] = (self.0[1] >> 24) as u8;

        scratch[20] = (self.0[1] >> 32) as u8;
        scratch[21] = (self.0[1] >> 40) as u8;
        scratch[22] = (self.0[1] >> 48) as u8;
        scratch[23] = (self.0[1] >> 56) as u8;

        scratch[24] = (self.0[1] >> 64) as u8;
        scratch[25] = (self.0[1] >> 72) as u8;
        scratch[26] = (self.0[1] >> 80) as u8;
        scratch[27] = (self.0[1] >> 88) as u8;

        scratch[28] = (self.0[1] >> 96) as u8;
        scratch[29] = (self.0[1] >> 104) as u8;
        scratch[30] = (self.0[1] >> 112) as u8;
        scratch[31] = (self.0[1] >> 120) as u8;
    }
}

impl Marshal for i8 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
    }
}

impl Marshal for i16 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
    }
}

impl Marshal for i32 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
        scratch[2] = (self >> 16) as u8;
        scratch[3] = (self >> 24) as u8;
    }
}

impl Marshal for i64 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
        scratch[2] = (self >> 16) as u8;
        scratch[3] = (self >> 24) as u8;

        scratch[4] = (self >> 32) as u8;
        scratch[5] = (self >> 40) as u8;
        scratch[6] = (self >> 48) as u8;
        scratch[7] = (self >> 56) as u8;
    }
}

impl Marshal for i128 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
        scratch[1] = (self >> 8) as u8;
        scratch[2] = (self >> 16) as u8;
        scratch[3] = (self >> 24) as u8;

        scratch[4] = (self >> 32) as u8;
        scratch[5] = (self >> 40) as u8;
        scratch[6] = (self >> 48) as u8;
        scratch[7] = (self >> 56) as u8;

        scratch[8] = (self >> 64) as u8;
        scratch[9] = (self >> 72) as u8;
        scratch[10] = (self >> 80) as u8;
        scratch[11] = (self >> 88) as u8;

        scratch[12] = (self >> 96) as u8;
        scratch[13] = (self >> 104) as u8;
        scratch[14] = (self >> 112) as u8;
        scratch[15] = (self >> 120) as u8;
    }
}

impl Marshal for i256 {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = self.0[0] as u8;
        scratch[1] = (self.0[0] >> 8) as u8;
        scratch[2] = (self.0[0] >> 16) as u8;
        scratch[3] = (self.0[0] >> 24) as u8;

        scratch[4] = (self.0[0] >> 32) as u8;
        scratch[5] = (self.0[0] >> 40) as u8;
        scratch[6] = (self.0[0] >> 48) as u8;
        scratch[7] = (self.0[0] >> 56) as u8;

        scratch[8] = (self.0[0] >> 64) as u8;
        scratch[9] = (self.0[0] >> 72) as u8;
        scratch[10] = (self.0[0] >> 80) as u8;
        scratch[11] = (self.0[0] >> 88) as u8;

        scratch[12] = (self.0[0] >> 96) as u8;
        scratch[13] = (self.0[0] >> 104) as u8;
        scratch[14] = (self.0[0] >> 112) as u8;
        scratch[15] = (self.0[0] >> 120) as u8;

        scratch[16] = self.0[1] as u8;
        scratch[17] = (self.0[1] >> 8) as u8;
        scratch[18] = (self.0[1] >> 16) as u8;
        scratch[19] = (self.0[1] >> 24) as u8;

        scratch[20] = (self.0[1] >> 32) as u8;
        scratch[21] = (self.0[1] >> 40) as u8;
        scratch[22] = (self.0[1] >> 48) as u8;
        scratch[23] = (self.0[1] >> 56) as u8;

        scratch[24] = (self.0[1] >> 64) as u8;
        scratch[25] = (self.0[1] >> 72) as u8;
        scratch[26] = (self.0[1] >> 80) as u8;
        scratch[27] = (self.0[1] >> 88) as u8;

        scratch[28] = (self.0[1] >> 96) as u8;
        scratch[29] = (self.0[1] >> 104) as u8;
        scratch[30] = (self.0[1] >> 112) as u8;
        scratch[31] = (self.0[1] >> 120) as u8;
    }
}

impl Marshal for f32 {
    fn marshal(&self, scratch: &mut [u8]) {
        let bits = self.to_bits();
        scratch[0] = bits as u8;
        scratch[1] = (bits >> 8) as u8;
        scratch[2] = (bits >> 16) as u8;
        scratch[3] = (bits >> 24) as u8;
    }
}

impl Marshal for f64 {
    fn marshal(&self, scratch: &mut [u8]) {
        let bits = self.to_bits();
        scratch[0] = bits as u8;
        scratch[1] = (bits >> 8) as u8;
        scratch[2] = (bits >> 16) as u8;
        scratch[3] = (bits >> 24) as u8;
        scratch[4] = (bits >> 32) as u8;
        scratch[5] = (bits >> 40) as u8;
        scratch[6] = (bits >> 48) as u8;
        scratch[7] = (bits >> 56) as u8;
    }
}

impl Marshal for bool {
    fn marshal(&self, scratch: &mut [u8]) {
        scratch[0] = *self as u8;
    }
}

#[cfg(test)]
mod test {
    use std::fmt;

    use crate::types::{Marshal, StatBuffer, Unmarshal};
    use rand::distributions::{Distribution, Standard};
    use rand::random;

    fn test_some<T>()
    where
        T: Copy + fmt::Debug + StatBuffer + Marshal + Unmarshal<T> + PartialEq,
        Standard: Distribution<T>,
    {
        for _ in 0..100 {
            let mut buffer = T::buffer();
            let v = random::<T>();

            v.marshal(buffer.as_mut());
            let u = T::unmarshal(buffer.as_ref());

            assert_eq!(v, u);
        }
    }

    #[test]
    fn test_u8() {
        test_some::<u8>()
    }

    #[test]
    fn test_u16() {
        test_some::<u16>()
    }

    #[test]
    fn test_u32() {
        test_some::<u32>()
    }

    #[test]
    fn test_u64() {
        test_some::<u64>()
    }

    #[test]
    fn test_u128() {
        test_some::<u128>()
    }

    #[test]
    fn test_i8() {
        test_some::<i8>()
    }

    #[test]
    fn test_i16() {
        test_some::<i16>()
    }

    #[test]
    fn test_i32() {
        test_some::<i32>()
    }

    #[test]
    fn test_i64() {
        test_some::<i64>()
    }

    #[test]
    fn test_i128() {
        test_some::<i128>()
    }

    #[test]
    fn test_f32() {
        test_some::<f32>()
    }

    #[test]
    fn test_f64() {
        test_some::<f64>()
    }

    #[test]
    fn test_bool() {
        test_some::<bool>()
    }
}
