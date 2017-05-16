//! Tests module.

#![cfg(test)]

use FBApi;
use graph::*;
use model::*;
use error::*;

/// Tests currently rely on a hardcoded access token.
static ACCESS_TOKEN : &'static str = "";

#[test]
fn test_access_token() {
  assert_ne!(ACCESS_TOKEN, "", r#"To test the FBAPI, an access token must be
  hard coded into the test/mod.rs file."#)
}

/// Test that the fbapi fails gracefully when a request is made to an edge for
/// data to be put in a struct, but the struct is of the wrong type.
#[test]
fn test_err_type_mismatch() {
  let fb_api = FBApi::new((2, 9));
  let mut edge = Edge::user_me();
  edge.add_fields(vec!["gender", "picture", "birthday"]);
  let user : Result<UserPicture, _> = fb_api.get(edge, ACCESS_TOKEN);

  assert!(user.is_err());
  match user.err().unwrap() {
    GetModelError::DeserializeFailed => (),
    _ => assert!(false), // Wrong error type.
  }
}

/// Test that the fbapi fails gracefully if a bad access token is given.
#[test]
fn test_bad_access_token() {
  let fb_api = FBApi::new((2, 9));
  let mut edge = Edge::user_me();
  edge.add_fields(vec!["gender", "picture", "birthday"]);
  let user : Result<UserPicture, _> = fb_api.get(edge, "asd");

  assert!(user.is_err());
  match user.err().unwrap() {
    GetModelError::AccessTokenInvalid(s) => {
      assert!(!s.is_empty(), "Facebook error message shouldn't be empty.");
    }
    _ => assert!(false), // Wrong error type.
  }
}

/// Test the 'me' edge.
#[test]
fn test_me_edge() {
  let fb_api = FBApi::new((2, 9));
  let mut edge = Edge::user_me();
  edge.add_fields(vec!["gender", "picture", "name"]);
  let user : Result<User, _> = fb_api.get(edge, ACCESS_TOKEN);

  assert!(user.is_ok());
  let user = user.unwrap();

  assert!(!user.id.is_empty());
  assert!(user.name.is_some());
  assert!(user.gender.is_some());
  assert!(user.picture.is_some());
  assert!(!user.picture.as_ref().unwrap().url.is_empty())
}

