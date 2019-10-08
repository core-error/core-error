use core::fmt::{Debug, Display};
#[cfg(rustc_1_7_0)]
use core::any::{TypeId, Any};
use super::typeinfo::TypeInfo;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// `Error` is a trait representing the basic expectations for error values,
/// i.e., values of type `E` in [`Result<T, E>`]. Errors must describe
/// themselves through the [`Display`] and [`Debug`] traits, and may provide
/// cause chain information:
///
/// The [`source`] method is generally used when errors cross "abstraction
/// boundaries". If one module must report an error that is caused by an error
/// from a lower-level module, it can allow access to that error via the
/// [`source`] method. This makes it possible for the high-level module to
/// provide its own errors while also revealing some of the implementation for
/// debugging via [`source`] chains.
///
/// [`Result<T, E>`]: ../result/enum.Result.html
/// [`Display`]: ../fmt/trait.Display.html
/// [`Debug`]: ../fmt/trait.Debug.html
/// [`source`]: trait.Error.html#method.source
pub trait Error: Debug + Display + TypeInfo {
    fn cause(&self) -> Option<&Error> {
        self.source()
    }

    fn source(&self) -> Option<&(Error + 'static)> {
        None
    }
}

// Gated on 1.7.0 because on 1.6.0, Any requires Sized, causing the following
// code to not compile (`self.type_id()` fails because Error + 'static is not
// Sized).
#[cfg(rustc_1_7_0)]
impl Error + 'static {
    pub fn is<T: Error + Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    pub fn downcast_ref<T: Error + Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const Error as *const T))
            }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Error + Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                Some(&mut *(self as *mut Error as *mut T))
            }
        } else {
            None
        }
    }
}

#[cfg(rustc_1_7_0)]
impl Error + 'static + Send {
    pub fn is<T: Error + Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    pub fn downcast_ref<T: Error + Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const Error as *const T))
            }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Error + Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                Some(&mut *(self as *mut Error as *mut T))
            }
        } else {
            None
        }
    }
}

#[cfg(rustc_1_7_0)]
impl Error + 'static + Send + Sync {
    pub fn is<T: Error + Any>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    pub fn downcast_ref<T: Error + Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe {
                Some(&*(self as *const Error as *const T))
            }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Error + Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe {
                Some(&mut *(self as *mut Error as *mut T))
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "alloc")]
impl Error {
    pub fn downcast<T: Error + 'static>(self: Box<Self>) -> Result<Box<T>, Box<Error>> {
        if self.is::<T>() {
            unsafe {
                let raw: *mut Error = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }
}

macro_rules! impl_error {
    ($( #[$version:meta] $ty:path,)*) => {
        $(
            #[$version]
            impl Error for $ty { }
        )*
    };
}

impl_error! {
    #[cfg(rustc_1_0_0)]   ::core::str::ParseBoolError,
    #[cfg(rustc_1_0_0)]   ::core::str::Utf8Error,
    #[cfg(rustc_1_0_0)]   ::core::num::ParseIntError,
    #[cfg(rustc_1_0_0)]   ::core::num::ParseFloatError,
    #[cfg(rustc_1_11_0)]  ::core::fmt::Error,
    #[cfg(rustc_1_13_0)]  ::core::cell::BorrowMutError,
    #[cfg(rustc_1_13_0)]  ::core::cell::BorrowError,
    #[cfg(rustc_1_20_0)]  ::core::char::ParseCharError,
    // Added in 1.27.0 in libcore. Added in 1.9.0 in libstd.
    // Rust is full of lies.
    #[cfg(rustc_1_27_0)]  ::core::char::DecodeUtf16Error,
    #[cfg(rustc_1_28_0)]  ::core::alloc::LayoutErr,
    #[cfg(rustc_1_34_0)]  ::core::num::TryFromIntError,
    #[cfg(rustc_1_34_0)]  ::core::array::TryFromSliceError,
    #[cfg(rustc_1_34_0)]  ::core::char::CharTryFromError,

    // This implementation is actually ParseError in disguise. ParseError is a
    // type alias to Infallible. In order to avoid having the alloc feature
    // toggling the error impl on Infallible (which would be confusing), we will
    // just unconditionally impl it here.
    #[cfg(rustc_1_34_0)]  ::core::convert::Infallible,
}

#[cfg(feature = "alloc")]
impl_error! {
    #[cfg(rustc_1_36_0)] ::alloc::string::FromUtf16Error,
    #[cfg(rustc_1_36_0)] ::alloc::string::FromUtf8Error,
}

#[cfg(feature = "alloc")]
impl<T: Error> Error for Box<T> {
    fn description(&self) -> &str {
        Error::description(&**self)
    }

    fn cause(&self) -> Option<&Error> {
        Error::cause(&**self)
    }

    fn source(&self) -> Option<&(Error + 'static)> {
        Error::source(&**self)
    }
}

#[cfg(test)]
mod test {
    // Ensure that ParseError implements Error
    #[cfg(all(rustc_1_36_0, feature = "alloc"))]
    const _ASSERT_PARSE_ERROR_IMPLS_ERROR: fn() = || {
        fn assert_impl<T: ?Sized + super::Error>() {}
        assert_impl::<::alloc::string::ParseError>();
    };
}
