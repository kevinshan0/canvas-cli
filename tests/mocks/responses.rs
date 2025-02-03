#![allow(dead_code)]

pub const COURSES_RESPONSE: &str = r#"[
    {
        "id": 1234,
        "name": "Test Course",
        "course_code": "TEST101",
        "enrollment_term_id": 1,
        "total_students": 30,
        "start_at": "2024-01-01T00:00:00Z",
        "end_at": "2024-05-01T00:00:00Z"
    }
]"#;

pub const ASSIGNMENTS_RESPONSE: &str = r#"[
    {
        "id": 4321,
        "name": "Test Assignment",
        "description": "Test Description",
        "due_at": "2024-02-01T23:59:59Z",
        "points_possible": 100.0,
        "html_url": "https://canvas.test/assignments/4321",
        "submission_types": ["online_upload"],
        "allowed_extensions": ["pdf", "doc"]
    }
]"#;

pub const FILES_RESPONSE: &str = r#"[
    {
        "id": 5678,
        "display_name": "test.pdf",
        "filename": "test.pdf",
        "content_type": "application/pdf",
        "url": "https://canvas.test/files/5678/download",
        "size": 1024,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
    }
]"#;

pub const ANNOUNCEMENTS_RESPONSE: &str = r#"[
    {
        "id": 8765,
        "title": "Test Announcement",
        "message": "This is a test announcement",
        "posted_at": "2024-01-01T00:00:00Z",
        "url": "https://canvas.test/announcements/8765"
    }
]"#;

pub const MODULES_RESPONSE: &str = r#"[
    {
        "id": 1234,
        "name": "Test Module",
        "position": 1,
        "unlock_at": null,
        "require_sequential_progress": true,
        "publish_final_grade": false,
        "prerequisite_module_ids": null,
        "state": "active",
        "completed_at": null,
        "items_count": 3,
        "items_url": "https://canvas.test/api/v1/courses/1/modules/1234/items"
    }
]"#;

pub const DISCUSSIONS_RESPONSE: &str = r#"[
    {
        "id": 4321,
        "title": "Test Discussion",
        "message": "Discussion content",
        "html_url": "https://canvas.test/courses/1/discussion_topics/4321",
        "posted_at": "2024-01-01T00:00:00Z",
        "allow_rating": true,
        "user_can_see_posts": true,
        "discussion_type": "threaded",
        "discussion_subentry_count": 5,
        "published": true,
        "locked": false,
        "pinned": false
    }
]"#;

pub const PAGES_RESPONSE: &str = r#"[
    {
        "page_id": "test-page",
        "url": "test-page",
        "title": "Test Page",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "editing_roles": "teachers",
        "published": true,
        "html_url": "https://canvas.test/courses/1/pages/test-page",
        "body": "Page content"
    }
]"#;
