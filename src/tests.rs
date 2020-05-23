#![cfg(test)]
use super::*;

#[test]
fn same_type() {
    let x: i32 = 3;
    let inc = |x| x + 1;
    let double = |x| x + x;
    let square = |x| x * x;
    let a = (x).pipe(inc).pipe(double).pipe(square);
    let b = square(double(inc(x)));
    assert_eq!(a, b);
}

#[test]
fn type_transformation() {
    let x = 'x';
    let a = x.pipe(|x| (x, x, x)).pipe(|x| [x, x]);
    let b = [('x', 'x', 'x'), ('x', 'x', 'x')];
    assert_eq!(a, b);
}

#[test]
fn slice() {
    let vec: &[i32] = &[0, 1, 2, 3];
    let vec = vec.pipe(|x: &[i32]| [x, &[4, 5, 6]].concat());
    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn trait_object() {
    use core::{cmp::PartialEq, fmt::Display, marker::Copy};
    fn run(x: impl AsRef<str> + PartialEq + Display + Copy + ?Sized) {
        let x = x.pipe(|x| x);
        assert_eq!(x.as_ref(), "abc");
    }
    run("abc");
}

#[test]
#[allow(clippy::blacklisted_name)]
fn pipe_ref() {
    #[derive(Debug, PartialEq, Eq)]
    struct FooBar(i32);
    let foo = FooBar(12);
    let bar = foo.pipe_ref(|x| x.0).pipe(FooBar);
    assert_eq!(foo, bar);
}

#[test]
#[allow(clippy::blacklisted_name)]
fn pipe_mut() {
    #[derive(Debug, PartialEq, Eq)]
    struct Foo(i32);
    let mut foo = Foo(0);
    foo.pipe_mut(|x| x.0 = 32);
    assert_eq!(foo, Foo(32));
}
