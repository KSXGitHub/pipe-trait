//! Add `pipe` method to every type.
//!
//! **Example:** Same type
//!
//! ```
//! # use pipe_trait::*;
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
//! # use pipe_trait::*;
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
pub trait Pipe
where
    Self: Sized,
{
    /// Apply `f` to `self`.
    ///
    /// **Example:** Same type
    ///
    /// ```
    /// # use pipe_trait::*;
    /// let inc = |x| x + 1;
    /// let double = |x| x + x;
    /// let square = |x| x * x;
    /// let a = (123i32).pipe(inc).pipe(double).pipe(square);
    /// let b = square(double(inc(123i32)));
    /// assert_eq!(a, b);
    /// ```
    ///
    /// **Example:** Type transformation
    ///
    /// ```
    /// # use pipe_trait::*;
    /// let x = 'x';
    /// let a = x // char
    ///     .pipe(|x| (x, x, x)) // (char, char, char)
    ///     .pipe(|x| [x, x]) // [(char, char, char); 2]
    ///     .pipe(|x| format!("{:?}", x)); // String
    /// let b = "[('x', 'x', 'x'), ('x', 'x', 'x')]";
    /// assert_eq!(a, b);
    /// ```
    fn pipe<Return>(self, f: impl FnOnce(Self) -> Return) -> Return {
        f(self)
    }
}

impl<X: Sized> Pipe for X {}

#[cfg(test)]
mod tests;
