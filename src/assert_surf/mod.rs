use surf::{Error as SurfError, Response as SurfResponse};

use super::{
    Asserhttp,
    AsserhttpStatus,
    TryAsserhttp,
    TryAsserhttpStatus,
};

mod status;

impl Asserhttp for SurfResponse {}

impl TryAsserhttp<SurfResponse> for Result<SurfResponse, SurfError> {}
