#[cfg(target_arch = "x86_64")]
pub mod avx512_m31;
pub mod cm31;
pub mod m31;
pub mod qm31;

#[macro_export]
macro_rules! impl_field {
    ($field_name: ty, $field_size: ident) => {
        impl Num for $field_name {
            type FromStrRadixErr = Box<dyn std::error::Error>;

            fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                unimplemented!(
                    "Num::from_str_radix is not implemented for {}",
                    stringify!($field_name)
                );
            }
        }

        impl $field_name {
            pub fn square(&self) -> Self {
                (*self) * (*self)
            }

            pub fn double(&self) -> Self {
                (*self) + (*self)
            }

            pub fn pow(&self, exp: u128) -> Self {
                let mut res = Self::one();
                let mut base = *self;
                let mut exp = exp;
                while exp > 0 {
                    if exp & 1 == 1 {
                        res *= base;
                    }
                    base = base.square();
                    exp >>= 1;
                }
                res
            }

            pub fn inverse(&self) -> Self {
                assert!(*self != Self::zero(), "0 has no inverse");
                self.pow(($field_size - 2) as u128)
            }
        }

        impl AddAssign for $field_name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl SubAssign for $field_name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl MulAssign for $field_name {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl Div for $field_name {
            type Output = Self;

            #[allow(clippy::suspicious_arithmetic_impl)]
            fn div(self, rhs: Self) -> Self::Output {
                self * rhs.inverse()
            }
        }

        impl DivAssign for $field_name {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        impl Rem for $field_name {
            type Output = Self;
            fn rem(self, _rhs: Self) -> Self::Output {
                unimplemented!("Rem is not implemented for {}", stringify!($field_name));
            }
        }

        impl RemAssign for $field_name {
            fn rem_assign(&mut self, _rhs: Self) {
                unimplemented!(
                    "RemAssign is not implemented for {}",
                    stringify!($field_name)
                );
            }
        }
    };
}
