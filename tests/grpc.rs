use crate::grpc::grpc_client::GrpcClient;
use asserhttp::grpc::*;
use grpc::*;
use rocket::futures::FutureExt;
use tokio::net::TcpListener;
use tonic::{
    transport::{server::TcpIncoming, Channel},
    Request, Response, Status,
};

pub mod grpc {
    tonic::include_proto!("grpc");
}

#[derive(Debug, Default)]
pub struct GrpcService;

#[tonic::async_trait]
impl grpc_server::Grpc for GrpcService {
    async fn status_ok(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(grpc::Empty::default()))
    }

    async fn status_cancelled(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::cancelled(""))
    }

    async fn status_unknown(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::unknown(""))
    }

    async fn status_deadline_exceeded(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::deadline_exceeded(""))
    }

    async fn status_not_found(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::not_found(""))
    }

    async fn status_already_exists(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::already_exists(""))
    }

    async fn status_permission_denied(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::permission_denied(""))
    }

    async fn status_resource_exhausted(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::resource_exhausted(""))
    }

    async fn status_failed_precondition(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::failed_precondition(""))
    }

    async fn status_aborted(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::aborted(""))
    }

    async fn status_out_of_range(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::out_of_range(""))
    }

    async fn status_unimplemented(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented(""))
    }

    async fn status_internal(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::internal(""))
    }

    async fn status_unavailable(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::unavailable(""))
    }

    async fn status_data_loss(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::data_loss(""))
    }

    async fn status_unauthenticated(&self, _req: Request<Empty>) -> Result<Response<Empty>, Status> {
        Err(Status::unauthenticated(""))
    }

    async fn body(&self, _req: Request<Empty>) -> Result<Response<Simple>, Status> {
        Ok(Response::new(Simple {
            string: "hello".to_string(),
        }))
    }
}

struct GrpcInstance(String, Option<tokio::sync::oneshot::Sender<()>>);

impl GrpcInstance {
    fn uri(&self) -> String {
        self.0.clone()
    }
}

impl Drop for GrpcInstance {
    fn drop(&mut self) {
        self.1.take().unwrap().send(()).unwrap();
    }
}

async fn grpc_server() -> GrpcInstance {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let uri = listener.local_addr().unwrap().to_string();
    let incoming = TcpIncoming::from_listener(listener, true, None).unwrap();

    let server = tonic::transport::Server::builder()
        .add_service(grpc_server::GrpcServer::new(GrpcService))
        .serve_with_incoming_shutdown(incoming, rx.map(drop));

    tokio::spawn(server);
    tokio::time::sleep(core::time::Duration::from_millis(100)).await;
    GrpcInstance(format!("http://{uri}/"), Some(tx))
}

async fn client() -> (GrpcClient<Channel>, GrpcInstance) {
    let srv = grpc_server().await;
    let client = GrpcClient::connect(srv.uri()).await.unwrap();
    (client, srv)
}

fn req() -> Request<Empty> {
    Request::new(grpc::Empty::default())
}

pub mod status {
    use super::*;

    pub mod ok {
        use super::*;

        #[tokio::test]
        async fn grpc_status_ok_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_ok().expect_body(Empty::default());
        }

