use canvas_cli::models::*;

mod mocks;
use mocks::responses::*;

#[test]
fn test_course_deserialization() {
    let courses: Vec<Course> = serde_json::from_str(COURSES_RESPONSE).unwrap();
    assert_eq!(courses.len(), 1);
    
    let course = &courses[0];
    assert_eq!(course.id, 1234);
    assert_eq!(course.name, Some("Test Course".to_string()));
    assert_eq!(course.course_code, Some("TEST101".to_string()));
}

#[test]
fn test_assignment_deserialization() {
    let assignments: Vec<Assignment> = serde_json::from_str(ASSIGNMENTS_RESPONSE).unwrap();
    assert_eq!(assignments.len(), 1);
    
    let assignment = &assignments[0];
    assert_eq!(assignment.id, 4321);
    assert_eq!(assignment.name, "Test Assignment");
    assert_eq!(assignment.points_possible, Some(100.0));
}

#[test]
fn test_file_deserialization() {
    let files: Vec<CanvasFile> = serde_json::from_str(FILES_RESPONSE).unwrap();
    assert_eq!(files.len(), 1);
    
    let file = &files[0];
    assert_eq!(file.id, 5678);
    assert_eq!(file.display_name, "test.pdf");
    assert_eq!(file.content_type, "application/pdf");
}
