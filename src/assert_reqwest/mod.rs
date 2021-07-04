use reqwest::{blocking::Response as ReqwestResponse, Error as ReqwestError, Response as AsyncReqwestResponse};

use super::Asserhttp;

mod status;
mod header;
mod body;

impl Asserhttp<AsyncReqwestResponse> for AsyncReqwestResponse {}

impl Asserhttp<ReqwestResponse> for ReqwestResponse {}

impl Asserhttp<AsyncReqwestResponse> for Result<AsyncReqwestResponse, ReqwestError> {}

impl Asserhttp<ReqwestResponse> for Result<ReqwestResponse, ReqwestError> {}