        #[should_panic(expected = "expected status to be 'OK' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_ok_should_fail() {
            let (mut client, _srv) = client().await;
            client
                .status_cancelled(req())
                .await
                .expect_status_ok()
                .expect_body(Empty::default());
        }
    }

    pub mod cancelled {
        use super::*;

        #[tokio::test]
        async fn grpc_status_cancelled_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Cancelled);
        }

        #[should_panic(expected = "expected status to be 'CANCELLED' but was 'UNKNOWN'")]
        #[tokio::test]
        async fn grpc_status_cancelled_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_unknown(req()).await.expect_status_error(Code::Cancelled);
        }

        #[should_panic(expected = "expected status to be 'CANCELLED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_cancelled_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Cancelled);
        }
    }

    pub mod unknown {
        use super::*;

        #[tokio::test]
        async fn grpc_status_unknown_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_unknown(req()).await.expect_status_error(Code::Unknown);
        }

        #[should_panic(expected = "expected status to be 'UNKNOWN' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_unknown_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Unknown);
        }

        #[should_panic(expected = "expected status to be 'UNKNOWN' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_unknown_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Unknown);
        }
    }

    pub mod deadline_exceeded {
        use super::*;

        #[tokio::test]
        async fn grpc_status_deadline_exceeded_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_deadline_exceeded(req())
                .await
                .expect_status_error(Code::DeadlineExceeded);
        }

        #[should_panic(expected = "expected status to be 'DEADLINE_EXCEEDED' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_deadline_exceeded_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::DeadlineExceeded);
        }

        #[should_panic(expected = "expected status to be 'DEADLINE_EXCEEDED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_deadline_exceeded_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::DeadlineExceeded);
        }
    }

    pub mod not_found {
        use super::*;

        #[tokio::test]
        async fn grpc_status_not_found_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_not_found(req()).await.expect_status_error(Code::NotFound);
        }

        #[should_panic(expected = "expected status to be 'NOT_FOUND' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_not_found_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::NotFound);
        }

        #[should_panic(expected = "expected status to be 'NOT_FOUND' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_not_found_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::NotFound);
        }
    }

    pub mod already_exists {
        use super::*;

        #[tokio::test]
        async fn grpc_status_already_exists_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_already_exists(req())
                .await
                .expect_status_error(Code::AlreadyExists);
        }

        #[should_panic(expected = "expected status to be 'ALREADY_EXISTS' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_already_exists_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::AlreadyExists);
        }

        #[should_panic(expected = "expected status to be 'ALREADY_EXISTS' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_already_exists_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::AlreadyExists);
        }
    }

    pub mod permission_denied {
        use super::*;

        #[tokio::test]
        async fn grpc_status_permission_denied_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_permission_denied(req())
                .await
                .expect_status_error(Code::PermissionDenied);
        }

        #[should_panic(expected = "expected status to be 'PERMISSION_DENIED' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_permission_denied_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::PermissionDenied);
        }

        #[should_panic(expected = "expected status to be 'PERMISSION_DENIED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_permission_denied_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::PermissionDenied);
        }
    }

    pub mod resource_exhausted {
        use super::*;

        #[tokio::test]
        async fn grpc_status_resource_exhausted_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_resource_exhausted(req())
                .await
                .expect_status_error(Code::ResourceExhausted);
        }

        #[should_panic(expected = "expected status to be 'RESOURCE_EXHAUSTED' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_resource_exhausted_should_fail() {
            let (mut client, _srv) = client().await;
            client
                .status_cancelled(req())
                .await
                .expect_status_error(Code::ResourceExhausted);
        }

        #[should_panic(expected = "expected status to be 'RESOURCE_EXHAUSTED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_resource_exhausted_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::ResourceExhausted);
        }
    }

    pub mod failed_precondition {
        use super::*;

        #[tokio::test]
        async fn grpc_status_failed_precondition_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_failed_precondition(req())
                .await
                .expect_status_error(Code::FailedPrecondition);
        }

        #[should_panic(expected = "expected status to be 'FAILED_PRECONDITION' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_failed_precondition_should_fail() {
            let (mut client, _srv) = client().await;
            client
                .status_cancelled(req())
                .await
                .expect_status_error(Code::FailedPrecondition);
        }

        #[should_panic(expected = "expected status to be 'FAILED_PRECONDITION' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_failed_precondition_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::FailedPrecondition);
        }
    }

    pub mod aborted {
        use super::*;

        #[tokio::test]
        async fn grpc_status_aborted_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_aborted(req()).await.expect_status_error(Code::Aborted);
        }

        #[should_panic(expected = "expected status to be 'ABORTED' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_aborted_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Aborted);
        }

        #[should_panic(expected = "expected status to be 'ABORTED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_aborted_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Aborted);
        }
    }

    pub mod out_of_range {
        use super::*;

        #[tokio::test]
        async fn grpc_status_out_of_range_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_out_of_range(req()).await.expect_status_error(Code::OutOfRange);
        }

        #[should_panic(expected = "expected status to be 'OUT_OF_RANGE' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_out_of_range_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::OutOfRange);
        }

        #[should_panic(expected = "expected status to be 'OUT_OF_RANGE' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_out_of_range_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::OutOfRange);
        }
    }

    pub mod unimplemented {
        use super::*;

        #[tokio::test]
        async fn grpc_status_unimplemented_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_unimplemented(req())
                .await
                .expect_status_error(Code::Unimplemented);
        }

        #[should_panic(expected = "expected status to be 'UNIMPLEMENTED' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_unimplemented_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Unimplemented);
        }

        #[should_panic(expected = "expected status to be 'UNIMPLEMENTED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_unimplemented_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Unimplemented);
        }
    }

    pub mod internal {
        use super::*;

        #[tokio::test]
        async fn grpc_status_internal_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_internal(req()).await.expect_status_error(Code::Internal);
        }

        #[should_panic(expected = "expected status to be 'INTERNAL' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_internal_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Internal);
        }

        #[should_panic(expected = "expected status to be 'INTERNAL' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_internal_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Internal);
        }
    }

    pub mod unavailable {
        use super::*;

        #[tokio::test]
        async fn grpc_status_unavailable_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_unavailable(req()).await.expect_status_error(Code::Unavailable);
        }

        #[should_panic(expected = "expected status to be 'UNAVAILABLE' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_unavailable_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Unavailable);
        }

        #[should_panic(expected = "expected status to be 'UNAVAILABLE' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_unavailable_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Unavailable);
        }
    }

    pub mod data_loss {
        use super::*;

        #[tokio::test]
        async fn grpc_status_data_loss_should_succeed() {
            let (mut client, _srv) = client().await;
            client.status_data_loss(req()).await.expect_status_error(Code::DataLoss);
        }

        #[should_panic(expected = "expected status to be 'DATA_LOSS' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_data_loss_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::DataLoss);
        }

        #[should_panic(expected = "expected status to be 'DATA_LOSS' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_data_loss_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::DataLoss);
        }
    }

    pub mod unauthenticated {
        use super::*;

        #[tokio::test]
        async fn grpc_status_unauthenticated_should_succeed() {
            let (mut client, _srv) = client().await;
            client
                .status_unauthenticated(req())
                .await
                .expect_status_error(Code::Unauthenticated);
        }

        #[should_panic(expected = "expected status to be 'UNAUTHENTICATED' but was 'CANCELLED'")]
        #[tokio::test]
        async fn grpc_status_unauthenticated_should_fail() {
            let (mut client, _srv) = client().await;
            client.status_cancelled(req()).await.expect_status_error(Code::Unauthenticated);
        }

        #[should_panic(expected = "expected status to be 'UNAUTHENTICATED' but was 'OK'")]
        #[tokio::test]
        async fn grpc_status_unauthenticated_should_fail_when_ok() {
            let (mut client, _srv) = client().await;
            client.status_ok(req()).await.expect_status_error(Code::Unauthenticated);
        }
    }
}

pub mod body {
    use super::*;

    #[tokio::test]
    async fn grpc_body_should_succeed_when_eq() {
        let (mut client, _srv) = client().await;
        client.body(req()).await.expect_status_ok().expect_body(Simple {
            string: "hello".to_string(),
        });
    }

    #[should_panic(expected = "expected gRPC body to be:\n  Simple { string: \"world\" }\nbut was:\n  Simple { string: \"hello\" }")]
    #[tokio::test]
    async fn grpc_body_should_fail_when_not_eq() {
        let (mut client, _srv) = client().await;
        client.body(req()).await.expect_status_ok().expect_body(Simple {
            string: "world".to_string(),
        });
    }
}
