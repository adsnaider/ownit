use std::borrow::Cow;

use ownit::Ownit;

#[derive(Ownit)]
pub struct Foo<'a, 'b, T: Clone> {
    nothinga: Cow<'a, str>,
    nothingb: Cow<'b, T>,
    foo: usize,
    baz: f64,
    bar: String,
}

#[derive(Ownit)]
pub struct Bar<'a, 'b, T: Clone>(Cow<'a, str>, Cow<'b, T>, usize, String);

#[derive(Ownit)]
pub struct Unit;

#[derive(Ownit)]
pub enum Enumeration<'a, 'b, T: Clone> {
    A(String),
    B,
    C(Cow<'a, str>, Cow<'b, T>),
    D { foo: Cow<'a, str>, bar: Cow<'b, T> },
}

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

#[allow(dead_code)]
fn it_works_4(b: Enumeration<'_, '_, String>) -> Enumeration<'static, 'static, String> {
    b.into_static()
}
