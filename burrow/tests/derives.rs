use std::borrow::Cow;

use burrow::Burrow;

#[derive(Burrow)]
pub struct Foo<'a, 'b, T: Clone> {
    nothinga: Cow<'a, str>,
    nothingb: Cow<'b, T>,
    foo: usize,
    bar: String,
}

#[derive(Burrow)]
pub struct Bar<'a, 'b, T: Clone>(Cow<'a, str>, Cow<'b, T>, usize, String);

#[derive(Burrow)]
pub struct Unit;

#[derive(Burrow)]
pub enum Enumeration {
    A,
    B,
    C,
}

// pub enum Enumeration<'a, 'b, T: Clone> {
//     A(String),
//     B,
//     C(Cow<'a, str>, Cow<'b, T>),
// }

// impl<T: Clone + 'static> Burrow for Enumeration<'_, '_, T> {
//     type OwnedSelf = Enumeration<'static, 'static, T>;

//     fn into_static(self) -> Self::OwnedSelf {
//         match self {
//             Enumeration::A(x0) => Enumeration::A(x0.into_static()),
//             Enumeration::B => Enumeration::B,
//             Enumeration::C(x0, x1) => Enumeration::C(x0.into_static(), x1.into_static()),
//         }
//     }
// }

#[allow(dead_code)]
fn it_works_1(b: Foo<'_, '_, String>) -> Foo<'static, 'static, String> {
    b.into_static()
}
#[allow(dead_code)]
fn it_works_2(b: Bar<'_, '_, String>) -> Bar<'static, 'static, String> {
    b.into_static()
}

#[allow(dead_code)]
fn it_works_3(b: Unit) -> Unit {
    b.into_static()
}
