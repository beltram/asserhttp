use actix_http::{encoding::Decoder, Payload};
use awc::error::SendRequestError as AwcError;

use super::Asserhttp;

mod status;
mod header;
mod body;

pub type AwcResponse = awc::ClientResponse<Decoder<Payload>>;

impl Asserhttp<AwcResponse> for AwcResponse {}

impl Asserhttp<AwcResponse> for Result<AwcResponse, AwcError> {}