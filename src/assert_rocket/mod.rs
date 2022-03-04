use rocket::local::{
    asynchronous::LocalResponse as AsyncRocketResponse,
    blocking::LocalResponse as BlockingRocketResponse,
};

use super::Asserhttp;

mod status;
mod header;
mod body;

impl<'a> Asserhttp<BlockingRocketResponse<'a>> for BlockingRocketResponse<'a> {}

impl<'a> Asserhttp<AsyncRocketResponse<'a>> for AsyncRocketResponse<'a> {}