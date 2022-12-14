/*use asserhttp::*;

pub trait MyHttpDsl<T>: Asserhttp<T> {
    fn is_oki(&mut self) -> &mut T {
        self.expect_status_ok()
    }

    fn is_json(&mut self) -> &mut T {
        self.expect_content_type_json()
    }

    fn has_body(&mut self) -> &mut T { self.expect_body_present() }
}

// impl<T> MyHttpDsl<T> for T where T: asserhttp::AllAccessors {}
// impl<T, E> MyHttpDsl<T> for Result<T, E> where T: asserhttp::AllAccessors, E: std::fmt::Debug {}

macro_rules! asserhttp_customize {
    ($custom_trait:ident) => {
        impl<T> $custom_trait<T> for T where T: asserhttp::AllAccessors {}
        impl<T, E> $custom_trait<T> for Result<T, E> where T: asserhttp::AllAccessors, E: std::fmt::Debug {}
    };
}

asserhttp_customize!(MyHttpDsl);

#[test]
#[stubr::mock("full.json")]
fn sample() {
    reqwest::blocking::get(stubr.uri()).unwrap().is_oki().is_json();
    reqwest::blocking::get(stubr.uri()).is_oki().is_json();
}*/