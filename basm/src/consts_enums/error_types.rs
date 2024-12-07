use crate::CONFIG;
use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[derive(Debug)]
// error enum type, will implement once I feel productive
pub enum Error<'a> {
    InvalidSyntax(&'a str, u32, Option<u32>),
    ExpectedArgument(&'a str, u32, Option<u32>),
    NonexistentData(&'a str, u32, Option<u32>),
    UnknownCharacter(&'a str, u32, Option<u32>),
    OtherError(&'a str, u32, Option<u32>),
    LineLessError(&'a str),
}
impl Error<'_> {
    pub fn perror(&self) {
        let exe_path = std::env::args().next().unwrap_or_default();
        let binary_name = Path::new(&exe_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let line_number = match self {
            Error::InvalidSyntax(_, n, _) => *n,
            Error::ExpectedArgument(_, n, _) => *n,
            Error::NonexistentData(_, n, _) => *n,
            Error::UnknownCharacter(_, n, _) => *n,
            Error::OtherError(_, n, _) => *n,
            Error::LineLessError(_) => {
                return eprintln!(
                    "{} {}\n{}",
                    binary_name.blue(),
                    "error: ".red().bold(),
                    self.message()
                )
            }
        };

        let error_message = match self {
            Error::InvalidSyntax(s, _, _) => format!("invalid syntax: \n{s}"),
            Error::ExpectedArgument(s, _, _) => format!("expected an argument: \n{s}"),
            Error::NonexistentData(s, _, _) => format!("nonexistent data: \n{s}"),
            Error::UnknownCharacter(s, _, _) => format!("has unknown character: \n{s}"),
            Error::OtherError(s, _, _) => (*s).to_string(),
            _ => unreachable!(),
        };
        let location = match self {
            Error::InvalidSyntax(_, _, n) => n,
            Error::ExpectedArgument(_, _, n) => n,
            Error::NonexistentData(_, _, n) => n,
            Error::UnknownCharacter(_, _, n) => n,
            Error::OtherError(_, _, n) => n,
            _ => unreachable!(),
        };
        eprintln!(
            "{} {}on line {}: {}",
            binary_name.bright_white(),
            "error ".red().bold(),
            line_number.to_string().as_str().green(),
            error_message
        );
        let input: &String = &CONFIG.file;
        let path = Path::new(input);
        for (current_line, line) in io::BufReader::new(File::open(path).unwrap())
            .lines()
            .enumerate()
        {
            if current_line + 1 == line_number.try_into().unwrap() {
                println!("{}", line.unwrap().trim().bright_white());
            }
        }
        if location.is_some() {
            for _ in 0..location.unwrap() {
                print!(" ");
            }
            println!("{}", "^".red().bold());
        }
    }

    fn message(&self) -> &str {
        match self {
            Error::LineLessError(s) => s,
            _ => "",
        }
    }
}
