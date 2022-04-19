use super::Asserhttp;

mod status;
mod header;
mod body;

pub type AwcResponse = awc::ClientResponse<actix_http::BoxedPayloadStream>;
pub type ResultAwcResponse = Result<AwcResponse, awc::error::SendRequestError>;

impl Asserhttp<AwcResponse> for AwcResponse {}

impl Asserhttp<AwcResponse> for ResultAwcResponse {}