Make it possible to chain regular functions.

**API Overview:**

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

**Example:** Same type

```rust
use pipe_trait::*;
let inc = |x| x + 1;
let double = |x| x + x;
let square = |x| x * x;
let a = (123i32).pipe(inc).pipe(double).pipe(square);
let b = square(double(inc(123i32)));
assert_eq!(a, b);
```

**Example:** Type transformation

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

**Example:** Pipe amongst method chain

```rust
# async {
# use std::fmt::*;
# use futures::future::*;
# #[derive(Debug, Copy, Clone)]
# struct Num(pub i32);
# impl Num {
#     pub fn inc(&self) -> Self { Self(self.0 + 1) }
#     pub fn double(&self) -> Self { Self(self.0 * 2) }
#     pub fn square(&self) -> Self { Self(self.0 * self.0) }
#     pub fn get(&self) -> i32 { self.0 }
#     pub fn future(self) -> Ready<Self> { ready(self) }
# }
# let my_future = Num(12).future();
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
# };
```

**Example:** Explicit type annotation

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
