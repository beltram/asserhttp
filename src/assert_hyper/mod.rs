use super::Asserhttp;

mod status;
mod header;
mod body;

pub type HyperResponse = hyper::Response<hyper::Body>;
pub type ResultHyperResponse = hyper::Result<HyperResponse>;

impl Asserhttp<HyperResponse> for HyperResponse {}

impl Asserhttp<HyperResponse> for ResultHyperResponse {}