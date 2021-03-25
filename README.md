# Pipe Trait

[![Test](https://github.com/KSXGitHub/pipe-trait/workflows/Test/badge.svg)](https://github.com/KSXGitHub/pipe-trait/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/pipe-trait?logo=rust)](https://crates.io/crates/pipe-trait)

Make it possible to chain regular functions.

## APIs

By adding `use pipe_trait::*`, 9 methods are added to all types:

|        identifier       |       pipe syntax      |  traditional syntax |
|:-----------------------:|:----------------------:|:-------------------:|
| `Pipe::pipe`            | `x.pipe(f)`            | `f(x)`              |
| `Pipe::pipe_ref`        | `x.pipe_ref(f)`        | `f(&x)`             |
| `Pipe::pipe_mut`        | `x.pipe_mut(f)`        | `f(&mut x)`         |
| `Pipe::pipe_as_ref`     | `x.pipe_as_ref(f)`     | `f(x.as_ref())`     |
| `Pipe::pipe_as_mut`     | `x.pipe_as_mut(f)`     | `f(x.as_mut())`     |
| `Pipe::pipe_deref`      | `x.pipe_deref(f)`      | `f(&x)`             |
| `Pipe::pipe_deref_mut`  | `x.pipe_deref_mut(f)`  | `f(&mut x)`         |
| `Pipe::pipe_borrow`     | `x.pipe_borrow(f)`     | `f(x.borrow())`     |
| `Pipe::pipe_borrow_mut` | `x.pipe_borrow_mut(f)` | `f(x.borrow_mut())` |

Read [the docs](https://docs.rs/pipe-trait) for more information.

## Usage Examples

### Same type

```rust
use pipe_trait::*;
let inc = |x| x + 1;
let double = |x| x + x;
let square = |x| x * x;
let a = (123i32).pipe(inc).pipe(double).pipe(square);
let b = square(double(inc(123i32)));
assert_eq!(a, b);
```

### Type transformation

```rust
use pipe_trait::*;
let x = 'x';
let a = x
    .pipe(|x| (x, x, x)) // (char, char, char)
    .pipe(|x| [x, x]) // [(char, char, char); 2]
    .pipe(|x| format!("{:?}", x)); // String
let b = "[('x', 'x', 'x'), ('x', 'x', 'x')]";
assert_eq!(a, b);
```

### Pipe amongst method chain

```rust
use pipe_trait::*;
fn log<X: Debug>(x: X) -> X {
    println!("value: {:?}", x);
    x
}
my_future
    .pipe(log)
    .await
    .pipe(log)
    .inc()
    .pipe(log)
    .double()
    .pipe(log)
    .square()
    .pipe(log)
    .get()
    .pipe(log);
```

### Explicit type annotation

```rust
use pipe_trait::*;
let x = "abc".to_string();
let a = x
    .pipe_ref::<&str, _>(AsRef::as_ref)
    .chars()
    .pipe::<Box<_>, _>(Box::new)
    .collect::<Vec<_>>();
let b = vec!['a', 'b', 'c'];
assert_eq!(a, b);
```

## License

[MIT](https://git.io/JfgHW) © [Hoàng Văn Khải](https://ksxgithub.github.io/)
