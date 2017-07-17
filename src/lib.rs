extern crate hyper;
extern crate hyper_native_tls;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper_native_tls::NativeTlsClient;
use hyper::client::Client;
use hyper::net::HttpsConnector;

pub mod model;
pub mod graph;
pub mod error;

mod tests;

use graph::*;
use model::*;
use error::*;

/// A wrapper around a hyper::client::Client.  An Arc wrapping FBApi can be
/// cloned to multiple threads, as it implements Send.

#[derive(Debug)]
pub struct FBApi {
  api_version: (u8, u8),
  client: Client,
}

impl FBApi {
  /// Initialise the facebook API.   
  /// # Params
  /// * `version` - The version of the facebook graph API to use, as a tuple.
  /// # Examples
  /// ```
  /// // Create api with graph api version 2.5.
  /// let fb_api = FBApi::new((2, 5)); 
  /// ```
  /// # Notes
  /// Tested with versions: 2.9
  pub fn new(version: (u8, u8)) -> FBApi {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    FBApi {
      api_version: version,
      client: Client::with_connector(connector),
    }
  }

  /// Attempt to request an edge on the graph API, and store the data in an
  /// FBModel.
  pub fn get<T: FBModel>(&self, edge: Edge, auth_token: &str) -> Result<T, GetModelError> {
    use std::io::Read;

    // Get URL of graph API
    let mut url = "https://graph.facebook.com".to_owned();
    url = url + "/v" + &self.api_version.0.to_string() + "." + &self.api_version.1.to_string();
    url += edge.get_path();

    let mut body = edge.build_body();

    body = body + "&access_token=" + auth_token;

    let mut res = try!(self.client.post(&url).body(&body).send());

    let mut buf = String::new();
    res.read_to_string(&mut buf).unwrap();

    // Parse into a value map first
    let v_map = try!(serde_json::from_str(&buf) as Result<serde_json::Value, _>);
    match v_map {
      serde_json::Value::Object(map) => {
        let error = map.get("error");
        if error.is_some() {
          let error = error.unwrap();
          if error.is_object() {
            let error_map = error.as_object().unwrap();
            // Parse type and message
            let e_type = try!(error_map.get("type").ok_or(
                GetModelError::UnknownFBError(
                  "Unknown error structure: ".to_owned(), buf.clone())));
            let e_message = try!(error_map.get("message").ok_or(
                GetModelError::UnknownFBError(
                  format!("Unknown error structure of type {}", e_type), buf.clone())));
            if !e_type.is_string() || !e_message.is_string() {
              return Err(GetModelError::UnknownFBError(
                  "Unknown error structure: ".to_owned(), buf));
            }
            let e_type = e_type.as_str().unwrap();
            let e_message = e_message.as_str().unwrap();
            return if e_type == "OAuthException" {
              Err(GetModelError::AccessTokenInvalid(e_message.to_owned()))
            }
            else {
              Err(GetModelError::UnknownFBError(e_type.to_owned(), e_message.to_owned()))
            }
          }
        }
      },
      _ => ()
    }

    let data = try!(serde_json::from_str(&buf) as Result<T, _>);
    return Ok(data);
  }
}
