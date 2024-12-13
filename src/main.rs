use std::io::{self, Read, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::env;
use timeout_readwrite::TimeoutReader;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        eprintln!("Usage: {} <command> [args...]", env::args().next().unwrap());
        std::process::exit(1);
    }

    let command = args[0].clone();
    let command_args = &args[1..];

    // Buffer for accumulating stdin
    let mut current_buffer = Vec::new();
    
    // Create a timeout reader for stdin with 1 second timeout
    let stdin = io::stdin();
    let timeout_reader = TimeoutReader::new(stdin, Duration::from_secs(1));
    let mut reader = BufReader::new(timeout_reader);
    
    // Track time for 1-second intervals
    let mut last_execution = Instant::now();

    loop {
        // Read with 1 second timeout
        let mut buf = [0; 1024];
        match reader.read(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => current_buffer.extend_from_slice(&buf[..n]),
            Err(e) => {
                if e.kind() == io::ErrorKind::TimedOut {
                    // Expected timeout - continue to process execution
                } else if e.kind() != io::ErrorKind::Interrupted {
                    return Err(e);
                }
            }
        }

        let elapsed = last_execution.elapsed();
        if elapsed >= Duration::from_secs(1) {
            // Spawn process with the buffered input
            let mut child = Command::new(&command)
                .args(command_args)
                .stdin(Stdio::piped())
                .spawn()?;

            // Write buffered content to child process
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                if !current_buffer.is_empty() {
                    stdin.write_all(&current_buffer)?;
                }
            }

            // Wait for process to complete
            let status = child.wait()?;
            if !status.success() {
                eprintln!("Child process failed with status: {}", status);
            }

            // Clear buffer and reset timer
            current_buffer.clear();
            last_execution = Instant::now();
        }
    }

    Ok(())
}