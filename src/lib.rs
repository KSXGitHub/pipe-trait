//! Make it possible to chain regular functions.
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

#![no_std]
pub use core::marker::Sized;

/// All sized types implement this trait.
pub trait Pipe {
    /// Apply `f` to `self`.
    ///
    /// ```
    /// # #[derive(Debug, PartialEq, Eq)]
    /// # struct Foo(i32);
    /// # fn double(x: i32) -> i32 { x * 2 }
    /// # use pipe_trait::*;
    /// assert_eq!(
    ///     12.pipe(double).pipe(Foo),
    ///     Foo(double(12)),
    /// )
    /// ```
    #[inline]
    fn pipe<Return>(self, f: impl FnOnce(Self) -> Return) -> Return
    where
        Self: Sized,
    {
        f(self)
    }

    /// Apply `f` to `&self`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Foo(i32);
    /// let a = Foo(12);
    /// let b = a
    ///     .pipe_ref(|a| a.0) // a is not moved
    ///     .pipe(Foo);
    /// assert_eq!(a, b); // a is used again
    /// ```
    #[inline]
    fn pipe_ref<Return>(&self, f: impl FnOnce(&Self) -> Return) -> Return {
        f(self)
    }

    /// Apply `f` to `&mut self`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Foo(i32, i32);
    /// let mut a = Foo(0, 0);
    /// a.pipe_mut(|a| a.0 = 12);
    /// a.pipe_mut(|a| a.1 = 34);
    /// assert_eq!(a, Foo(12, 34));
    /// ```
    #[inline]
    fn pipe_mut<Return>(&mut self, f: impl FnOnce(&mut Self) -> Return) -> Return {
        f(self)
    }
}

impl<X> Pipe for X {}

#[cfg(test)]
mod tests;
