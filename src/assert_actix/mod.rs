use actix_http::{body::Body as ActixBody, Response as ActixResponse, Error as ActixError};
use actix_web::dev::ServiceResponse as ActixServiceResponse;

use super::Asserhttp;

mod status;
mod header;
mod body;

impl Asserhttp<ActixResponse<ActixBody>> for ActixResponse<ActixBody> {}

impl Asserhttp<ActixResponse<ActixBody>> for Result<ActixResponse<ActixBody>, ActixError> {}

impl Asserhttp<ActixServiceResponse<ActixBody>> for ActixServiceResponse<ActixBody> {}