use actix_http::{body::BoxBody, Error as ActixError};
use actix_web::{dev::ServiceResponse as ActixServiceResponse, HttpResponse as ActixResponse};

use super::Asserhttp;

mod status;
mod header;
mod body;

impl Asserhttp<ActixResponse<BoxBody>> for ActixResponse<BoxBody> {}

impl Asserhttp<ActixResponse<BoxBody>> for Result<ActixResponse<BoxBody>, ActixError> {}

impl Asserhttp<ActixServiceResponse<BoxBody>> for ActixServiceResponse<BoxBody> {}