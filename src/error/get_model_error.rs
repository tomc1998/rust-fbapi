use hyper;
use std::fmt;
use std::error::Error;
use serde_json;

/// An error used when the fbapi fails to get an object from an edge. This
/// normally happens if the type of edge doesn't match the value we're
/// trying to serialize into.
#[derive(Debug)]
pub enum GetModelError {
  /// Error occurs when a HTTP request fails
  RequestFailed(hyper::error::Error),
  /// Error occurs when the api cannot deserialize the response from facebook
  /// into the given data type.
  DeserializeFailed,
  /// Error occurs when the access token provided for a query is not valid
  /// (OAuthException). Contains a string, which is a message returned by the
  /// graph API.
  AccessTokenInvalid(String),
  /// An error which is not explicitly handled, from the facebook API. Contains
  /// a string tuple, the first of which is a 'type', and the second of which
  /// is the a message along with the error.
  UnknownFBError(String, String),
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
        write!(f, "{}", self.description()), 
      GetModelError::AccessTokenInvalid(ref m) => write!(f, "Access token invalid: {}", m),
      GetModelError::UnknownFBError(ref t, ref m) => 
        write!(f, "Unknown FB API error of type {} : {}", t, m),
    }
  }
}

impl Error for GetModelError {
  fn description(&self) -> &str {
    match *self {
      GetModelError::RequestFailed(ref e) => e.description(),
      GetModelError::DeserializeFailed => r#"Deserialization failed. This is
      normally due to a developer requesting an edge to be deserialized into an
      object which doesn't match - for example, using /me for a UserPicture
      object."#,
      GetModelError::AccessTokenInvalid(_) => "Access token invalid",
      GetModelError::UnknownFBError(_, _) => "Unknown FB API Error",
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      GetModelError::RequestFailed(ref e) => Some(e),
      GetModelError::DeserializeFailed => None,
      GetModelError::AccessTokenInvalid(_) => None,
      GetModelError::UnknownFBError(_,_) => None,
    }
  }
}
