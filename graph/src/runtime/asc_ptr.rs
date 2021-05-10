use super::DeterministicHostError;

use super::{AscHeap, AscType, IndexForAscTypeId};
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;

/// A pointer to an object in the Asc heap.
pub struct AscPtr<C>(u32, PhantomData<C>);

impl<T> Copy for AscPtr<T> {}

impl<T> Clone for AscPtr<T> {
    fn clone(&self) -> Self {
        AscPtr(self.0, PhantomData)
    }
}

impl<T> Default for AscPtr<T> {
    fn default() -> Self {
        AscPtr(0, PhantomData)
    }
}

impl<T> fmt::Debug for AscPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<C> AscPtr<C> {
    /// A raw pointer to be passed to Wasm.
    pub fn wasm_ptr(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn new(heap_ptr: u32) -> Self {
        Self(heap_ptr, PhantomData)
    }
}

impl<C: AscType> AscPtr<C> {
    /// Create a pointer that is equivalent to AssemblyScript's `null`.
    #[inline(always)]
    pub fn null() -> Self {
        AscPtr::new(0)
    }

    /// Read from `self` into the Rust struct `C`.
    pub fn read_ptr<H: AscHeap>(self, heap: &H) -> Result<C, DeterministicHostError> {
        let bytes = heap.get(self.0, C::asc_size(self, heap)?)?;
        C::from_asc_bytes(&bytes)
    }

    /// Allocate `asc_obj` as an Asc object of class `C`.
    pub fn alloc_obj<H: AscHeap>(
        asc_obj: C,
        heap: &mut H,
    ) -> Result<AscPtr<C>, DeterministicHostError> {
        let bytes = asc_obj.to_asc_bytes()?;
        let header = if let Some(type_id_index) = C::INDEX_ASC_TYPE_ID {
            Self::generate_header(heap, type_id_index, bytes.len() as u32)
        } else {
            vec![]
        };

        let header_len = header.len() as u32;

        let heap_ptr = heap.raw_new(&[header, bytes].concat())?;

        // Use header length as offset. so the AscPtr points directly at the content.
        Ok(AscPtr::new(heap_ptr + header_len))
    }

    fn generate_header<H: AscHeap>(
        heap: &mut H,
        type_id_index: IndexForAscTypeId,
        content_length: u32,
    ) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::with_capacity(20);

        let mm_info: [u8; 4] = (0u32).to_le_bytes();
        let gc_info: [u8; 4] = (0u32).to_le_bytes();
        let gc_info2: [u8; 4] = (0u32).to_le_bytes();
        let asc_type_id = heap.asc_type_id(type_id_index);
        let rt_id: [u8; 4] = asc_type_id.to_le_bytes();
        let rt_size: [u8; 4] = content_length.to_le_bytes();

        header.extend(&mm_info);
        header.extend(&gc_info);
        header.extend(&gc_info2);
        header.extend(&rt_id);
        header.extend(&rt_size);

        header
    }

    /// Helper used by arrays and strings to read their length.
    pub fn read_u32<H: AscHeap>(&self, heap: &H) -> Result<u32, DeterministicHostError> {
        // Read the bytes pointed to by `self` as the bytes of a `u32`.
        let raw_bytes = heap.get(self.0, size_of::<u32>() as u32)?;
        let mut u32_bytes: [u8; size_of::<u32>()] = [0; size_of::<u32>()];
        u32_bytes.copy_from_slice(&raw_bytes);
        Ok(u32::from_le_bytes(u32_bytes))
    }

    /// Conversion to `u64` for use with `AscEnum`.
    pub fn to_payload(&self) -> u64 {
        self.0 as u64
    }

    /// We typically assume `AscPtr` is never null, but for types such as `string | null` it can be.
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }

    // Erase type information.
    pub fn erase(self) -> AscPtr<()> {
        AscPtr::new(self.0)
    }
}

impl<C> From<u32> for AscPtr<C> {
    fn from(ptr: u32) -> Self {
        AscPtr::new(ptr)
    }
}

impl<T> AscType for AscPtr<T> {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        self.0.to_asc_bytes()
    }

    fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
        let bytes = u32::from_asc_bytes(asc_obj)?;
        Ok(AscPtr::new(bytes))
    }
}
