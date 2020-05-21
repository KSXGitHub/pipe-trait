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
    let a = x
        .pipe(|x| (x, x, x))
        .pipe(|x| [x, x])
        .pipe(|x| format!("{:?}", x));
    let b = "[('x', 'x', 'x'), ('x', 'x', 'x')]";
    assert_eq!(a, b);
}

#[test]
fn mutable_function() {
    use std::fmt::Write;
    let mut writable = "".to_owned();
    let mut_fun: Box<dyn FnMut(_) -> _> = Box::new(|x| write!(writable, "{}", x));
    "abc".pipe(|x| format!("{}{}", x, x)).pipe(mut_fun).unwrap();
    assert_eq!(writable, "abcabc");
}

#[test]
fn slice() {
    let vec: &[i32] = &[0, 1, 2, 3];
    let vec = vec.pipe(|x: &[i32]| [x, &[4, 5, 6]].concat());
    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn trait_object() {
    fn run(x: impl std::fmt::Display) {
        let x = x.pipe(|x| format!("{}", x));
        assert_eq!(x, "abc");
    }
    run("abc");
}
