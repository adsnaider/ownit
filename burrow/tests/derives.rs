use std::borrow::Cow;

use burrow::Burrow;

#[derive(Burrow)]
pub struct Foo<'a> {
    nothing: Cow<'a, str>,
    foo: usize,
    bar: String,
}

#[derive(Burrow)]
pub struct Bar<'a, 'b> {
    nothinga: Cow<'a, str>,
    nothingb: Cow<'b, str>,
    foo: usize,
    bar: String,
}

#[allow(dead_code)]
fn it_works_foo(f: Foo<'_>) -> Foo<'static> {
    f.to_static()
}

#[allow(dead_code)]
fn it_works_bar(b: Bar<'_, '_>) -> Bar<'static, 'static> {
    b.to_static()
}
