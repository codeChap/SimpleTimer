use clap::Parser;
use std::time::Instant;

use std::fs::{OpenOptions, File};
use std::io::{stdin, stdout, Write as IoWrite, Read}; // Use alias for Write
use std::path::PathBuf;
use dirs;
use chrono; // Ensure chrono is explicitly used or imported if needed for time formatting

/// Simple command-line timer that logs time spent on tasks to a CSV file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The name of the task being tracked. If omitted, you will be prompted.
    #[arg(short, long, value_name = "TASK_NAME")]
    task: Option<String>,

    /// Optional code to associate with the task entry in the log. If omitted, you will be prompted.
    #[arg(short, long, value_name = "CODE")]
    code: Option<String>,
}

fn main() {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Determine the task name: use from args or prompt if missing
    let task_name = match cli.task {
        Some(t) => t, // Use task name from argument
        None => {
            // Prompt user for task name
            print!("Enter task name: ");
            stdout().flush().expect("Failed to flush stdout"); // Ensure prompt appears before input

            let mut user_task = String::new();
            stdin().read_line(&mut user_task)
                 .expect("Failed to read task name from stdin");

            user_task.trim().to_string() // Trim whitespace
        }
    };

    // Ensure task name is not empty, default to "Unnamed Task" if it is after trimming
    let task_name = if task_name.is_empty() {
        println!("Task name cannot be empty, using 'Unnamed Task'.");
        "Unnamed Task".to_string()
    } else {
        task_name
    };


    // Determine the code: use from args or prompt if missing
    let code = match cli.code {
        Some(c) => c, // Use code from argument
        None => {
            // Prompt user for code
            print!("Enter code for this task: ");
            stdout().flush().expect("Failed to flush stdout"); // Ensure prompt appears before input

            let mut user_code = String::new();
            stdin().read_line(&mut user_code)
                 .expect("Failed to read code from stdin");

            user_code.trim().to_string() // Trim whitespace
        }
    };

    // Ensure code is not empty, default to "NA" if it is after trimming
    let code = if code.is_empty() {
        println!("Code cannot be empty, using 'NA'.");
        "NA".to_string()
    } else {
        code
    };

    println!("Tracking task '{}' with code '{}'. Press Ctrl+C to stop.", task_name, code);
    let start_time = Instant::now();

    // Clone the final task_name and code to move them into the closure
    let task_name_clone = task_name.clone();
    let code_clone = code.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        let duration_secs = start_time.elapsed().as_secs();
        let total_minutes = duration_secs / 60;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;
        let seconds = duration_secs % 60; // Still needed for console output

        // Log to console (keep showing seconds here for immediate feedback)
        println!("\nStopped. Time spent on task '{}' (Code: {}): {}h {}m {}s", task_name_clone, code_clone, hours, minutes, seconds);

        // Get home directory and create path for time log (CSV)
        let mut log_path = dirs::home_dir().expect("Could not find home directory");
        log_path.push("time_log.csv");

        // Create CSV with headers if needed
        create_csv_with_headers_if_needed(&log_path);

        // Escape task name for CSV (replace quotes with double quotes)
        let escaped_task_name = task_name_clone.replace("\"", "\"\"");
        // Escape code for CSV
        let escaped_code = code_clone.replace("\"", "\"\"");

        // Format as CSV: date, time, code, task name, duration (hours), duration (minutes)
        let log_entry = format!(
            "{},{},\"{}\",\"{}\",{},{}\n",
            chrono::Local::now().format("%Y-%m-%d"),
            chrono::Local::now().format("%H:%M:%S"), // Keep precise time of logging
            escaped_code, // Use the escaped code
            escaped_task_name,
            hours,
            minutes
        );

        let mut file = OpenOptions::new()
            .append(true)
            .create(true) // Ensure file is created if it doesn't exist after header check
            .open(&log_path)
            .expect("Failed to open log file");

        // Note: std::io::Write was imported as IoWrite, but file.write_all uses the trait implicitly.
        file.write_all(log_entry.as_bytes())
            .expect("Failed to write to log file");

        std::process::exit(0);
    }).expect("Error setting Ctrl+C handler");

    // Keep the program running and display elapsed time
    loop {
        let elapsed = start_time.elapsed();
        let total_seconds = elapsed.as_secs();

        // Calculate hours, minutes, and seconds
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        // Format the time string (hh:mm:ss)
        let time_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

        // Print on the same line using carriage return \r
        // Keep the original tracking message and append elapsed time
        print!("\rTracking task '{}' with code '{}'. Elapsed: {}", task_name, code, time_str);
        stdout().flush().expect("Failed to flush stdout");

        // Sleep for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn create_csv_with_headers_if_needed(path: &PathBuf) {
    // Check if file exists and is empty
    let file_exists = path.exists();
    let file_empty = if file_exists {
        match File::open(path) {
            Ok(mut file) => {
                let mut content = String::new();
                // Check if read results in 0 bytes read, indicating empty or error
                file.read_to_string(&mut content).unwrap_or(0) == 0 && content.is_empty()
            },
            Err(_) => true, // Treat error opening as if file needs creation/headers
        }
    } else {
        true // File doesn't exist, needs creation and headers
    };

    // Create file with headers if it doesn't exist or is empty
    if !file_exists || file_empty {
        match File::create(path) {
            Ok(mut file) => {
                let headers = "Date,Time,Code,Task,Hours,Minutes\n";
                // Note: std::io::Write was imported as IoWrite, but file.write_all uses the trait implicitly.
                file.write_all(headers.as_bytes()).expect("Failed to write headers");
            },
            Err(e) => {
                eprintln!("Failed to create CSV file '{}': {}", path.display(), e);
                // Optionally exit or handle the error differently
                std::process::exit(1);
            }
        }
    }
}