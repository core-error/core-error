use ::core::any::TypeId;

/// Unsafe trait to get the type_id of 'static types. Implemented for all types, so it can be
/// easily added as a supertrait.
pub unsafe trait TypeInfo {
    fn type_id(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }

    #[cfg(rust_1_38_0)]
    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
}

unsafe impl<T: ?Sized> TypeInfo for T {}
