use super::Asserhttp;

mod status;
mod header;
mod body;

pub type RocketResponse<'a> = rocket::local::blocking::LocalResponse<'a>;
pub type AsyncRocketResponse<'a> = rocket::local::asynchronous::LocalResponse<'a>;

impl<'a> Asserhttp<RocketResponse<'a>> for RocketResponse<'a> {}

impl<'a> Asserhttp<AsyncRocketResponse<'a>> for AsyncRocketResponse<'a> {}