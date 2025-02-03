use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use crate::models::*;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<()> {
        // Create courses table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS courses (
                id INTEGER PRIMARY KEY,
                name TEXT,
                course_code TEXT,
                enrollment_term_id INTEGER,
                total_students INTEGER,
                start_at TEXT,
                end_at TEXT,
                last_sync TEXT NOT NULL
            )",
            [],
        )?;

        // Create assignments table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS assignments (
                id INTEGER PRIMARY KEY,
                course_id INTEGER,
                name TEXT NOT NULL,
                description TEXT,
                due_at TEXT,
                points_possible REAL,
                html_url TEXT,
                last_sync TEXT NOT NULL,
                FOREIGN KEY(course_id) REFERENCES courses(id)
            )",
            [],
        )?;

        // Create discussions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS discussions (
                id INTEGER PRIMARY KEY,
                course_id INTEGER,
                title TEXT NOT NULL,
                message TEXT,
                posted_at TEXT,
                discussion_type TEXT,
                pinned BOOLEAN,
                last_sync TEXT NOT NULL,
                FOREIGN KEY(course_id) REFERENCES courses(id)
            )",
            [],
        )?;

        // Create modules table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS modules (
                id INTEGER PRIMARY KEY,
                course_id INTEGER,
                name TEXT NOT NULL,
                position INTEGER,
                unlock_at TEXT,
                state TEXT,
                completed_at TEXT,
                last_sync TEXT NOT NULL,
                FOREIGN KEY(course_id) REFERENCES courses(id)
            )",
            [],
        )?;

        // Create module items table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS module_items (
                id INTEGER PRIMARY KEY,
                module_id INTEGER,
                position INTEGER,
                title TEXT NOT NULL,
                item_type TEXT,
                content_id INTEGER,
                last_sync TEXT NOT NULL,
                FOREIGN KEY(module_id) REFERENCES modules(id)
            )",
            [],
        )?;

        // Create sync_status table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sync_status (
                type_name TEXT PRIMARY KEY,
                last_sync TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    /// Get a reference to the underlying database connection
    #[allow(dead_code)]
    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    #[allow(dead_code)]
    pub fn update_sync_status(&self, type_name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sync_status (type_name, last_sync)
            VALUES (?1, ?2)",
            params![
                type_name,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn sync_course(&self, course: &Course) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO courses (id, name, course_code, enrollment_term_id, 
                total_students, start_at, end_at, last_sync)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                course.id,
                course.name,
                course.course_code,
                course.enrollment_term_id,
                course.total_students,
                course.start_at.map(|dt| dt.to_rfc3339()),
                course.end_at.map(|dt| dt.to_rfc3339()),
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn sync_assignment(&self, course_id: u64, assignment: &Assignment) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO assignments (id, course_id, name, description, 
                due_at, points_possible, html_url, last_sync)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                assignment.id,
                course_id,
                assignment.name,
                assignment.description,
                assignment.due_at.map(|dt| dt.to_rfc3339()),
                assignment.points_possible,
                assignment.html_url,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn sync_discussion(&self, course_id: u64, discussion: &Discussion) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO discussions (id, course_id, title, message, 
                posted_at, discussion_type, pinned, last_sync)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                discussion.id,
                course_id,
                discussion.title,
                discussion.message,
                discussion.posted_at.map(|dt| dt.to_rfc3339()),
                discussion.discussion_type,
                discussion.pinned,
                Utc::now().to_rfc3339()
            ],
        )?;
        
        // Update sync status after successful sync
        self.update_sync_status("discussions")?;
        Ok(())
    }

    pub fn sync_module(&self, course_id: u64, module: &Module) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO modules (id, course_id, name, position, 
                unlock_at, state, completed_at, last_sync)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                module.id,
                course_id,
                module.name,
                module.position,
                module.unlock_at.map(|dt| dt.to_rfc3339()),
                module.state,
                module.completed_at.map(|dt| dt.to_rfc3339()),
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn sync_module_item(&self, module_id: u64, item: &ModuleItem) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO module_items (id, module_id, position, 
                title, item_type, content_id, last_sync)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                item.id,
                module_id,
                item.position,
                item.title,
                item.item_type,
                item.content_id,
                Utc::now().to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn get_stale_courses(&self, hours: i64) -> Result<Vec<u64>> {
        let mut stmt = self.conn.prepare(
            "SELECT id FROM courses 
            WHERE datetime(last_sync) < datetime('now', ?1 || ' hours')"
        )?;
        
        let course_ids: Result<Vec<u64>> = stmt
            .query_map([format!("-{}", hours)], |row| row.get(0))?
            .collect();

        course_ids
    }

    pub fn get_sync_status(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT type_name, last_sync FROM sync_status"
        )?;
        
        let status = stmt.query_map([], |row| {
            let last_sync_str: String = row.get(1)?;
            let last_sync = DateTime::parse_from_rfc3339(&last_sync_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            
            Ok((row.get(0)?, last_sync))
        })?;

        status.collect()
    }
}