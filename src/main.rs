// Copyright 2025 Colton Loftus
// SPDX-License-Identifier: AGPL-3.0-or-later

use toggle_by_mic_status::{microphone_is_in_use, notify_and_print};

use clap::Parser;
use mac_notification_sys::set_application;

/// Sßtart and restart a program based on the state of the default microphone
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Automatically respawn the program when the mic is not in use.
    /// This may need to be disabled if the program needs to be launched manually
    /// Default: true
    #[arg(short, long, default_value_t = true)]
    auto_spawn: bool,

    /// The absolute path of the program to run
    #[arg(short, long, default_value_t = String::from("/Applications/BreakTimer.app/Contents/MacOS/BreakTimer"))]
    program_name: String,
}

fn spawn_program(program_name: &str) {
    if let Err(e) = std::process::Command::new(program_name).spawn() {
        eprintln!("Failed to execute {}: {}", program_name, e);
        std::process::exit(1)
    }
}

// Kill a program by name
fn kill_program(program_path: &str) {
    let program_name = program_path.split('/').last();

    match program_name {
        None => {
            eprintln!(
                "Failed to get program name from absolute path: {}",
                program_path
            );
            std::process::exit(1)
        }
        Some(program_name) => {
            match std::process::Command::new("pkill")
                .arg("-x")
                .arg(program_name)
                .spawn()
            {
                Err(e) => {
                    eprintln!("Failed to execute pkill: {}", e);
                    std::process::exit(1)
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    // have to set this to get notifications
    set_application("com.apple.Terminal").unwrap();

    println!("Watching default microphone... ");

    let mut mic_in_use = false;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        match microphone_is_in_use() {
            Ok(is_in_use) => {
                if !mic_in_use && is_in_use {
                    notify_and_print(
                        format!("Microphone is now in use! Killing {}", args.program_name).as_str(),
                    );
                    mic_in_use = true;
                    kill_program(&args.program_name);
                } else if mic_in_use && !is_in_use {
                    println!("Microphone is now not in use!");
                    if args.auto_spawn {
                        notify_and_print(format!("Respawning {}", args.program_name).as_str());
                        spawn_program(&args.program_name);
                    }
                    mic_in_use = false;
                }
            }
            Err(e) => {
                println!("Error checking microphone: {}", e);
            }
        }
    }
}
