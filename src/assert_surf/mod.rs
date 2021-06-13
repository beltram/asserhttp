use surf::{Error as SurfError, Response as SurfResponse};

use super::Asserhttp;

mod status;
mod header;
mod body;

impl Asserhttp<SurfResponse> for SurfResponse {}

impl Asserhttp<SurfResponse> for Result<SurfResponse, SurfError> {}