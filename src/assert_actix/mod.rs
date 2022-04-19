use super::Asserhttp;

mod status;
mod header;
mod body;

pub type ActixResponse = actix_web::HttpResponse<actix_http::body::BoxBody>;
pub type ResultActixResponse = Result<ActixResponse, actix_http::Error>;
pub type ActixServiceResponse = actix_web::dev::ServiceResponse;

impl Asserhttp<ActixResponse> for ActixResponse {}

impl Asserhttp<ActixResponse> for ResultActixResponse {}

impl Asserhttp<ActixServiceResponse> for ActixServiceResponse {}