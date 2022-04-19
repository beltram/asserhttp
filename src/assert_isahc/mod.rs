use super::Asserhttp;

mod status;
mod header;
mod body;

pub type IsahcResponse = isahc::Response<isahc::Body>;
pub type ResultIsahcResponse = Result<IsahcResponse, isahc::Error>;
pub type AsyncIsahcResponse = isahc::Response<isahc::AsyncBody>;
pub type ResultAsyncIsahcResponse = Result<AsyncIsahcResponse, isahc::Error>;

impl Asserhttp<IsahcResponse> for IsahcResponse {}

impl Asserhttp<IsahcResponse> for ResultIsahcResponse {}

impl Asserhttp<AsyncIsahcResponse> for AsyncIsahcResponse {}

impl Asserhttp<AsyncIsahcResponse> for ResultAsyncIsahcResponse {}