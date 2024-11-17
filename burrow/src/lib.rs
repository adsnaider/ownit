use std::borrow::Cow;

#[cfg(feature = "derive")]
pub use burrow_derive::Burrow;

pub trait Burrow {
    type OwnedSelf;

    fn into_static(self) -> Self::OwnedSelf;
}

impl<T: ToOwned + ?Sized + 'static> Burrow for Cow<'_, T> {
    type OwnedSelf = Cow<'static, T>;

    fn into_static(self) -> Self::OwnedSelf {
        Cow::Owned(self.into_owned())
    }
}

blanket_owned!(String);
blanket_owned!(Vec<..>);

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

blanket_owned!(f32);
blanket_owned!(f64);

mod util {
    #[macro_export]
    macro_rules! blanket_owned {
        ($ty:ident) => {
            impl Burrow for $ty {
                type OwnedSelf = $ty;

                fn into_static(self) -> Self::OwnedSelf {
                    self
                }
            }
        };
        ($ty:ident<..>) => {
            impl<T: 'static> Burrow for $ty<T> {
                type OwnedSelf = $ty<T>;

                fn into_static(self) -> Self::OwnedSelf {
                    self
                }
            }
        };
    }
}
