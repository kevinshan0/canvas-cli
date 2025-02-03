use crate::models::*;
use reqwest::blocking::{Client, multipart};
use std::error::Error;
use std::path::Path;
use serde_json;

pub struct CanvasClient {
    base_url: String,
    access_token: String,
    client: Client,
}

impl CanvasClient {
    pub fn new(base_url: &str, access_token: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            access_token: access_token.to_string(),
            client: Client::new(),
        }
    }

    fn get<T: for<'de> serde::Deserialize<'de>>(&self, url: &str) -> Result<T, Box<dyn Error>> {
        let resp = self.client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()?
            .error_for_status()?;
        Ok(resp.json()?)
    }

    pub fn get_courses(&self) -> Result<Vec<Course>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses", self.base_url);
        self.get(&url)
    }

    pub fn get_course(&self, course_id: u64) -> Result<Course, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}", self.base_url, course_id);
        self.get(&url)
    }

    pub fn get_assignments(&self, course_id: u64) -> Result<Vec<Assignment>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/assignments", self.base_url, course_id);
        self.get(&url)
    }

    pub fn get_assignment(&self, course_id: u64, assignment_id: u64) -> Result<Assignment, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/assignments/{}", self.base_url, course_id, assignment_id);
        self.get(&url)
    }

    pub fn submit_assignment(&self, course_id: u64, assignment_id: u64, file_path: &Path) -> Result<Submission, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/assignments/{}/submissions", self.base_url, course_id, assignment_id);
        
        let form = multipart::Form::new()
            .file("file", file_path)?;

        let resp = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .multipart(form)
            .send()?
            .error_for_status()?;
        
        Ok(resp.json()?)
    }

    pub fn get_announcements(&self, course_id: u64) -> Result<Vec<Announcement>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/announcements", self.base_url, course_id);
        self.get(&url)
    }

    pub fn get_grades(&self, course_id: u64) -> Result<Grade, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/grades", self.base_url, course_id);
        self.get(&url)
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let url = format!("{}/api/v1/users/self/todo", self.base_url);
        self.get(&url)
    }

    pub fn get_modules(&self, course_id: u64) -> Result<Vec<Module>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/modules", self.base_url, course_id);
        self.get(&url)
    }

    pub fn get_module_items(&self, course_id: u64, module_id: u64) -> Result<Vec<ModuleItem>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/modules/{}/items", self.base_url, course_id, module_id);
        self.get(&url)
    }

    pub fn get_discussions(&self, course_id: u64) -> Result<Vec<Discussion>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/discussion_topics", self.base_url, course_id);
        self.get(&url)
    }

    pub fn get_discussion_entries(&self, course_id: u64, topic_id: u64) -> Result<Vec<DiscussionEntry>, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/discussion_topics/{}/entries", 
            self.base_url, course_id, topic_id);
        self.get(&url)
    }

    pub fn post_discussion_entry(&self, course_id: u64, topic_id: u64, message: &str) -> Result<DiscussionEntry, Box<dyn Error>> {
        let url = format!("{}/api/v1/courses/{}/discussion_topics/{}/entries", 
            self.base_url, course_id, topic_id);
        
        let resp = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({ "message": message }))
            .send()?
            .error_for_status()?;
        
        Ok(resp.json()?)
    }
}
