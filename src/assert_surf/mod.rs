use super::Asserhttp;

mod status;
mod header;
mod body;

pub type SurfResponse = surf::Response;
pub type ResultSurfResponse = Result<SurfResponse, surf::Error>;

impl Asserhttp<SurfResponse> for SurfResponse {}

impl Asserhttp<SurfResponse> for ResultSurfResponse {}