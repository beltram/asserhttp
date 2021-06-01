mod assert_surf;
mod assert_isahc;

pub trait Asserhttp: AsserhttpStatus {}

pub trait AsserhttpStatus {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self;
    fn assert_status_ok(&mut self) -> &mut Self { self.assert_status_eq(200) }
    fn assert_status_created(&mut self) -> &mut Self { self.assert_status_eq(201) }
    fn assert_status_accepted(&mut self) -> &mut Self { self.assert_status_eq(202) }
    fn assert_status_no_content(&mut self) -> &mut Self { self.assert_status_eq(204) }
}

pub trait TryAsserhttp<T>: TryAsserhttpStatus<T> {}

pub trait TryAsserhttpStatus<T> {
    fn assert_status_eq(&mut self, status: u16) -> &mut T;
    fn assert_status_ok(&mut self) -> &mut T { self.assert_status_eq(200) }
    fn assert_status_created(&mut self) -> &mut T { self.assert_status_eq(201) }
    fn assert_status_accepted(&mut self) -> &mut T { self.assert_status_eq(202) }
    fn assert_status_no_content(&mut self) -> &mut T { self.assert_status_eq(204) }
}