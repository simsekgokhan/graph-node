//! Facilities for creating and reading objects on the memory of an AssemblyScript (Asc) WASM
//! module. Objects are passed through the `asc_new` and `asc_get` methods of an `AscHeap`
//! implementation. These methods take types that implement `To`/`FromAscObj` and are therefore
//! convertible to/from an `AscType`.

mod asc_heap;
mod asc_ptr;

pub use asc_heap::{AscHeap, FromAscObj, ToAscObj, TryFromAscObj};
pub use asc_ptr::AscPtr;

use anyhow::Error;
use std::convert::TryInto;
use std::fmt;
use std::mem::size_of;

/// A type that has a direct correspondence to an Asc type.
///
/// This can be derived for structs that are `#[repr(C)]`, contain no padding
/// and whose fields are all `AscValue`. Enums can derive if they are `#[repr(u32)]`.
///
/// Special classes like `ArrayBuffer` use custom impls.
///
/// See https://github.com/graphprotocol/graph-node/issues/607 for more considerations.
pub trait AscType: Sized {
    /// Constant string with the name of the type in AssemblyScript.
    /// This is used to get the identifier for the type in memory layout.
    /// Info about memory layout:
    /// https://www.assemblyscript.org/memory.html#common-header-layout.
    /// Info about identifier (`idof<T>`):
    /// https://www.assemblyscript.org/garbage-collection.html#runtime-interface
    const INDEX_ASC_TYPE_ID: Option<IndexForAscTypeId> = None;

    /// Transform the Rust representation of this instance into an sequence of
    /// bytes that is precisely the memory layout of a corresponding Asc instance.
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError>;

    /// The Rust representation of an Asc object as layed out in Asc memory.
    fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError>;

    /// Size of the corresponding Asc instance in bytes.
    fn asc_size<H: AscHeap>(ptr: AscPtr<Self>, heap: &H) -> Result<u32, DeterministicHostError> {
        // If its a class with the common header
        if Self::INDEX_ASC_TYPE_ID.is_some() {
            let header = heap.get(ptr.wasm_ptr() - 20, 20)?;
            let rt_size = header.get(16..20).unwrap(); // safe unwrap because line above already errors on out of bounds access
            Ok(u32::from_le_bytes(rt_size.try_into().unwrap())) // safe unwrap because line above already gets a 4 sized u8 array
        } else {
            Ok(std::mem::size_of::<Self>() as u32)
        }
    }
}

// Only implemented because of structs that derive AscType and
// contain fields that are PhantomData.
impl<T> AscType for std::marker::PhantomData<T> {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        Ok(vec![])
    }

    fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
        assert!(asc_obj.len() == 0);

        Ok(Self)
    }
}

/// An Asc primitive or an `AscPtr` into the Asc heap. A type marked as
/// `AscValue` must have the same byte representation in Rust and Asc, including
/// same size, and size must be equal to alignment.
pub trait AscValue: AscType + Copy + Default {}

impl AscType for bool {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        Ok(vec![*self as u8])
    }

    fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
        if asc_obj.len() != 1 {
            Err(DeterministicHostError(anyhow::anyhow!(
                "Incorrect size for bool. Expected 1, got {},",
                asc_obj.len()
            )))
        } else {
            Ok(asc_obj[0] != 0)
        }
    }
}

impl AscValue for bool {}
impl<T> AscValue for AscPtr<T> {}

macro_rules! impl_asc_type {
    ($($T:ty),*) => {
        $(
            impl AscType for $T {
                fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
                    Ok(self.to_le_bytes().to_vec())
                }

                fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
                    let bytes = asc_obj.try_into().map_err(|_| {
                        DeterministicHostError(anyhow::anyhow!(
                            "Incorrect size for {}. Expected {}, got {},",
                            stringify!($T),
                            size_of::<Self>(),
                            asc_obj.len()
                        ))
                    })?;

                    Ok(Self::from_le_bytes(bytes))
                }
            }

            impl AscValue for $T {}
        )*
    };
}

impl_asc_type!(u8, u16, u32, u64, i8, i32, i64, f32, f64);

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum IndexForAscTypeId {
    String,
}

impl AscType for IndexForAscTypeId {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        (*self as u32).to_asc_bytes()
    }

    fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
        let mut u32_bytes: [u8; size_of::<u32>()] = [0; size_of::<u32>()];
        if std::mem::size_of_val(&u32_bytes) != std::mem::size_of_val(&asc_obj) {
            return Err(DeterministicHostError(anyhow::anyhow!(
                "Invalid asc bytes size"
            )));
        }
        u32_bytes.copy_from_slice(&asc_obj);
        let discr = u32::from_le_bytes(u32_bytes);
        match discr {
            0u32 => Ok(Self::String),
            _ => Err(DeterministicHostError(anyhow::anyhow!(
                "value {} is out of range for {}",
                discr,
                "IndexForAscTypeId"
            ))),
        }
    }
}

impl ToAscObj<u32> for IndexForAscTypeId {
    fn to_asc_obj<H: AscHeap>(&self, _heap: &mut H) -> Result<u32, DeterministicHostError> {
        Ok(*self as u32)
    }
}

#[derive(Debug)]
pub struct DeterministicHostError(pub Error);

impl fmt::Display for DeterministicHostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DeterministicHostError {}
