use hyper::{
    Response as HyperResponse,
    Result as HyperResult,
    Body as HyperBody,
};

use super::Asserhttp;

mod status;
mod header;
mod body;

impl Asserhttp<HyperResponse<HyperBody>> for HyperResponse<HyperBody> {}

impl Asserhttp<HyperResponse<HyperBody>> for HyperResult<HyperResponse<HyperBody>> {}