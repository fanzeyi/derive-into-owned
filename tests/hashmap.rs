#![allow(dead_code)]

#[macro_use]
extern crate derive_into_owned;

use std::{borrow::Cow, collections::HashMap};

#[derive(IntoOwned, Borrowed)]
struct Foo<'a> {
    bees: HashMap<Cow<'a, str>, Bar<'a>>,
}

#[derive(IntoOwned, Borrowed)]
struct Bar<'a> {
    s: Cow<'a, str>,
}

#[test]
fn vec() {
    let key = "key".to_string();
    let local = "asdf".to_string();
    let foo = Foo {
        bees: vec![((&key).into(), Bar { s: (&local).into() })]
            .into_iter()
            .collect(),
    };
    accept_static(foo.into_owned());
}

fn accept_static(foo: Foo<'static>) {
    drop(foo);
}
