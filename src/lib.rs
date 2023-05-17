pub mod scalar;
pub mod xsb233;
pub mod xsk233;

use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq};

pub trait Point:
    core::fmt::Debug
    + Clone
    + ConstantTimeEq
    + ConditionallyNegatable
    + ConditionallySelectable
    + Eq
    + Default
{
    fn mulgen(scalar: &scalar::Scalar) -> Self;
    fn neutral() -> &'static Self;
    fn generator() -> &'static Self;

    fn is_neutral(&self) -> Choice;

    fn decode(&mut self, repr: &[u8; 30]) -> Choice;
    fn encode(&self, dst: &mut [u8; 30]);

    fn add(&mut self, lhs: &Self, rhs: &Self);
    fn sub(&mut self, lhs: &Self, rhs: &Self);
    fn neg(&mut self, point: &Self);
    fn double(&mut self, point: &Self);
    fn xdouble(&mut self, point: &Self, n: u32);
    fn mul(&mut self, point: &Self, scalar: &scalar::Scalar);

    // this is a janky way to do it and it might not be very good actually
    // it for sure isn't constant time
    //   well, rejection sampling never is
    // it is not really injective
    // it may even loop forever if you are very unlucky
    fn map_to_curve(data: &[u8; 30]) -> Self {
        let mut data: [u8; 30] = data.clone();
        let mut out = Self::default();
        let mut ctr = 0u8;
        let mut is_valid: bool = false;

        while !is_valid {
            data[0] = data[0] ^ ctr;
            is_valid = out.decode(&data).into();
            data[0] = data[0] ^ ctr;
            ctr += 1;
        }

        out
    }
}

// c-xs233 uses 0xffffffff for true and 0 for false
// to implement constant time functionality. In order
// to be compatible with the rust ecosystem we want to
// use subtle::Choice, which uses 1 and 0, so we need
// to convert between the two (in constant time).

fn to_choice(val: u32) -> Choice {
    let zero_or_one = val.wrapping_neg();
    Choice::from(zero_or_one as u8)
}

fn from_choice(choice: Choice) -> u32 {
    let zero_or_one = choice.unwrap_u8() as u32;
    zero_or_one.wrapping_neg()
}
