mod models;
mod client;
mod cli;
mod db;

use clap::Parser;
use std::error::Error;
use dotenv::dotenv;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use cli::{Cli, Commands};
use client::CanvasClient;
use db::Database;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    dotenv().ok();
    let base_url = env::var("BASE_URL")?;
    let access_token = env::var("ACCESS_KEY")?;

    if (base_url.is_empty() || access_token.is_empty()) && (cli.base_url.is_none() || cli.access_token.is_none()) {
        eprintln!("Error: BASE_URL and ACCESS_KEY must be set either in .env file or as command line arguments.");
        std::process::exit(1);
    }

    let canvas = CanvasClient::new(&base_url, &access_token);

    match &cli.command {
        Commands::Courses => {
            let courses = canvas.get_courses()?;
            println!("Courses:");
            for course in courses {
                println!("  {} : {} : {}",
                    course.id, 
                    course.name.as_deref().unwrap_or("No name"),
                    course.course_code.as_deref().unwrap_or("No course code"),
                );
            }
        }
        Commands::Course(args) => {
            let course = canvas.get_course(args.course_id)?;
            println!("Course details for {}:", args.course_id);
            println!("  Name: {}", course.name.as_deref().unwrap_or("No name"));
            println!("  Code: {}", course.course_code.as_deref().unwrap_or("No code"));
        }
        Commands::Assignments(args) => {
            let assignments = canvas.get_assignments(args.course_id)?;
            println!("Assignments for course {}:", args.course_id);
            for assignment in assignments {
                println!("  {}: {}", assignment.id, assignment.name);
            }
        }
        Commands::Assignment(args) => {
            let assignment = canvas.get_assignment(args.course_id, args.assignment_id)?;
            println!("Assignment details:");
            println!("  Name: {}", assignment.name);
            println!("  Due: {}", assignment.due_at.map_or("No due date".to_string(), |d| d.to_string()));
            
            if let Some(path) = &args.file_path {
                println!("Submitting file: {}", path);
                canvas.submit_assignment(args.course_id, args.assignment_id, Path::new(path))?;
                println!("Submission successful!");
            }
        }
        Commands::Announcements(args) => {
            let announcements = canvas.get_announcements(args.course_id)?;
            println!("Announcements for course {}:", args.course_id);
            for announcement in announcements {
                println!("  {}: {}", announcement.id, announcement.title);
            }
        }
        Commands::Grades(args) => {
            let grades = canvas.get_grades(args.course_id)?;
            println!("Grades for course {}:", args.course_id);
            println!("  Current grade: {}", grades.current_grade.as_deref().unwrap_or("No grade"));
            println!("  Final grade: {}", grades.final_grade.as_deref().unwrap_or("No grade"));
        }
        Commands::Todos => {
            let todos = canvas.get_todos()?;
            println!("Todo items:");
            for todo in todos {
                let title = todo.title.as_deref().unwrap_or("Untitled");
                let due = todo.due_at.map_or("No due date".to_string(), |d| d.to_string());
                println!("  {} (Due: {})", title, due);
            }
        }
        Commands::Modules(args) => {
            let modules = canvas.get_modules(args.course_id)?;
            println!("Modules for course {}:", args.course_id);
            for module in modules {
                println!("  {}: {} ({})", 
                    module.id, 
                    module.name,
                    if module.completed_at.is_some() { "completed" } else { "in progress" }
                );
            }
        }
        Commands::ModuleItems(args) => {
            let items = canvas.get_module_items(args.course_id, args.module_id)?;
            println!("Items for module {}:", args.module_id);
            for item in items {
                println!("  {}: {} [{}]", 
                    item.id, 
                    item.title,
                    item.item_type
                );
            }
        }
        Commands::Discussions(args) => {
            let discussions = canvas.get_discussions(args.course_id)?;
            println!("Discussions for course {}:", args.course_id);
            for discussion in discussions {
                println!("  {}: {} ({} replies)", 
                    discussion.id, 
                    discussion.title,
                    discussion.discussion_subentry_count
                );
            }
        }
        Commands::DiscussionEntries(args) => {
            let entries = canvas.get_discussion_entries(args.course_id, args.topic_id)?;
            println!("Entries for discussion {}:", args.topic_id);
            for entry in entries {
                println!("  {}: {} (posted: {})", 
                    entry.id,
                    entry.message,
                    entry.created_at
                );
            }
        }
        Commands::PostDiscussion(args) => {
            let entry = canvas.post_discussion_entry(args.course_id, args.topic_id, &args.message)?;
            println!("Posted comment successfully!");
            println!("Entry ID: {}", entry.id);
        }
        Commands::Sync => {
            println!("Syncing data with local database...");
            let db_path = PathBuf::from("canvas.db");
            let db = Database::new(&db_path)?;

            // Get courses that haven't been synced in the last 6 hours
            let stale_courses = db.get_stale_courses(6)?;
            let courses = if stale_courses.is_empty() {
                canvas.get_courses()?
            } else {
                println!("Syncing {} stale courses...", stale_courses.len());
                stale_courses.iter()
                    .filter_map(|&id| canvas.get_course(id).ok())
                    .collect()
            };

            for course in &courses {
                println!("Syncing course: {}", course.name.as_deref().unwrap_or("Unnamed"));
                if let Err(e) = db.sync_course(course) {
                    eprintln!("Error syncing course {}: {}", course.id, e);
                    continue;
                }
                
                // Sync assignments with error handling
                match canvas.get_assignments(course.id) {
                    Ok(assignments) => {
                        for assignment in &assignments {
                            if let Err(e) = db.sync_assignment(course.id, assignment) {
                                eprintln!("Error syncing assignment {}: {}", assignment.id, e);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error fetching assignments for course {}: {}", course.id, e),
                }

                // Sync discussions with error handling
                match canvas.get_discussions(course.id) {
                    Ok(discussions) => {
                        for discussion in &discussions {
                            if let Err(e) = db.sync_discussion(course.id, discussion) {
                                eprintln!("Error syncing discussion {}: {}", discussion.id, e);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error fetching discussions for course {}: {}", course.id, e),
                }

                // Sync modules with error handling
                match canvas.get_modules(course.id) {
                    Ok(modules) => {
                        for module in &modules {
                            if let Err(e) = db.sync_module(course.id, module) {
                                eprintln!("Error syncing module {}: {}", module.id, e);
                                continue;
                            }
                            
                            match canvas.get_module_items(course.id, module.id) {
                                Ok(items) => {
                                    for item in &items {
                                        if let Err(e) = db.sync_module_item(module.id, item) {
                                            eprintln!("Error syncing module item {}: {}", item.id, e);
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Error fetching items for module {}: {}", module.id, e),
                            }
                        }
                    }
                    Err(e) => eprintln!("Error fetching modules for course {}: {}", course.id, e),
                }
            }
            
            // Display sync status
            match db.get_sync_status() {
                Ok(status) => {
                    println!("\nSync Status:");
                    for (type_name, last_sync) in status {
                        println!("  {}: Last synced at {}", type_name, last_sync);
                    }
                }
                Err(e) => eprintln!("Error getting sync status: {}", e),
            }
            
            println!("Sync complete!");
        }
    }

    Ok(())
}

