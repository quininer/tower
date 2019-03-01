use std::marker::PhantomData;
use tower_service::Service;
use Layer;

/// Two middlewares chained together.
///
/// This type is produced by `Layer::chain`.
#[derive(Debug)]
pub struct Chain<Inner, Outer, M> {
    inner: Inner,
    outer: Outer,
    _p: PhantomData<fn(M)>,
}

/// Error's produced when chaining two layers together
pub enum ChainError<I, O> {
    /// Error produced from the inner layer call
    Inner(I),
    /// Error produced from the outer layer call
    Outer(O),
}

impl<Inner, Outer, M> Chain<Inner, Outer, M> {
    /// Create a new `Chain`.
    pub fn new(inner: Inner, outer: Outer) -> Self {
        Chain {
            inner,
            outer,
            _p: PhantomData,
        }
    }
}

impl<S, Req, MidReq, NextReq, Inner, Outer> Layer<S, Req, NextReq> for Chain<Inner, Outer, MidReq>
where
    S: Service<NextReq>,
    Inner: Layer<S, MidReq, NextReq>,
    Outer: Layer<Inner::Service, Req, MidReq>,
{
    type Response = Outer::Response;
    type Error = Outer::Error;
    type LayerError = ChainError<Inner::LayerError, Outer::LayerError>;
    type Service = Outer::Service;

    fn layer(&self, service: S) -> Result<Self::Service, Self::LayerError> {
        let inner = self
            .inner
            .layer(service)
            .map_err(|e| ChainError::Inner(e))?;

        self.outer.layer(inner).map_err(|e| ChainError::Outer(e))
    }
}
