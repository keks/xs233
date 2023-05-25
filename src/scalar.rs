use std::ffi::c_void;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Scalar<const N: usize>([u8; N]);

impl<const N: usize> Scalar<N> {
    pub fn new(le_bytes: [u8; N]) -> Self {
        Self(le_bytes)
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn as_void_ptr(&self) -> *const c_void {
        let scalar_ptr: *const Self = self;
        scalar_ptr.cast()
    }

    pub fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }
}
