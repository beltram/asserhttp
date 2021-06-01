use isahc::get;
use stubr::Stubr;

use asserhttp::*;

mod eq {
    use super::*;

    #[test]
    fn should_assert_status_eq() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).unwrap().assert_status_eq(200);
    }

    #[test]
    #[should_panic]
    fn assert_status_eq_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).unwrap().assert_status_eq(100);
    }

    #[test]
    fn result_should_assert_status_eq() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).assert_status_eq(200);
    }

    #[test]
    #[should_panic]
    fn result_assert_status_eq_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).assert_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[test]
    fn should_assert_status_ok() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_ok();
    }

    #[test]
    #[should_panic]
    fn assert_status_ok_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).unwrap().assert_status_ok();
    }

    #[test]
    fn result_should_assert_status_ok() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_ok();
    }

    #[test]
    #[should_panic]
    fn result_assert_status_ok_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).assert_status_ok();
    }
}

mod created {
    use super::*;

    #[test]
    fn should_assert_created() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).unwrap().assert_status_created();
    }

    #[test]
    #[should_panic]
    fn assert_created_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_created();
    }

    #[test]
    fn result_should_assert_created() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).assert_status_created();
    }

    #[test]
    #[should_panic]
    fn result_assert_created_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_created();
    }
}

mod accepted {
    use super::*;

    #[test]
    fn should_assert_accepted() {
        let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
        get(&srv.uri()).unwrap().assert_status_accepted();
    }

    #[test]
    #[should_panic]
    fn assert_accepted_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_accepted();
    }

    #[test]
    fn result_should_assert_accepted() {
        let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
        get(&srv.uri()).assert_status_accepted();
    }

    #[test]
    #[should_panic]
    fn result_assert_accepted_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[test]
    fn should_assert_no_content() {
        let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
        get(&srv.uri()).unwrap().assert_status_no_content();
    }

    #[test]
    #[should_panic]
    fn assert_no_content_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_no_content();
    }

    #[test]
    fn result_should_assert_no_content() {
        let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
        get(&srv.uri()).assert_status_no_content();
    }

    #[test]
    #[should_panic]
    fn result_assert_no_content_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_no_content();
    }
}

mod bad_request {
    use super::*;

    #[test]
    fn should_assert_bad_request() {
        let srv = Stubr::start_blocking("tests/stubs/status/bad-request.json");
        get(&srv.uri()).unwrap().assert_status_bad_request();
    }

    #[test]
    #[should_panic]
    fn assert_bad_request_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_bad_request();
    }

    #[test]
    fn result_should_assert_bad_request() {
        let srv = Stubr::start_blocking("tests/stubs/status/bad-request.json");
        get(&srv.uri()).assert_status_bad_request();
    }

    #[test]
    #[should_panic]
    fn result_assert_bad_request_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[test]
    fn should_assert_unauthorized() {
        let srv = Stubr::start_blocking("tests/stubs/status/unauthorized.json");
        get(&srv.uri()).unwrap().assert_status_unauthorized();
    }

    #[test]
    #[should_panic]
    fn assert_unauthorized_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_unauthorized();
    }

    #[test]
    fn result_should_assert_unauthorized() {
        let srv = Stubr::start_blocking("tests/stubs/status/unauthorized.json");
        get(&srv.uri()).assert_status_unauthorized();
    }

    #[test]
    #[should_panic]
    fn result_assert_unauthorized_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[test]
    fn should_assert_forbidden() {
        let srv = Stubr::start_blocking("tests/stubs/status/forbidden.json");
        get(&srv.uri()).unwrap().assert_status_forbidden();
    }

    #[test]
    #[should_panic]
    fn assert_forbidden_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_forbidden();
    }

    #[test]
    fn result_should_assert_forbidden() {
        let srv = Stubr::start_blocking("tests/stubs/status/forbidden.json");
        get(&srv.uri()).assert_status_forbidden();
    }

    #[test]
    #[should_panic]
    fn result_assert_forbidden_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[test]
    fn should_assert_not_found() {
        let srv = Stubr::start_blocking("tests/stubs/status/not-found.json");
        get(&srv.uri()).unwrap().assert_status_not_found();
    }

    #[test]
    #[should_panic]
    fn assert_not_found_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_not_found();
    }

    #[test]
    fn result_should_assert_not_found() {
        let srv = Stubr::start_blocking("tests/stubs/status/not-found.json");
        get(&srv.uri()).assert_status_not_found();
    }

    #[test]
    #[should_panic]
    fn result_assert_not_found_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[test]
    fn should_assert_conflict() {
        let srv = Stubr::start_blocking("tests/stubs/status/conflict.json");
        get(&srv.uri()).unwrap().assert_status_conflict();
    }

    #[test]
    #[should_panic]
    fn assert_conflict_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_conflict();
    }

    #[test]
    fn result_should_assert_conflict() {
        let srv = Stubr::start_blocking("tests/stubs/status/conflict.json");
        get(&srv.uri()).assert_status_conflict();
    }

    #[test]
    #[should_panic]
    fn result_assert_conflict_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_conflict();
    }
}

mod gone {
    use super::*;

    #[test]
    fn should_assert_gone() {
        let srv = Stubr::start_blocking("tests/stubs/status/gone.json");
        get(&srv.uri()).unwrap().assert_status_gone();
    }

    #[test]
    #[should_panic]
    fn assert_gone_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_gone();
    }

    #[test]
    fn result_should_assert_gone() {
        let srv = Stubr::start_blocking("tests/stubs/status/gone.json");
        get(&srv.uri()).assert_status_gone();
    }

    #[test]
    #[should_panic]
    fn result_assert_gone_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_gone();
    }
}

mod server_error {
    use super::*;

    #[test]
    fn should_assert_server_error() {
        let srv = Stubr::start_blocking("tests/stubs/status/server-error.json");
        get(&srv.uri()).unwrap().assert_status_server_error();
    }

    #[test]
    #[should_panic]
    fn assert_server_error_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().assert_status_server_error();
    }

    #[test]
    fn result_should_assert_server_error() {
        let srv = Stubr::start_blocking("tests/stubs/status/server-error.json");
        get(&srv.uri()).assert_status_server_error();
    }

    #[test]
    #[should_panic]
    fn result_assert_server_error_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).assert_status_server_error();
    }
}