struct Foo<'a>(<& /*'a*/ [fn()] as core::ops::Deref>::Target);

const a: *mut Foo = 0 as _;