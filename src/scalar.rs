use std::{ffi::c_void, ops::Add};

#[derive(PartialEq, Eq, Clone)]
pub struct Scalar([u8; 30]);

impl Scalar {
    pub fn new(le_bytes: [u8; 30]) -> Self {
        Self(le_bytes)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn as_void_ptr(&self) -> *const c_void {
        let scalar_ptr: *const Scalar = self;
        scalar_ptr.cast()
    }
}

impl Add for Scalar {
    type Output = Self;

    // this is not constant time
    fn add(self, rhs: Self) -> Self::Output {
        let Self(self_bytes) = self;
        let Self(rhs_bytes) = rhs;

        let mut out = [0u8; 30];
        let mut carry = false;

        for i in 0..30 {
            if carry {
                out[i] += 1;
            }
            (out[i], carry) = u8::overflowing_add(self_bytes[i], rhs_bytes[i]);
        }

        Self(out)
    }
}
