use surf::{Error as SurfError, Response as SurfResponse};

use super::{Asserhttp, AsserhttpStatus};

mod status;

impl Asserhttp<SurfResponse> for SurfResponse {}

impl Asserhttp<SurfResponse> for Result<SurfResponse, SurfError> {}