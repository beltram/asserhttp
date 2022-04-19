use actix_http::body::MessageBody;

use super::accessor::{BodyAccessor, HeaderAccessor, StatusAccessor};

type ActixResponse = actix_web::HttpResponse<actix_http::body::BoxBody>;
type ActixServiceResponse = actix_web::dev::ServiceResponse;

impl StatusAccessor for ActixResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl StatusAccessor for ActixServiceResponse {
    fn get_status(&self) -> u16 {
        self.status().as_u16()
    }
}

impl HeaderAccessor for ActixResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter()
            .map(|(k, _)| k.as_str().to_string())
            .collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self.headers().get(key)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap();
        vec![value]
    }
}

impl HeaderAccessor for ActixServiceResponse {
    fn get_keys(&self) -> Vec<String> {
        self.headers().iter()
            .map(|(k, _)| k.as_str().to_string())
            .collect::<Vec<_>>()
    }

    fn get_raw_values(&self, key: &str) -> Vec<String> {
        let value = self.headers().get(key)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap();
        vec![value]
    }
}

impl BodyAccessor for ActixResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        body_bytes(self)
    }
}

impl BodyAccessor for ActixServiceResponse {
    fn get_bytes(&mut self) -> anyhow::Result<Vec<u8>> {
        body_bytes(self.response_mut())
    }
}

fn body_bytes(original: &mut ActixResponse) -> anyhow::Result<Vec<u8>> {
    let mut resp_cpy = ActixResponse::build(original.status());
    original.headers().iter().for_each(|h| { resp_cpy.insert_header(h); });
    let mut resp_cpy = resp_cpy.finish();
    std::mem::swap(original, &mut resp_cpy);
    let (_, body) = resp_cpy.into_parts();
    body.try_into_bytes()
        .map(|b| b.to_vec())
        .map_err(|_| String::from("Error"))
        .map_err(anyhow::Error::msg)
}