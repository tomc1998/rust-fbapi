use hyper;
use std::fmt;
use std::error;
use serde_json;

/// An error used when the fbapi fails to get an object from an edge. This
/// normally happens if the type of edge doesn't match the value we're
/// trying to serialize into.
#[derive(Debug)]
pub enum GetModelError {
  RequestFailed(hyper::error::Error),
  DeserializeFailed,
}

impl From<hyper::error::Error> for GetModelError {
  fn from(e: hyper::error::Error) -> Self {
    GetModelError::RequestFailed(e)
  }
}
impl From<serde_json::Error> for GetModelError {
  fn from(_: serde_json::Error) -> Self {
    GetModelError::DeserializeFailed
  }
}

impl fmt::Display for GetModelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      GetModelError::RequestFailed(ref e) => write!(f, "Request error: {}", e),
      GetModelError::DeserializeFailed => 
        write!(f, r#"Deserialization failed. This is normally due to a
        developer requesting an edge to be deserialized into an object which
        doesn't match - for example, using /me for a UserPicture object."#), 
    }
  }
}

impl error::Error for GetModelError {
  fn description(&self) -> &str {
    match *self {
      GetModelError::RequestFailed(ref e) => e.description(),
      GetModelError::DeserializeFailed => r#"Deserialization failed. This is normally due to a
        developer requesting an edge to be deserialized into an object which
        doesn't match - for example, using /me for a UserPicture object."#,
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      GetModelError::RequestFailed(ref e) => Some(e),
      GetModelError::DeserializeFailed => None,
    }
  }
}
