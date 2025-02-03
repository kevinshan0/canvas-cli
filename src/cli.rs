use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "canvas-cli", about = "CLI tool for Canvas API", version = "0.1.0")]
pub struct Cli {
    #[arg(short, long)]
    pub base_url: Option<String>,

    #[arg(short, long)]
    pub access_token: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all courses
    Courses,
    /// Show course details and assignments
    Course(CourseArgs),
    /// List upcoming assignments
    Assignments(CourseArgs),
    /// Show assignment details and submit work
    Assignment(AssignmentArgs),
    /// List course announcements
    Announcements(CourseArgs),
    /// Show grades for a course
    Grades(CourseArgs),
    /// List upcoming todos
    Todos,
    /// List course modules
    Modules(CourseArgs),
    /// Show module items
    ModuleItems(ModuleArgs),
    /// List course discussions
    Discussions(CourseArgs),
    /// Show discussion entries
    DiscussionEntries(DiscussionArgs),
    /// Post to discussion
    PostDiscussion(PostDiscussionArgs),
    /// Sync data with local database
    Sync,
}

#[derive(Args)]
pub struct CourseArgs {
    /// Course ID
    pub course_id: u64,
}

#[derive(Args)]
pub struct AssignmentArgs {
    /// Course ID
    pub course_id: u64,
    /// Assignment ID
    pub assignment_id: u64,
    /// File to submit (optional)
    #[arg(long)]
    pub file_path: Option<String>,
}

#[derive(Args)]
pub struct ModuleArgs {
    /// Course ID
    pub course_id: u64,
    /// Module ID
    pub module_id: u64,
}

#[derive(Args)]
pub struct DiscussionArgs {
    /// Course ID
    pub course_id: u64,
    /// Discussion Topic ID
    pub topic_id: u64,
}

#[derive(Args)]
pub struct PostDiscussionArgs {
    /// Course ID
    pub course_id: u64,
    /// Discussion Topic ID
    pub topic_id: u64,
    /// Message to post
    pub message: String,
}
