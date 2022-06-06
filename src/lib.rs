//! A crate that allows declaring an equality bound on two types and coercing between them.
//!
//! # Examples
//!
//! ```
//! # use eq_type::Is;
//! #
//! pub trait ResultExt<T, E>: Sized + Is<Result<T, E>> {
//!     /// Given any `E` and `EI` that implement `Into<ER>`, converts from Result<Result<T, EI>, E> to Result<T, ER>.
//!     fn flatten_into<TI, EI, ER>(self) -> Result<TI, ER>
//!     where
//!         T: Is<Result<TI, EI>>,
//!         E: Into<ER>,
//!         EI: Into<ER>,
//!     {
//!         self.coerce().map_err(|e| e.into()).and_then(|x| x.coerce().map_err(|e| e.into()))
//!     }
//! }
//! #
//! # fn main() { }
//! ```

mod private {
    pub trait Sealed {}
}

pub trait Is<Rhs: ?Sized>: private::Sealed {
    fn coerce(self) -> Rhs
    where
        Self: Sized,
        Rhs: Sized;

    fn rcoerce(rhs: Rhs) -> Self
    where
        Self: Sized,
        Rhs: Sized;
}

impl<T: ?Sized> private::Sealed for T {}

impl<T: ?Sized> Is<T> for T {
    #[inline(always)]
    fn coerce(self) -> T
    where
        T: Sized,
    {
        self
    }

    #[inline(always)]
    fn rcoerce(t: T) -> T
    where
        T: Sized,
    {
        t
    }
}

#[cfg(test)]
mod tests {
    use super::Is;

    fn forward<T, U>(t: T) -> U
    where
        T: Is<U>,
    {
        t.coerce()
    }

    fn backward<T, U>(u: U) -> T
    where
        U: Is<T>,
    {
        u.coerce()
    }

    #[test]
    fn test() {
        let mut x = 4;
        x = forward(x);
        x = backward(x);
        x = backward(forward(x));
        x = forward(backward(x));
        assert_eq!(x, 4);
    }
}
