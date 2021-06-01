mod assert_surf;
mod assert_isahc;

pub trait Asserhttp {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self;
    fn assert_ok(&mut self) -> &mut Self { self.assert_status_eq(200) }
}

pub trait TryAsserhttp<T> {
    fn assert_status_eq(&mut self, status: u16) -> &mut T;
    fn assert_ok(&mut self) -> &mut T { self.assert_status_eq(200) }
}