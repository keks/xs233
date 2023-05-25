pub mod scalar;
pub mod xsb233;
pub mod xsk233;

pub trait Point:
    core::fmt::Debug
    + Default
    + Clone
    + subtle::ConstantTimeEq
    + subtle::ConditionallyNegatable
    + subtle::ConditionallySelectable
{
    type EncodedPoint;
    fn decode(&mut self, repr: &Self::EncodedPoint) -> subtle::Choice;
    fn encode(&self, dst: &mut Self::EncodedPoint);

    fn mulgen<const N: usize>(scalar: &scalar::Scalar<N>) -> Self;
    fn neutral() -> &'static Self;
    fn generator() -> &'static Self;

    fn is_neutral(&self) -> subtle::Choice;

    fn mul<const N: usize>(&mut self, point: &Self, scalar: &scalar::Scalar<N>);
    fn mul_inplace<const N: usize>(&mut self, scalar: &scalar::Scalar<N>);
    fn add(&mut self, lhs: &Self, rhs: &Self);
    fn add_assign(&mut self, rhs: &Self);
    fn sub(&mut self, lhs: &Self, rhs: &Self);
    fn sub_assign(&mut self, rhs: &Self);
    fn neg(&mut self, point: &Self);
    fn neg_inplace(&mut self);
    fn double(&mut self, point: &Self);
    fn double_inplace(&mut self);
    fn xdouble(&mut self, point: &Self, n: u32);
    fn xdouble_inplace(&mut self, n: u32);
}

// this is a janky way to do it and it might not be very good actually
// it for sure isn't constant time
//   well, rejection sampling never is
// it is not really injective
// it may even loop forever if you are very unlucky
pub fn map_uniform_bytes_to_curve<const L: usize, P: Point<EncodedPoint = [u8; L]>>(
    mut data: [u8; L],
) -> P {
    let mut out = P::default();
    let mut ctr = 0u8;
    let mut is_valid: bool = false;

    // this works for the two points in this crate but is not truly generic
    data[L - 1] &= 1;

    while !is_valid {
        data[0] ^= ctr;
        is_valid = out.decode(&data).into();
        data[0] ^= ctr;
        ctr += 1;
    }

    out
}

// The macro impl_ops! implements arithmetic and comparison for types that implement Point
#[macro_export]
macro_rules! impl_ops {
    ($type:ty) => {
        impl ::core::ops::Add for $type {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self.add_assign(&rhs);
                self
            }
        }

        impl ::core::ops::Add for &$type {
            type Output = $type;

            fn add(self, rhs: Self) -> Self::Output {
                let mut out = <$type as ::core::default::Default>::default();
                <$type as $crate::Point>::add(&mut out, self, rhs);
                out
            }
        }

        impl ::core::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                <Self as $crate::Point>::add_assign(self, &rhs);
            }
        }

        impl ::core::ops::Sub for $type {
            type Output = Self;

            fn sub(mut self, rhs: Self) -> Self::Output {
                self.sub_assign(&rhs);
                self
            }
        }

        impl ::core::ops::Sub for &$type {
            type Output = $type;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut out = <$type as ::core::default::Default>::default();
                <$type as $crate::Point>::sub(&mut out, self, rhs);
                out
            }
        }

        impl ::core::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                <Self as $crate::Point>::sub_assign(self, &rhs);
            }
        }

        impl ::core::ops::Neg for $type {
            type Output = Self;

            fn neg(mut self) -> Self::Output {
                self.neg_inplace();
                self
            }
        }

        impl<const N: usize> ::core::ops::Mul<$type> for $crate::scalar::Scalar<N> {
            type Output = $type;

            fn mul(self, mut rhs: $type) -> $type {
                rhs.mul_inplace(&self);
                rhs
            }
        }

        impl<const N: usize> ::core::ops::Mul<$type> for &$crate::scalar::Scalar<N> {
            type Output = $type;

            fn mul(self, mut rhs: $type) -> $type {
                rhs.mul_inplace(self);
                rhs
            }
        }

        impl<const N: usize> ::core::ops::Mul<&$type> for $crate::scalar::Scalar<N> {
            type Output = $type;

            fn mul(self, rhs: &$type) -> $type {
                let mut out = <$type as ::core::default::Default>::default();
                <$type as $crate::Point>::mul(&mut out, rhs, &self);
                out
            }
        }

        impl<const N: usize> ::core::ops::Mul<&$type> for &$crate::scalar::Scalar<N> {
            type Output = $type;

            fn mul(self, rhs: &$type) -> $type {
                let mut out = <$type as ::core::default::Default>::default();
                <$type as $crate::Point>::mul(&mut out, rhs, &self);
                out
            }
        }

        impl ::core::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.ct_eq(other).unwrap_u8() == 1
            }
        }

        impl ::core::cmp::Eq for $type {}
    };
}

// c-xs233 uses 0xffffffff for true and 0 for false
// to implement constant time functionality. In order
// to be compatible with the rust ecosystem we want to
// use subtle::Choice, which uses 1 and 0, so we need
// to convert between the two (in constant time).

fn to_choice(val: u32) -> subtle::Choice {
    let zero_or_one = val.wrapping_neg();
    subtle::Choice::from(zero_or_one as u8)
}

fn from_choice(choice: subtle::Choice) -> u32 {
    let zero_or_one = choice.unwrap_u8() as u32;
    zero_or_one.wrapping_neg()
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn map_to_curve_100k_elements() {
        let mut buf = [0u8; 30];
        let mut rng = ChaCha8Rng::from_seed([23u8; 32]);

        for _ in 0..100_000 {
            rng.fill(&mut buf);
            let _: crate::xsk233::Xsk233Point = crate::map_uniform_bytes_to_curve(buf);
        }
    }
}
