use rocket::{get, routes, http::Status, local::asynchronous::Client};

use asserhttp::*;

mod eq {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_eq() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_eq(200);
    }

    #[rocket::async_test]
    async fn should_expect_status_eq_enum() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_eq(Status::Ok);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_eq_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_ok() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[rocket::async_test]
    async fn expect_status_ok_should_panic() {
        #[get("/")]
        fn created() -> Status { Status::Created }
        let client = Client::tracked(rocket::build().mount("/", routes![created])).await.unwrap();
        client.get("/").dispatch().await.expect_status_ok();
    }
}

mod created {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_created() {
        #[get("/")]
        fn created() -> Status { Status::Created }
        let client = Client::tracked(rocket::build().mount("/", routes![created])).await.unwrap();
        client.get("/").dispatch().await.expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_created_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_accepted() {
        #[get("/")]
        fn accepted() -> Status { Status::Accepted }
        let client = Client::tracked(rocket::build().mount("/", routes![accepted])).await.unwrap();
        client.get("/").dispatch().await.expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_accepted_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_no_content() {
        #[get("/")]
        fn no_content() -> Status { Status::NoContent }
        let client = Client::tracked(rocket::build().mount("/", routes![no_content])).await.unwrap();
        client.get("/").dispatch().await.expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_no_content_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_no_content();
    }
}

mod bad_request {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_bad_request() {
        #[get("/")]
        fn bad_request() -> Status { Status::BadRequest }
        let client = Client::tracked(rocket::build().mount("/", routes![bad_request])).await.unwrap();
        client.get("/").dispatch().await.expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_bad_request_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_unauthorized() {
        #[get("/")]
        fn unauthorized() -> Status { Status::Unauthorized }
        let client = Client::tracked(rocket::build().mount("/", routes![unauthorized])).await.unwrap();
        client.get("/").dispatch().await.expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_unauthorized_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_forbidden() {
        #[get("/")]
        fn forbidden() -> Status { Status::Forbidden }
        let client = Client::tracked(rocket::build().mount("/", routes![forbidden])).await.unwrap();
        client.get("/").dispatch().await.expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_forbidden_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_not_found() {
        #[get("/")]
        fn not_found() -> Status { Status::NotFound }
        let client = Client::tracked(rocket::build().mount("/", routes![not_found])).await.unwrap();
        client.get("/").dispatch().await.expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_not_found_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_conflict() {
        #[get("/")]
        fn conflict() -> Status { Status::Conflict }
        let client = Client::tracked(rocket::build().mount("/", routes![conflict])).await.unwrap();
        client.get("/").dispatch().await.expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_conflict_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_gone() {
        #[get("/")]
        fn gone() -> Status { Status::Gone }
        let client = Client::tracked(rocket::build().mount("/", routes![gone])).await.unwrap();
        client.get("/").dispatch().await.expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_gone_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_internal_server_error() {
        #[get("/")]
        fn internal_server_error() -> Status { Status::InternalServerError }
        let client = Client::tracked(rocket::build().mount("/", routes![internal_server_error])).await.unwrap();
        client.get("/").dispatch().await.expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[rocket::async_test]
    async fn expect_status_internal_server_error_should_panic() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_in_inclusive_lower_range() {
        #[get("/")]
        fn created() -> Status { Status::Created }
        let client = Client::tracked(rocket::build().mount("/", routes![created])).await.unwrap();
        client.get("/").dispatch().await.expect_status_in_range(201, 300);
    }

    #[rocket::async_test]
    async fn should_expect_status_in_exclusive_upper_range() {
        #[get("/")]
        fn created() -> Status { Status::Created }
        let client = Client::tracked(rocket::build().mount("/", routes![created])).await.unwrap();
        client.get("/").dispatch().await.expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[rocket::async_test]
    async fn expect_status_in_range_should_panic_when_lower() {
        #[get("/")]
        fn created() -> Status { Status::Created }
        let client = Client::tracked(rocket::build().mount("/", routes![created])).await.unwrap();
        client.get("/").dispatch().await.expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[rocket::async_test]
    async fn expect_status_in_range_should_panic_when_upper() {
        #[get("/")]
        fn created() -> Status { Status::Created }
        let client = Client::tracked(rocket::build().mount("/", routes![created])).await.unwrap();
        client.get("/").dispatch().await.expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_success() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[rocket::async_test]
    async fn expect_status_success_should_panic_when_not() {
        #[get("/")]
        fn bad_request() -> Status { Status::BadRequest }
        let client = Client::tracked(rocket::build().mount("/", routes![bad_request])).await.unwrap();
        client.get("/").dispatch().await.expect_status_success();
    }
}

mod client_error {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_client_error() {
        #[get("/")]
        fn bad_request() -> Status { Status::BadRequest }
        let client = Client::tracked(rocket::build().mount("/", routes![bad_request])).await.unwrap();
        client.get("/").dispatch().await.expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[rocket::async_test]
    async fn expect_status_client_error_should_panic_when_not() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[rocket::async_test]
    async fn should_expect_status_server_error() {
        #[get("/")]
        fn internal_server_error() -> Status { Status::InternalServerError }
        let client = Client::tracked(rocket::build().mount("/", routes![internal_server_error])).await.unwrap();
        client.get("/").dispatch().await.expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[rocket::async_test]
    async fn expect_status_server_error_should_panic_when_not() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
        client.get("/").dispatch().await.expect_status_server_error();
    }
}