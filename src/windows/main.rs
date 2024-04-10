use std::{env::args, io::stdin, process::exit};

use chrono::Local;
use figlet_rs::FIGfont;

const TITLE: &str = "Rusty Cleaner";
const VERSION: &str = "v0.0.1";
const AVAILABLE_OPTIONS: [&str; 2] = ["🗑️  --clean", "😿  --exit"];

struct RustyCleaner<'a> {
    title: &'a str,
    version: &'a str,
}

mod cleaner;
use cleaner::{clear_chromium_web_browsers, clear_discord, clear_firefox, clear_temporary_files};

impl<'a> RustyCleaner<'a> {
    fn show_info(&self) -> Self {
        let logo: String = FIGfont::standard()
            .unwrap()
            .convert(self.title)
            .unwrap()
            .to_string();

        println!("{}", logo);
        println!("\n✨ The lightweight and powerful CLI tool to clean your system of unnecessary files. (Written in Rust 🦀)");
        println!("📦 Version: {} ✅\n", self.version);
        println!("🏗️ DevCheckOG");
        println!("🐈 https://github.com/DevCheckOG");
        println!("🕒 {}\n", Local::now().format("%H:%M:%S %d/%m/%Y"));

        Self {
            title: self.title,
            version: self.version,
        }
    }

    fn show_options(&self) -> Self {
        println!("🔎 Options:\n");

        AVAILABLE_OPTIONS.map(|option| println!("{}", option));

        println!();

        loop {
            let mut select_option: String = String::new();

            if stdin().read_line(&mut select_option).is_ok() {
                select_option = select_option.trim().to_string();

                if select_option == "--clean" {
                    self.clean();
                    continue;
                } else if select_option == "--exit" {
                    println!("\n😿 Exiting...");
                    exit(0);
                }
            }

            select_option.clear();
        }
    }

    fn clean(&self) {
        println!("\n🗑️ Cleaning...");
        clear_discord();
        clear_temporary_files();
        clear_firefox();
        clear_chromium_web_browsers();
        println!("✅ Cleaned!\n");
    }
}

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
fn main() {
    let args: Vec<String> = args().collect::<Vec<_>>();

    if args[args.len() - 1] != "--clean" {
        RustyCleaner {
            title: TITLE,
            version: VERSION,
        }
        .show_info()
        .show_options();

        return;
    }

    RustyCleaner {
        title: TITLE,
        version: VERSION,
    }
    .clean();
}
