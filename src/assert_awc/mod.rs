use actix_http::{BoxedPayloadStream, encoding::Decoder, Payload};
use awc::{ClientResponse, error::SendRequestError as AwcError};

use super::Asserhttp;

mod status;
mod header;
mod body;

pub type AwcResponse = ClientResponse<Decoder<Payload<BoxedPayloadStream>>>;

impl Asserhttp<AwcResponse> for AwcResponse {}

impl Asserhttp<AwcResponse> for Result<AwcResponse, AwcError> {}