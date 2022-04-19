use super::Asserhttp;

mod status;
mod header;
mod body;

pub type ReqwestResponse = reqwest::blocking::Response;
pub type ResultReqwestResponse = Result<ReqwestResponse, reqwest::Error>;
pub type AsyncReqwestResponse = reqwest::Response;
pub type ResultAsyncReqwestResponse = Result<AsyncReqwestResponse, reqwest::Error>;

impl Asserhttp<ReqwestResponse> for ReqwestResponse {}

impl Asserhttp<ReqwestResponse> for ResultReqwestResponse {}

impl Asserhttp<AsyncReqwestResponse> for AsyncReqwestResponse {}

impl Asserhttp<AsyncReqwestResponse> for ResultAsyncReqwestResponse {}
