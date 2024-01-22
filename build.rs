fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/authentication.proto")?;
    Ok(())
}

/// Session based authentication
/// Microservices
/// CI/CD

//* Stage1 : Implementing the services & client.
/// Implement the auth and health check services.
/// Set-up communication between them.
/// Create a stand-alone client that can call the auth service.

//* Stage2 : Docker & CI/CD
/// Dockerize our app.
/// Add continuous integration via GitHub Actions.
/// Implement continuous deployment to DigitalOcean.

/// <작업 순서>
/// 새 프로젝트를 만들고 Stage1-Step1 내용을 복사한다. (Stage1-Step1)
/// cargo new microservice-project
/// 
/// auth 서비스 구현 (Stage1-Step2)
//* - auth-service/users.rs
///    creating, storing, retrieving, deleting user.
//* - auth-service/sessions.rs
///    creating, storing, retrieving, deleting session.
//* - auth-service/auth.rs
///    the auth service interface as defined in authentication.proto
//* - auth-service/main.rs
///    create an instance of the auth service and start the gRPC server.
///
/// health check service 구현 (Stage1-Step3)
//* - health-check/main.rs
///    continuously make gRPC requests to the auth services and print out the responses.
///
///
/// (Stage2-Step1)
///
/// 
/// 
/// 
fn _nouse() {}



