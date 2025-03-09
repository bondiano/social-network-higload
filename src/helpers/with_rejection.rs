use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Extractor for customizing extractor rejections
///
/// `WithRejection` wraps another extractor and gives you the result. If the
/// extraction fails, the `Rejection` is transformed into `R` and returned as a
/// response
///
/// `E` is expected to implement [`FromRequest`]
///
/// `R` is expected to implement [`IntoResponse`] and [`From<E::Rejection>`]
pub struct WithRejection<E, R>(pub E, pub PhantomData<R>);

impl<E, R> WithRejection<E, R> {
  /// Returns the wrapped extractor
  pub fn into_inner(self) -> E {
    self.0
  }
}

impl<E, R> Debug for WithRejection<E, R>
where
  E: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("WithRejection")
      .field(&self.0)
      .field(&self.1)
      .finish()
  }
}

impl<E, R> Clone for WithRejection<E, R>
where
  E: Clone,
{
  fn clone(&self) -> Self {
    Self(self.0.clone(), self.1)
  }
}

impl<E, R> Copy for WithRejection<E, R> where E: Copy {}

impl<E: Default, R> Default for WithRejection<E, R> {
  fn default() -> Self {
    Self(Default::default(), Default::default())
  }
}

impl<E, R> Deref for WithRejection<E, R> {
  type Target = E;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<E, R> DerefMut for WithRejection<E, R> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<E, R, S> FromRequest<S> for WithRejection<E, R>
where
  S: Send + Sync,
  E: FromRequest<S>,
  R: From<E::Rejection> + IntoResponse,
{
  type Rejection = R;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let extractor = E::from_request(req, state).await?;
    Ok(WithRejection(extractor, PhantomData))
  }
}

impl<E, R, S> FromRequestParts<S> for WithRejection<E, R>
where
  S: Send + Sync,
  E: FromRequestParts<S>,
  R: From<E::Rejection> + IntoResponse,
{
  type Rejection = R;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let extractor = E::from_request_parts(parts, state).await?;
    Ok(WithRejection(extractor, PhantomData))
  }
}

impl<E, R> Display for WithRejection<E, R>
where
  E: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use axum::body::Body;
  use axum::http::Request;
  use axum::response::Response;

  #[tokio::test]
  async fn extractor_rejection_is_transformed() {
    struct TestExtractor;
    struct TestRejection;

    impl<S> FromRequestParts<S> for TestExtractor
    where
      S: Send + Sync,
    {
      type Rejection = ();

      async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Err(())
      }
    }

    impl IntoResponse for TestRejection {
      fn into_response(self) -> Response {
        ().into_response()
      }
    }

    impl From<()> for TestRejection {
      fn from(_: ()) -> Self {
        TestRejection
      }
    }

    let req = Request::new(Body::empty());
    let result = WithRejection::<TestExtractor, TestRejection>::from_request(req, &()).await;
    assert!(matches!(result, Err(TestRejection)));

    let (mut parts, _) = Request::new(()).into_parts();
    let result =
      WithRejection::<TestExtractor, TestRejection>::from_request_parts(&mut parts, &()).await;
    assert!(matches!(result, Err(TestRejection)));
  }
}
