//! Make it possible to chain regular functions.
//!
//! **API Overview:**
//!
//! By adding `use pipe_trait::*`, 3 methods are added to all types:
//!
//! |        identifier       |       pipe syntax      |  traditional syntax |
//! |:-----------------------:|:----------------------:|:-------------------:|
//! | `Pipe::pipe`            | `x.pipe(f)`            | `f(x)`              |
//! | `Pipe::pipe_ref`        | `x.pipe_ref(f)`        | `f(&x)`             |
//! | `Pipe::pipe_mut`        | `x.pipe_mut(f)`        | `f(&mut x)`         |
//! | `Pipe::pipe_as_ref`     | `x.pipe_as_ref(f)`     | `f(x.as_ref())`     |
//! | `Pipe::pipe_as_mut`     | `x.pipe_as_mut(f)`     | `f(x.as_mut())`     |
//! | `Pipe::pipe_deref`      | `x.pipe_deref(f)`      | `f(&x)`             |
//! | `Pipe::pipe_deref_mut`  | `x.pipe_deref_mut(f)`  | `f(&mut x)`         |
//! | `Pipe::pipe_borrow`     | `x.pipe_borrow(f)`     | `f(x.borrow())`     |
//! | `Pipe::pipe_borrow_mut` | `x.pipe_borrow_mut(f)` | `f(x.borrow_mut())` |
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
//! **Example:** Explicit type annotation
//!
//! ```
//! use pipe_trait::*;
//! let x = "abc".to_string();
//! let a = x
//!     .pipe_ref::<&str, _>(AsRef::as_ref)
//!     .chars()
//!     .pipe::<Box<_>, _>(Box::new)
//!     .collect::<Vec<_>>();
//! let b = vec!['a', 'b', 'c'];
//! assert_eq!(a, b);
//! ```
//!

#![no_std]
use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

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
    fn pipe<Return, Function>(self, f: Function) -> Return
    where
        Self: Sized,
        Function: FnOnce(Self) -> Return,
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
    fn pipe_ref<'a, Return, Function>(&'a self, f: Function) -> Return
    where
        Function: FnOnce(&'a Self) -> Return,
    {
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
    fn pipe_mut<'a, Return, Function>(&'a mut self, f: Function) -> Return
    where
        Function: FnOnce(&'a mut Self) -> Return,
    {
        f(self)
    }

    /// Apply `f` to `&self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `AsRef<Param>`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// fn uppercase(x: &str) -> String {
    ///   x.to_uppercase()
    /// }
    /// let x: String = "abc".to_string();
    /// let y: String = x.pipe_as_ref(uppercase);
    /// assert_eq!(y, "ABC");
    /// ```
    #[inline]
    fn pipe_as_ref<'a, Param, Return, Function>(&'a self, f: Function) -> Return
    where
        Self: AsRef<Param>,
        Param: ?Sized + 'a,
        Function: FnOnce(&'a Param) -> Return,
    {
        f(self.as_ref())
    }

    /// Apply `f` to `&mut self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `AsMut<Param>`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// fn modify(target: &mut [i32]) {
    ///   target[0] = 123;
    /// }
    /// let mut vec: Vec<i32> = vec![0, 1, 2, 3];
    /// vec.pipe_as_mut(modify);
    /// assert_eq!(vec, vec![123, 1, 2, 3]);
    /// ```
    #[inline]
    fn pipe_as_mut<'a, Param, Return, Function>(&'a mut self, f: Function) -> Return
    where
        Self: AsMut<Param>,
        Param: ?Sized + 'a,
        Function: FnOnce(&'a mut Param) -> Return,
    {
        f(self.as_mut())
    }

    /// Apply `f` to `&self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `Deref<Param>`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// fn uppercase(x: &str) -> String {
    ///   x.to_uppercase()
    /// }
    /// let x: String = "abc".to_string();
    /// let y: String = x.pipe_deref(uppercase);
    /// assert_eq!(y, "ABC");
    /// ```
    #[inline]
    fn pipe_deref<'a, Param, Return, Function>(&'a self, f: Function) -> Return
    where
        Self: Deref<Target = Param>,
        Param: ?Sized + 'a,
        Function: FnOnce(&'a Param) -> Return,
    {
        f(self)
    }

    /// Apply `f` to `&mut self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `DerefMut<Param>`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// fn modify(target: &mut [i32]) {
    ///   target[0] = 123;
    /// }
    /// let mut vec: Vec<i32> = vec![0, 1, 2, 3];
    /// vec.pipe_deref_mut(modify);
    /// assert_eq!(vec, vec![123, 1, 2, 3]);
    /// ```
    #[inline]
    fn pipe_deref_mut<'a, Param, Return, Function>(&'a mut self, f: Function) -> Return
    where
        Self: DerefMut<Target = Param>,
        Param: ?Sized + 'a,
        Function: FnOnce(&'a mut Param) -> Return,
    {
        f(self)
    }

    /// Apply `f` to `&self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `Deref<Param>`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// fn uppercase(x: &str) -> String {
    ///   x.to_uppercase()
    /// }
    /// let x: String = "abc".to_string();
    /// let y: String = x.pipe_borrow(uppercase);
    /// assert_eq!(y, "ABC");
    /// ```
    #[inline]
    fn pipe_borrow<'a, Param, Return, Function>(&'a self, f: Function) -> Return
    where
        Self: Borrow<Param>,
        Param: ?Sized + 'a,
        Function: FnOnce(&'a Param) -> Return,
    {
        f(self.borrow())
    }

    /// Apply `f` to `&mut self` where `f` takes a single parameter of type `Param`
    /// and `Self` implements trait `DerefMut<Param>`.
    ///
    /// ```
    /// # use pipe_trait::*;
    /// fn modify(target: &mut [i32]) {
    ///   target[0] = 123;
    /// }
    /// let mut vec: Vec<i32> = vec![0, 1, 2, 3];
    /// vec.pipe_borrow_mut(modify);
    /// assert_eq!(vec, vec![123, 1, 2, 3]);
    /// ```
    #[inline]
    fn pipe_borrow_mut<'a, Param, Return, Function>(&'a mut self, f: Function) -> Return
    where
        Self: BorrowMut<Param>,
        Param: ?Sized + 'a,
        Function: FnOnce(&'a mut Param) -> Return,
    {
        f(self.borrow_mut())
    }
}

impl<X> Pipe for X {}

#[cfg(test)]
mod tests;
