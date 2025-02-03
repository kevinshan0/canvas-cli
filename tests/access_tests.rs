use canvas_cli::client::CanvasClient;
use dotenv::dotenv;
use std::env;
use reqwest::StatusCode;

fn setup_client() -> CanvasClient {
    dotenv().ok();
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let access_token = env::var("ACCESS_KEY").expect("ACCESS_KEY must be set");
    CanvasClient::new(&base_url, &access_token)
}

fn is_forbidden(err: &Box<dyn std::error::Error>) -> bool {
    if let Some(req_err) = err.downcast_ref::<reqwest::Error>() {
        if let Some(status) = req_err.status() {
            return status == StatusCode::FORBIDDEN;
        }
    }
    false
}

#[test]
fn test_course_access() {
    let canvas = setup_client();
    let course_id = env::var("TEST_COURSE_ID")
        .expect("TEST_COURSE_ID must be set")
        .parse::<u64>()
        .unwrap();

    println!("\nTesting Course-related endpoints:");
    
    // Test getting course list
    match canvas.get_courses() {
        Ok(_) => println!("✓ get_courses: Accessible"),
        Err(e) => println!("✗ get_courses: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
    }

    // Test getting specific course
    match canvas.get_course(course_id) {
        Ok(_) => println!("✓ get_course: Accessible"),
        Err(e) => println!("✗ get_course: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
    }
}

#[test]
fn test_assignment_access() {
    let canvas = setup_client();
    let course_id = env::var("TEST_COURSE_ID")
        .expect("TEST_COURSE_ID must be set")
        .parse::<u64>()
        .unwrap();

    println!("\nTesting Assignment-related endpoints:");
    
    // Test getting assignments
    match canvas.get_assignments(course_id) {
        Ok(assignments) => {
            println!("✓ get_assignments: Accessible");
            if let Some(assignment) = assignments.first() {
                match canvas.get_assignment(course_id, assignment.id) {
                    Ok(_) => println!("✓ get_assignment: Accessible"),
                    Err(e) => println!("✗ get_assignment: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
                }
            }
        }
        Err(e) => println!("✗ get_assignments: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
    }
}

#[test]
fn test_discussion_access() {
    let canvas = setup_client();
    let course_id = env::var("TEST_COURSE_ID")
        .expect("TEST_COURSE_ID must be set")
        .parse::<u64>()
        .unwrap();

    println!("\nTesting Discussion-related endpoints:");
    
    // Test getting discussions
    match canvas.get_discussions(course_id) {
        Ok(discussions) => {
            println!("✓ get_discussions: Accessible");
            if let Some(discussion) = discussions.first() {
                match canvas.get_discussion_entries(course_id, discussion.id) {
                    Ok(_) => println!("✓ get_discussion_entries: Accessible"),
                    Err(e) => println!("✗ get_discussion_entries: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
                }
            }
        }
        Err(e) => println!("✗ get_discussions: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
    }
}

#[test]
fn test_module_access() {
    let canvas = setup_client();
    let course_id = env::var("TEST_COURSE_ID")
        .expect("TEST_COURSE_ID must be set")
        .parse::<u64>()
        .unwrap();

    println!("\nTesting Module-related endpoints:");
    
    // Test getting modules
    match canvas.get_modules(course_id) {
        Ok(modules) => {
            println!("✓ get_modules: Accessible");
            if let Some(module) = modules.first() {
                match canvas.get_module_items(course_id, module.id) {
                    Ok(_) => println!("✓ get_module_items: Accessible"),
                    Err(e) => println!("✗ get_module_items: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
                }
            }
        }
        Err(e) => println!("✗ get_modules: {}", if is_forbidden(&e) { "Forbidden" } else { "Error" }),
    }
}
