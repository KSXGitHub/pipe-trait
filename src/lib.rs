#![doc = include_str!("doc.md")]
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
    /// and `Self` implements trait [`AsRef<Param>`].
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
    /// and `Self` implements trait [`AsMut<Param>`].
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
    /// and `Self` implements trait `Deref<Target = Param>`.
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
    /// and `Self` implements trait [`DerefMut<Target = Param>`].
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
    /// and `Self` implements trait [`Borrow<Param>`].
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
    /// and `Self` implements trait [`BorrowMut<Param>`].
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
