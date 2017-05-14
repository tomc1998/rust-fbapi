//! Module containing data models for the facebook API.

mod user;

use serde::Deserialize;

pub trait FBModel : Deserialize {}

pub use self::user::User;
pub use self::user::UserPicture;

