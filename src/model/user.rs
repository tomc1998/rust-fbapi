//! Module for the user data.

use model;
use serde;
use serde_json;
use serde::Deserialize;
use serde::de::{Visitor, MapVisitor};
use std::fmt;

/// A struct representing a user's picture.
#[derive(Debug, Clone, Deserialize)]
pub struct UserPicture {
  pub is_silhouette: bool,

  pub url: String,
  /// Width of picture - only filled if requesting a picture via the picture
  /// edge, not when requesting as a field of a user.
  pub width: Option<u32>,
  /// Height of picture - only filled if requesting a picture via the picture
  /// edge, not when requesting as a field of a user.
  pub height: Option<u32>,
}
impl model::FBModel for UserPicture {}

/// Structure representing a user. Anything noted on the facebook API
/// documentation, but not contained in this struct, is not yet
/// implemented.
#[derive(Debug, Clone)]
pub struct User {
  /// The user's app-specific ID. Default.
  pub id: String,

  /// The user's gender. Either 'male', 'female', a custom user-defined
  /// value based on preferred pronouns, or None if the preferred pronoun
  /// is neutral (I think this means 'they').
  pub gender: Option<String>,

  pub first_name: Option<String>,

  pub last_name: Option<String>,

  /// The user's full name.
  pub name: Option<String>,

  /// The user's picture. This will be filled if requesting a user with the
  /// 'picture' field, however one can also request the picture using the
  /// picture edge.
  pub picture: Option<UserPicture>,
}

impl User {
  fn new_empty() -> User {
    User {
      id: "".to_owned(),
      gender: None,
      first_name: None,
      last_name: None,
      name: None,
      picture: None
    }
  }
}

impl model::FBModel for User {}

impl<'a> Deserialize for User {
  fn deserialize<D: serde::Deserializer>(deserializer: D) -> Result<Self,
    <D as serde::Deserializer>::Error> {
      struct MyVisitor;
      impl Visitor for MyVisitor {
        type Value = User;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          write!(formatter, "A valid JSON object")
        }

        /// Impl of the Visitor trait to visit all the key / value pairs in the
        /// JSON object for User.
        fn visit_map<V: MapVisitor>(self, mut value: V) -> Result<Self::Value, V::Error> {
          use serde_json::value::*;
          let mut res = User::new_empty();
          loop {
            let kv_pair = value.visit::<String, Value>().unwrap();
            if kv_pair.is_none() { break } // No more key value pairs to be visited
            let (k, v) = kv_pair.unwrap();
            // Deserialise into the User object based on the value of k, the key
            match k.as_ref() {
              "picture" => res.picture = serde_json::from_value(
                v.as_object().unwrap().get("data").unwrap().clone()).unwrap(),
              "id" => res.id = serde_json::from_value(v).unwrap(),
              "gender" => res.gender = serde_json::from_value(v).unwrap(),
              "first_name" => res.first_name = serde_json::from_value(v).unwrap(),
              "last_name" => res.last_name = serde_json::from_value(v).unwrap(),
              "name" => res.name = serde_json::from_value(v).unwrap(),
              _ => ()
            }
          }

          return Ok(res);
        }
      }

      let data = deserializer.deserialize_map(MyVisitor).unwrap();
      return Ok(data);
    }
}

