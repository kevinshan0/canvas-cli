use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct Course {
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub course_code: Option<String>,
    pub enrollment_term_id: Option<u64>,
    pub total_students: Option<u64>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Assignment {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub due_at: Option<DateTime<Utc>>,
    pub points_possible: Option<f64>,
    pub html_url: String,
    pub submission_types: Vec<String>,
    pub allowed_extensions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Submission {
    pub id: u64,
    pub assignment_id: u64,
    pub score: Option<f64>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub late: bool,
    pub missing: bool,
    pub grade: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CanvasFile {
    pub id: u64,
    pub display_name: String,
    pub filename: String,
    pub content_type: String,
    pub url: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Announcement {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub posted_at: DateTime<Utc>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub assignment_id: Option<u64>,
    pub due_at: Option<DateTime<Utc>>,
    pub html_url: String,
    pub course_id: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Grade {
    pub course_id: u64,
    pub current_grade: Option<String>,
    pub final_grade: Option<String>,
    pub current_score: Option<f64>,
    pub final_score: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Module {
    pub id: u64,
    pub name: String,
    pub position: i32,
    pub unlock_at: Option<DateTime<Utc>>,
    pub require_sequential_progress: bool,
    pub publish_final_grade: bool,
    pub prerequisite_module_ids: Option<Vec<u64>>,
    pub state: String,
    pub completed_at: Option<DateTime<Utc>>,
    pub items_count: i32,
    pub items_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ModuleItem {
    pub id: u64,
    pub module_id: u64,
    pub position: i32,
    pub title: String,
    pub indent: i32,
    #[serde(rename = "type")]
    pub item_type: String,
    pub html_url: String,
    pub content_id: Option<u64>,
    pub external_url: Option<String>,
    pub completion_requirement: Option<CompletionRequirement>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CompletionRequirement {
    #[serde(rename = "type")]
    pub requirement_type: String,
    pub min_score: Option<f64>,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Discussion {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub html_url: String,
    #[serde(default)]
    pub posted_at: Option<DateTime<Utc>>,  // Change to Option
    pub allow_rating: bool,
    pub user_can_see_posts: bool,
    pub discussion_type: String,
    pub discussion_subentry_count: i32,
    pub published: bool,
    pub locked: bool,
    pub pinned: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DiscussionEntry {
    pub id: u64,
    pub user_id: u64,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub rating_count: Option<i32>,
    pub rating_sum: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Page {
    pub page_id: String,
    pub url: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub editing_roles: String,
    pub published: bool,
    pub html_url: String,
    pub body: Option<String>,
}
