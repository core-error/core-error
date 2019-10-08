use ::core::any::{TypeId, Any};

/// Unsafe trait to get the type_id of 'static types. Implemented for all types, so it can be
/// easily added as a supertrait.
pub unsafe trait TypeInfo {
    // The bound is slightly different from the RFC in order to make it work on
    // as many rust versions as possible.
    //
    // Any is functionally equivalent to 'static, but on Rust 1.6.0,
    // `TypeId::of` requires the marker trait Reflect. This trait is implemented
    // on all types that impl Any, and Any is impl'd for every type where T:
    // 'static.
    fn type_id(&self) -> TypeId where Self: Any {
        TypeId::of::<Self>()
    }

    #[cfg(rust_1_38_0)]
    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

unsafe impl<T: ?Sized> TypeInfo for T {}
