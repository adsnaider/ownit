use std::borrow::Cow;

use burrow::Burrow;

#[derive(Burrow)]
pub struct Foo<'a, 'b> {
    nothinga: Cow<'a, str>,
    nothingb: Cow<'b, str>,
    foo: usize,
    bar: String,
}

#[derive(Burrow)]
pub struct Bar<'a, 'b>(Cow<'a, str>, Cow<'b, str>, usize, String);

#[allow(dead_code)]
fn it_works_1(b: Foo<'_, '_>) -> Foo<'static, 'static> {
    b.into_static()
}
#[allow(dead_code)]
fn it_works_2(b: Bar<'_, '_>) -> Bar<'static, 'static> {
    b.into_static()
}
