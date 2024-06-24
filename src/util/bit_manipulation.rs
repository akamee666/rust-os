pub trait GetBits {
    fn get_bits(&self, shift: Self, length: Self) -> Self;
    fn get_bit(&self, shift: Self) -> Self;
}

pub trait SetBits {
    fn set_bits(&mut self, shift: Self, length: Self, val: Self);
    fn set_bit(&mut self, shift: Self, enable: bool);
}

macro_rules! impl_set_bits {
    ($t:ty) => {
        impl SetBits for $t {
            fn set_bits(&mut self, shift: Self, length: Self, val: Self) {
                let mut mask = (1 << length) - 1;
                mask <<= shift;

                *self &= !mask;
                *self |= (val << shift) & mask;
            }

            fn set_bit(&mut self, shift: Self, enable: bool) {
                self.set_bits(shift, 1, enable as Self);
            }
        }
    };
}

impl_set_bits!(u8);
impl_set_bits!(u16);
impl_set_bits!(u32);
impl_set_bits!(u64);

macro_rules! impl_get_bits {
    ($t:ty) => {
        impl GetBits for $t {
            fn get_bits(&self, shift: Self, length: Self) -> Self {
                let mask = (1 << length) - 1;
                (*self >> shift) & mask
            }

            fn get_bit(&self, shift: Self) -> Self {
                self.get_bits(shift, 1)
            }
        }
    };
}

impl_get_bits!(u8);
impl_get_bits!(u16);
impl_get_bits!(u32);
impl_get_bits!(u64);
