use canvas_cli::client::CanvasClient;
use mockito::Server;
use std::error::Error;

mod mocks;
use mocks::responses::*;

#[test]
fn test_get_courses() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    
    let _mock = server.mock("GET", "/api/v1/courses")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(COURSES_RESPONSE)
        .create();

    let client = CanvasClient::new(&server.url(), "fake-token");
    let courses = client.get_courses()?;
    
    assert_eq!(courses.len(), 1);
    assert_eq!(courses[0].id, 1234);
    
    Ok(())
}

#[test]
fn test_get_assignments() -> Result<(), Box<dyn Error>> {
    let mut server = Server::new();
    
    let _mock = server.mock("GET", "/api/v1/courses/1234/assignments")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(ASSIGNMENTS_RESPONSE)
        .create();

    let client = CanvasClient::new(&server.url(), "fake-token");
    let assignments = client.get_assignments(1234)?;
    
    assert_eq!(assignments.len(), 1);
    assert_eq!(assignments[0].id, 4321);
    
    Ok(())
}

#[test]
fn test_error_handling() {
    let mut server = Server::new();
    
    let _mock = server.mock("GET", "/api/v1/courses")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"errors": ["Unauthorized"]}"#)
        .create();

    let client = CanvasClient::new(&server.url(), "invalid-token");
    let result = client.get_courses();
    
    assert!(result.is_err());
}
