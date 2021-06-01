mod assert_surf;
mod assert_isahc;

pub trait Asserhttp: AsserhttpStatus {}

pub trait AsserhttpStatus {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self;
    fn assert_status_ok(&mut self) -> &mut Self { self.assert_status_eq(200) }
    fn assert_status_created(&mut self) -> &mut Self { self.assert_status_eq(201) }
    fn assert_status_accepted(&mut self) -> &mut Self { self.assert_status_eq(202) }
    fn assert_status_no_content(&mut self) -> &mut Self { self.assert_status_eq(204) }
    fn assert_status_bad_request(&mut self) -> &mut Self { self.assert_status_eq(400) }
    fn assert_status_unauthorized(&mut self) -> &mut Self { self.assert_status_eq(401) }
    fn assert_status_forbidden(&mut self) -> &mut Self { self.assert_status_eq(403) }
    fn assert_status_not_found(&mut self) -> &mut Self { self.assert_status_eq(404) }
    fn assert_status_conflict(&mut self) -> &mut Self { self.assert_status_eq(409) }
    fn assert_status_gone(&mut self) -> &mut Self { self.assert_status_eq(410) }
    fn assert_status_server_error(&mut self) -> &mut Self { self.assert_status_eq(500) }
}

pub trait TryAsserhttp<T>: TryAsserhttpStatus<T> {}

pub trait TryAsserhttpStatus<T> {
    fn assert_status_eq(&mut self, status: u16) -> &mut T;
    fn assert_status_ok(&mut self) -> &mut T { self.assert_status_eq(200) }
    fn assert_status_created(&mut self) -> &mut T { self.assert_status_eq(201) }
    fn assert_status_accepted(&mut self) -> &mut T { self.assert_status_eq(202) }
    fn assert_status_no_content(&mut self) -> &mut T { self.assert_status_eq(204) }
    fn assert_status_bad_request(&mut self) -> &mut T { self.assert_status_eq(400) }
    fn assert_status_unauthorized(&mut self) -> &mut T { self.assert_status_eq(401) }
    fn assert_status_forbidden(&mut self) -> &mut T { self.assert_status_eq(403) }
    fn assert_status_not_found(&mut self) -> &mut T { self.assert_status_eq(404) }
    fn assert_status_conflict(&mut self) -> &mut T { self.assert_status_eq(409) }
    fn assert_status_gone(&mut self) -> &mut T { self.assert_status_eq(410) }
    fn assert_status_server_error(&mut self) -> &mut T { self.assert_status_eq(500) }
}