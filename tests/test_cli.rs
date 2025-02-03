use canvas_cli::cli::{Cli, Commands};
use clap::Parser;

#[test]
fn test_courses_command() {
    let cli = Cli::try_parse_from(&["canvas-cli", "courses"]).unwrap();
    match cli.command {
        Commands::Courses => (),
        _ => panic!("Expected Courses command"),
    }
}

#[test]
fn test_course_command() {
    let cli = Cli::try_parse_from(&["canvas-cli", "course", "1234"]).unwrap();
    match cli.command {
        Commands::Course(args) => assert_eq!(args.course_id, 1234),
        _ => panic!("Expected Course command"),
    }
}

#[test]
fn test_assignment_command() {
    let cli = Cli::try_parse_from(&[
        "canvas-cli", "assignment",
        "1234",  // course_id
        "4321",  // assignment_id
        "--file-path", "test.pdf"
    ]).unwrap();
    
    match cli.command {
        Commands::Assignment(args) => {
            assert_eq!(args.course_id, 1234);
            assert_eq!(args.assignment_id, 4321);
            assert_eq!(args.file_path, Some("test.pdf".to_string()));
        },
        _ => panic!("Expected Assignment command"),
    }
}
