//! Add `pipe` method to every type.
//!
//! **Example:** Pipe amongst method chain
//!
//! ```
//! # async {
//! # use std::fmt::*;
//! # use futures::future::*;
//! # #[derive(Debug, Copy, Clone)]
//! # struct Num(pub i32);
//! # impl Num {
//! #     pub fn inc(&self) -> Self { Self(self.0 + 1) }
//! #     pub fn double(&self) -> Self { Self(self.0 * 2) }
//! #     pub fn square(&self) -> Self { Self(self.0 * self.0) }
//! #     pub fn get(&self) -> i32 { self.0 }
//! #     pub fn future(self) -> Ready<Self> { ready(self) }
//! # }
//! # let my_future = Num(12).future();
//! use pipe_trait::*;
//! fn log<X: Debug>(x: X) -> X {
//!     println!("value: {:?}", x);
//!     x
//! }
//! my_future
//!     .pipe(log)
//!     .await
//!     .pipe(log)
//!     .inc()
//!     .pipe(log)
//!     .double()
//!     .pipe(log)
//!     .square()
//!     .pipe(log)
//!     .get()
//!     .pipe(log);
//! # };
//! ```
//!
//! **Example:** Same type
//!
//! ```
//! use pipe_trait::*;
//! let inc = |x| x + 1;
//! let double = |x| x + x;
//! let square = |x| x * x;
//! let a = (123i32).pipe(inc).pipe(double).pipe(square);
//! let b = square(double(inc(123i32)));
//! assert_eq!(a, b);
//! ```
//!
//! **Example:** Type transformation
//!
//! ```
//! use pipe_trait::*;
//! let x = 'x';
//! let a = x
//!     .pipe(|x| (x, x, x)) // (char, char, char)
//!     .pipe(|x| [x, x]) // [(char, char, char); 2]
//!     .pipe(|x| format!("{:?}", x)); // String
//! let b = "[('x', 'x', 'x'), ('x', 'x', 'x')]";
//! assert_eq!(a, b);
//! ```

#![no_std]
pub use core::marker::Sized;

/// All sized types implement this trait.
pub trait Pipe: Sized {
    /// Apply `f` to `self`.
    fn pipe<Return>(self, f: impl FnOnce(Self) -> Return) -> Return {
        f(self)
    }
}

impl<X: Sized> Pipe for X {}

#[cfg(test)]
mod tests;
