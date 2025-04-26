# Simple Timer

A simple command-line timer built with Rust for tracking time spent on tasks and logging them to a CSV file.

## Features

*   Track time spent on specific tasks.
*   Associate a code with each task entry.
*   Prompt for task name and code if not provided via command-line arguments.
*   Log entries to a CSV file in your home directory (`time_log.csv`).
*   Displays elapsed time while the timer is running.

## Installation

Clone this repro and run:
```bash
cargo build --release
```

## Usage

Run the timer from your terminal.

You can provide the task name and code as arguments:

```bash
timer --task "Work on feature X" --code "FEAT-42"
```

If you omit the arguments, the tool will prompt you:

The timer will start, and the elapsed time will be displayed in your terminal.

## Stopping the Timer

Press `Ctrl+C` to stop the timer. The elapsed time will be calculated, and an entry will be logged to the CSV file.

## Log File

Task entries are logged to a CSV file named `time_log.csv` in your home directory (`~` on Linux/macOS, `%USERPROFILE%` on Windows).

The format of the CSV is:

`Date,Time,Code,Task,Hours,Minutes`

The duration is rounded down to the nearest minute for logging purposes (although the console output shows seconds upon stopping).
