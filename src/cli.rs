use crate::backend::{
    build_livestream, bulletin_categorizer, get_config, get_help, save_obs_file, wrap_line,
};
use figlet_rs::FIGlet;
use owo_colors::OwoColorize;
// Yellow bold w \t for main messages
// Blue for choices
// Red bold for errors or unexpected occuences
use std::fs::read_to_string;
use std::io;
pub fn cli() {
    let font = FIGlet::standard().unwrap();
    println!(
        "{}{}{}{}",
        font.convert("OBS").unwrap().yellow().bold(),
        font.convert("Church").unwrap().yellow().bold(),
        font.convert("Automation").unwrap().yellow().bold(),
        font.convert(": ]").unwrap().yellow().bold()
    );
    'big_loop: loop {
        println!(
            "{}\n{}\n{}\n{}",
            "\tWould you like to:".yellow().bold(),
            "(1) Generate a json based on a txt file".blue(),
            "(2) Exit the program".blue(),
            "(3) Open documentation".blue()
        );
        let input = grab_input(Some(vec!["1", "2", "3"]));
        match input.as_str() {
            "1" => {
                let txt = loop {
                    println!("{}", "\tPlease enter the path to the txt".yellow().bold());
                    let path = grab_input(None);
                    if !path.ends_with(".txt") {
                        println!("{}", "Must be a txt file".red().bold());
                        continue;
                    }
                    let txt = read_to_string(&path);
                    let Ok(s) = txt else {
                        println!("{}{}", "Err: ".red().bold(), txt.unwrap_err().red().bold());
                        continue;
                    };
                    break s.trim().to_string();
                };
                let mut line: usize = 0;
                let mut sifted = bulletin_categorizer(
                    txt.lines().map(|x| x.to_string()).collect(),
                    get_config(),
                );
                loop {
                    println!(
                        "{}{}",
                        "\tActions:\n".yellow().bold(),
                        "(0) = unidentified\n(1) = credits\n(2) = regular text\n(3) = hymn\n(4) = P: C:\n(5) = insert empty scene\n(6) = service name\n(7) = N/A\n(8) = special music\n(9) = with previous\n(w) = write and exit\n() = do nothing\n(b) = go back".blue()
                    );
                    println!(
                        "{}{}",
                        sifted[line].0.bright_green(),
                        "\tLine:".yellow().bold()
                    );
                    println!("{}", wrap_line(&sifted[line].1, 75).green());
                    let action = grab_input(Some(vec![
                        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "w", "", "b",
                    ]));
                    if action.as_str() == "w" {
                        break;
                    } else if action.is_empty() {
                        if line + 1 < sifted.len() {
                            line += 1;
                        }
                        continue;
                    } else if action.as_str() == "b" {
                        if line != 0 {
                            line -= 1;
                        }
                        continue;
                    }
                    sifted[line].0 = action.parse().unwrap_or(0);
                }
                save_obs_file(build_livestream(sifted));
            }
            "2" => {
                break 'big_loop;
            }
            "3" => {
                get_help();
            }
            _ => unreachable!(),
        }
    }
}
fn grab_input(allowable_cases: Option<Vec<&str>>) -> String {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(c) = &allowable_cases {
                    if !c.contains(&&input.trim().to_lowercase().as_str()) {
                        println!("{}", "Please provide a valid input".red().bold());
                        continue;
                    }
                }
            }
            Err(e) => {
                println!(
                    "{}{e}{}",
                    "Encountered error: ".red().bold(),
                    "\nPlease try again".red().bold()
                );
                continue;
            }
        }
        return input.trim().to_string();
    }
}
