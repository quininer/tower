//! Types and utilities for working with `Layer`.

use Layer;

mod chain;

pub use self::chain::{Chain, ChainError};

/// An extension trait for `Layer`'s that provides a variety of convenient
/// adapters.
pub trait LayerExt<S, Request, NextRequest>: Layer<S, Request, NextRequest> {
    /// Return a new `Layer` instance that applies both `self` and
    /// `middleware` to services being wrapped.
    ///
    /// This defines a middleware stack.
    fn chain<T, MidReq>(self, middleware: T) -> Chain<Self, T, MidReq>
    where
        T: Layer<S, NextRequest, MidReq>,
        Self: Sized,
    {
        Chain::new(self, middleware)
    }
}

impl<T, S, Request, NextRequest> LayerExt<S, Request, NextRequest> for T where
    T: Layer<S, Request, NextRequest>
{
}
