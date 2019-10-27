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
//!
//! # Implementation note
//!
//! In the old versions of rust, `TypeId::of` was gated on Reflect. This means
//! it could only be used on types that implement `Any`. In 1.13.0, `TypeId::of`
//! was changed to only require `T: 'static`. The two are functionally
//! equivalent, since `Any` is implemented on every `T: 'static`.
//!
//! To stay backward compatible, we originally used `T: Any` on every compiler.
//! But a new rust lint, [where_clauses_object_safety], broke this approach.
//! This lint warns when casting a trait with a `where` clause into a `dyn`
//! object. Indeed, the compiler can't prove such traits are object-safe.
//!
//! In our case though, it is pretty trivial to prove. Since `Any` is
//! implemented for every `T: 'static`, and only `T: 'static` can be turned into
//! `dyn T`, it's immediately obvious that it should work. Alas, the rust
//! compiler isn't smart enough to distinguish those cases, and causes warns in
//! downstream crate users.
//!
//! To avoid this, we have two implementations of `type_info` based on the rust
//! compiler version. On versions older than 1.13, we use `Self: Any`. At that
//! point, the compiler didn't have the new lint, so the problem is avoided.
//! On 1.13 and newer, we use `Self: 'static`, which doesn't trigger the lint.
//!
//! [where_clauses_object_safety]: https://github.com/rust-lang/rust/issues/51443

#[cfg(not(rustc_1_13_0))]
use core::any::{Any, TypeId};

/// Unsafe trait to get the type_id of 'static types. Implemented for all types, so it can be
/// easily added as a supertrait.
pub unsafe trait TypeInfo {
    /// Gets the `TypeId` of `self`.
    #[cfg(rustc_1_13_0)]
    fn type_id(&self) -> TypeId
    where
        Self: 'static,
    {
        TypeId::of::<Self>()
    }

    /// Gets the `TypeId` of `self`.
    #[cfg(not(rustc_1_13_0))]
    fn type_id(&self) -> TypeId
    where
        Self: Any,
    {
        TypeId::of::<Self>()
    }
}

unsafe impl<T: ?Sized> TypeInfo for T {}
