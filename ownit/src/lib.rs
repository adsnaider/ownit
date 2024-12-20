#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "derive")]
pub use ownit_derive::Ownit;

/// Trait akin to [`ToOwned`] but more general in practice.
pub trait Ownit {
    /// Owned version of self.
    ///
    /// Generally this is going to be the type itself but with 'static lifetimes
    /// (e.g. `Cow<'a, T> -> Cow<'static, T>`)
    type OwnedSelf: 'static;

    /// Makes an owned (`'static`) version of `self`
    fn into_static(self) -> Self::OwnedSelf;
}

mod impls {
    use super::Ownit;
    use std::{
        borrow::Cow,
        path::PathBuf,
        rc::Rc,
        sync::atomic::{
            AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
            AtomicU64, AtomicU8, AtomicUsize,
        },
        time::{Duration, Instant, SystemTime},
    };

    macro_rules! blanket_owned {
        ($ty:ident) => {
            impl Ownit for $ty {
                type OwnedSelf = $ty;

                fn into_static(self) -> Self::OwnedSelf {
                    self
                }
            }
        };
        ($ty:ident<..>) => {
            impl<T: 'static> Ownit for $ty<T> {
                type OwnedSelf = $ty<T>;

                fn into_static(self) -> Self::OwnedSelf {
                    self
                }
            }
        };
    }

    impl<T: ToOwned + ?Sized + 'static> Ownit for Cow<'_, T> {
        type OwnedSelf = Cow<'static, T>;

        fn into_static(self) -> Self::OwnedSelf {
            Cow::Owned(self.into_owned())
        }
    }

    impl<T: Ownit> Ownit for Vec<T> {
        type OwnedSelf = Vec<T::OwnedSelf>;

        fn into_static(self) -> Self::OwnedSelf {
            self.into_iter().map(|t| t.into_static()).collect()
        }
    }

    impl<T: Ownit> Ownit for Box<T> {
        type OwnedSelf = Box<T::OwnedSelf>;

        fn into_static(self) -> Self::OwnedSelf {
            let inner = *self;
            Box::new(inner.into_static())
        }
    }

    impl<T: Ownit> Ownit for Option<T> {
        type OwnedSelf = Option<T::OwnedSelf>;

        fn into_static(self) -> Self::OwnedSelf {
            self.map(Ownit::into_static)
        }
    }

    impl<T: Ownit, E: Ownit> Ownit for Result<T, E> {
        type OwnedSelf = Result<T::OwnedSelf, E::OwnedSelf>;

        fn into_static(self) -> Self::OwnedSelf {
            match self {
                Ok(t) => Ok(t.into_static()),
                Err(e) => Err(e.into_static()),
            }
        }
    }

    impl<T: Ownit + Clone> Ownit for Rc<T> {
        type OwnedSelf = Rc<T::OwnedSelf>;

        fn into_static(self) -> Self::OwnedSelf {
            let inner = Rc::unwrap_or_clone(self);
            Rc::new(inner.into_static())
        }
    }
    impl Ownit for () {
        type OwnedSelf = ();

        fn into_static(self) -> Self::OwnedSelf {
            ()
        }
    }

    impl<const N: usize, T: Ownit> Ownit for [T; N] {
        type OwnedSelf = [T::OwnedSelf; N];

        fn into_static(self) -> Self::OwnedSelf {
            self.map(Ownit::into_static)
        }
    }

    blanket_owned!(String);
    blanket_owned!(PathBuf);
    blanket_owned!(Duration);
    blanket_owned!(Instant);
    blanket_owned!(SystemTime);

    blanket_owned!(bool);
    blanket_owned!(char);

    blanket_owned!(u8);
    blanket_owned!(u16);
    blanket_owned!(u32);
    blanket_owned!(u64);
    blanket_owned!(u128);
    blanket_owned!(usize);

    blanket_owned!(i8);
    blanket_owned!(i16);
    blanket_owned!(i32);
    blanket_owned!(i64);
    blanket_owned!(i128);
    blanket_owned!(isize);

    blanket_owned!(AtomicU8);
    blanket_owned!(AtomicU16);
    blanket_owned!(AtomicU32);
    blanket_owned!(AtomicU64);
    blanket_owned!(AtomicUsize);

    blanket_owned!(AtomicI8);
    blanket_owned!(AtomicI16);
    blanket_owned!(AtomicI32);
    blanket_owned!(AtomicI64);
    blanket_owned!(AtomicIsize);

    blanket_owned!(f32);
    blanket_owned!(f64);
}
