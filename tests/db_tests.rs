use canvas_cli::db::Database;
use canvas_cli::models::*;
use chrono::{Duration, Utc};
use rusqlite::params;
use tempfile::tempdir;

fn create_test_db() -> (Database, tempfile::TempDir) {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::new(&db_path).unwrap();
    (db, temp_dir)
}

fn create_test_course() -> Course {
    Course {
        id: 1,
        name: Some("Test Course".to_string()),
        course_code: Some("TEST101".to_string()),
        enrollment_term_id: Some(1),
        total_students: Some(30),
        start_at: Some(Utc::now()),
        end_at: Some(Utc::now() + Duration::days(90)),
    }
}

fn create_test_assignment() -> Assignment {
    Assignment {
        id: 1,
        name: "Test Assignment".to_string(),
        description: Some("Test Description".to_string()),
        due_at: Some(Utc::now() + Duration::days(7)),
        points_possible: Some(100.0),
        html_url: "http://test.com/assignment".to_string(),
        submission_types: vec!["online_upload".to_string()],
        allowed_extensions: Some(vec!["pdf".to_string()]),
    }
}

fn create_test_discussion() -> Discussion {
    Discussion {
        id: 1,
        title: "Test Discussion".to_string(),
        message: "Test Message".to_string(),
        html_url: "http://test.com/discussion".to_string(),
        posted_at: Some(Utc::now()),
        allow_rating: true,
        user_can_see_posts: true,
        discussion_type: "threaded".to_string(),
        discussion_subentry_count: 0,
        published: true,
        locked: false,
        pinned: true,
    }
}

fn create_test_module() -> Module {
    Module {
        id: 1,
        name: "Test Module".to_string(),
        position: 1,
        unlock_at: Some(Utc::now()),
        require_sequential_progress: false,
        publish_final_grade: false,
        prerequisite_module_ids: None,
        state: "active".to_string(),
        completed_at: None,
        items_count: 0,
        items_url: "http://test.com/items".to_string(),
    }
}

fn create_test_module_item(module_id: u64) -> ModuleItem {
    ModuleItem {
        id: 1,
        module_id,
        position: 1,
        title: "Test Item".to_string(),
        indent: 0,
        item_type: "Assignment".to_string(),
        html_url: "http://test.com/item".to_string(),
        content_id: Some(1),
        external_url: None,
        completion_requirement: None,
    }
}

#[test]
fn test_database_initialization() {
    let (db, _temp) = create_test_db();
    let tables = db.get_connection().prepare("SELECT name FROM sqlite_master WHERE type='table'").unwrap()
        .query_map([], |row| row.get::<_, String>(0)).unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    
    assert!(tables.contains(&"courses".to_string()));
    assert!(tables.contains(&"assignments".to_string()));
    assert!(tables.contains(&"discussions".to_string()));
    assert!(tables.contains(&"modules".to_string()));
    assert!(tables.contains(&"module_items".to_string()));
}

#[test]
fn test_sync_course() {
    let (db, _temp) = create_test_db();
    let course = create_test_course();
    
    assert!(db.sync_course(&course).is_ok());
    
    let saved_name: String = db.get_connection().query_row(
        "SELECT name FROM courses WHERE id = ?1",
        params![course.id],
        |row| row.get(0)
    ).unwrap();
    
    assert_eq!(saved_name, course.name.unwrap());
}

#[test]
fn test_sync_assignment() {
    let (db, _temp) = create_test_db();
    let course = create_test_course();
    let assignment = create_test_assignment();
    
    db.sync_course(&course).unwrap();
    assert!(db.sync_assignment(course.id, &assignment).is_ok());
    
    let saved_name: String = db.get_connection().query_row(
        "SELECT name FROM assignments WHERE id = ?1",
        params![assignment.id],
        |row| row.get(0)
    ).unwrap();
    
    assert_eq!(saved_name, assignment.name);
}

#[test]
fn test_sync_discussion() {
    let (db, _temp) = create_test_db();
    let course = create_test_course();
    let discussion = create_test_discussion();
    
    db.sync_course(&course).unwrap();
    assert!(db.sync_discussion(course.id, &discussion).is_ok());
    
    let saved_title: String = db.get_connection().query_row(
        "SELECT title FROM discussions WHERE id = ?1",
        params![discussion.id],
        |row| row.get(0)
    ).unwrap();
    
    assert_eq!(saved_title, discussion.title);
}

#[test]
fn test_get_stale_courses() {
    let (db, _temp) = create_test_db();
    let course = create_test_course();
    
    db.sync_course(&course).unwrap();
    
    db.get_connection().execute(
        "UPDATE courses SET last_sync = datetime('now', '-7 hours') WHERE id = ?1",
        params![course.id]
    ).unwrap();
    
    let stale_courses = db.get_stale_courses(6).unwrap();
    assert!(stale_courses.contains(&course.id));
    
    db.sync_course(&course).unwrap();
    let fresh_courses = db.get_stale_courses(6).unwrap();
    assert!(!fresh_courses.contains(&course.id));
}

#[test]
fn test_sync_module() {
    let (db, _temp) = create_test_db();
    let course = create_test_course();
    let module = create_test_module();
    
    db.sync_course(&course).unwrap();
    assert!(db.sync_module(course.id, &module).is_ok());
    
    let saved_name: String = db.get_connection().query_row(
        "SELECT name FROM modules WHERE id = ?1",
        params![module.id],
        |row| row.get(0)
    ).unwrap();
    
    assert_eq!(saved_name, module.name);
}

#[test]
fn test_sync_module_item() {
    let (db, _temp) = create_test_db();
    let course = create_test_course();
    let module = create_test_module();
    let item = create_test_module_item(module.id);
    
    db.sync_course(&course).unwrap();
    db.sync_module(course.id, &module).unwrap();
    assert!(db.sync_module_item(module.id, &item).is_ok());
    
    let saved_title: String = db.get_connection().query_row(
        "SELECT title FROM module_items WHERE id = ?1",
        params![item.id],
        |row| row.get(0)
    ).unwrap();
    
    assert_eq!(saved_title, item.title);
}

#[test]
fn test_sync_status() {
    let (db, _temp) = create_test_db();
    
    assert!(db.update_sync_status("courses").is_ok());
    
    let status = db.get_sync_status().unwrap();
    assert!(!status.is_empty());
    assert_eq!(status[0].0, "courses");
}

#[test]
fn test_get_connection() {
    let (db, _temp) = create_test_db();
    let conn = db.get_connection();
    
    // Test that we can execute a simple query
    let result: Result<i32, _> = conn.query_row("SELECT 1", [], |row| row.get(0));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}
