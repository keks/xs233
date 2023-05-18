use std::ffi::c_void;

#[derive(PartialEq, Eq, Clone, Debug)]
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

    pub(crate) fn as_bytes(&self) -> &[u8; 30] {
        &self.0
    }
}
