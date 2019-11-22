//! Trait allowing getting the `type_id` of everything.
//!
//! This module implements a trait, `TypeInfo`, that's implemented on every
//! type. This trait has a single `type_id` function, which allows getting the
//! `TypeId` of that type. This function cannot be implemented manually thanks
//! to the blanket implementation, ensuring it is safe to rely on its value.
//!
//! The intended usage is as a super-trait for traits like `Error` that need a
//! safe way to get the type_id.
//!
//! See [RFC 2738 for more information](https://github.com/rust-lang/rfcs/pull/2738)

#[cfg(not(rustc_1_13_0))]
use core::any::Any;
use core::any::TypeId;

/// Unsafe trait to get the type_id of 'static types. Implemented for all types, so it can be
/// easily added as a supertrait.
pub unsafe trait TypeInfo {
    /// Gets the `TypeId` of `self`.
    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }
}

unsafe impl<T: ?Sized> TypeInfo for T {}
