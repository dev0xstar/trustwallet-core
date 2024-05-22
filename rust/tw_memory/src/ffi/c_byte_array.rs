// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#[repr(C)]
pub struct CByteArrayResult {
    pub code: i32,
    pub result: CByteArray,
}

crate::impl_c_result!(CByteArrayResult, CByteArray, CByteArray::null());

/// A C-compatible wrapper over a byte array allocated in Rust.
#[repr(C)]
#[derive(Debug)]
pub struct CByteArray {
    data: *mut u8,
    size: usize,
    capacity: usize,
}

impl Drop for CByteArray {
    fn drop(&mut self) {
        unsafe { self.release_if_allocated() };
    }
}

impl From<Vec<u8>> for CByteArray {
    fn from(data: Vec<u8>) -> Self {
        CByteArray::new(data)
    }
}

impl CByteArray {
    /// Returns a null `CByteArray` instance.
    pub fn null() -> CByteArray {
        CByteArray {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }

    /// Returns a `CByteArray` instance from the given `mut_vec` bytes.
    pub fn new(mut mut_vec: Vec<u8>) -> CByteArray {
        let data = mut_vec.as_mut_ptr();
        let size = mut_vec.len();
        let capacity = mut_vec.capacity();
        std::mem::forget(mut_vec);
        CByteArray {
            data,
            size,
            capacity,
        }
    }

    /// Converts `CByteArray` into `Vec<u8>` without additional allocation.
    ///
    /// # Safe
    ///
    /// `data`, `size` and `capacity` must be valid and exactly the same as after [`CByteArray::new`].
    pub unsafe fn into_vec(mut self) -> Vec<u8> {
        let data = Vec::from_raw_parts(self.data, self.size, self.capacity);
        // Set the self to null to avoid realising the memory on `CByteArray::drop`.
        self.set_null();
        data
    }

    /// Releases the memory if it was allocated previously and not released yet.
    unsafe fn release_if_allocated(&mut self) {
        // Do nothing if the memory has been released already or not allocated yet.
        if self.data.is_null() {
            return;
        }

        let _ = Vec::from_raw_parts(self.data, self.size, self.capacity);
        self.set_null();
    }

    /// Set the pointer, size and capacity to the default values.
    fn set_null(&mut self) {
        self.data = std::ptr::null_mut();
        self.size = 0;
        self.capacity = 0;
    }

    /// Returns the const pointer to the data.
    pub fn data(&self) -> *const u8 {
        self.data.cast_const()
    }

    /// Returns the data length.
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Releases the memory previously allocated for the pointer to `CByteArray`.
/// \param ptr *non-null* C-compatible byte array.
#[no_mangle]
pub unsafe extern "C" fn free_c_byte_array(ptr: *mut CByteArray) {
    if ptr.is_null() {
        return;
    }
    (*ptr).release_if_allocated();
}
