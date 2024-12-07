/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
use basm::Error::*;
use basm::*;
use colored::*;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    if CONFIG.debug {
        println!("Main func started.");
    }

    let input: &String = &CONFIG.file;
    let file = Path::new(input);

    if input.is_empty() {
        LineLessError("no input files").perror();
        process::exit(1);
    }
    if File::open(file).is_err() {
        LineLessError(format!("file {} does not exist", input).as_str()).perror();
        process::exit(1);
    }
    if let Ok(metadata) = fs::metadata(input) {
        if metadata.is_dir() {
            LineLessError(format!("{} is a directory", input).as_str()).perror();
            process::exit(1);
        }
    }

    if CONFIG.debug {
        println!("File is Some");
    }

    let lines = process_includes(input)?;

    let lines: Vec<String> = lines.iter().map(|line| line.trim().to_string()).collect();

    if CONFIG.verbose || CONFIG.debug {
        println!("{}", "Processing lines:".blue());
        for (index, line) in lines.iter().enumerate() {
            println!("{}: {}", index + 1, line.green());
        }
    }

    let mut encoded_instructions = Vec::new();
    let mut line_count: u32 = 1;
    let mut write_to_file: bool = true;
    let mut has_err: bool = false;
    process_start(&lines);
    load_subroutines(&lines);

    for line in lines {
        let mut lexer = Lexer::new(&line, line_count);
        let tokens = lexer.lex();
        if tokens.is_empty() {
            line_count += 1;
            continue;
        }

        let instruction = tokens.first();
        let operand1 = tokens.get(1);
        let operand2 = {
            if let Some(Token::Comma) = tokens.get(2) {
                tokens.get(3)
            } else {
                tokens.get(2)
            }
        };

        if CONFIG.debug {
            println!("Raw line: {}", line.green());
        }

        if CONFIG.debug {
            for token in tokens {
                println!(
                    "{} {}",
                    "Token:".green().bold(),
                    token.to_string().blue().bold()
                );
            }
            println!();
        }

        if let Some(ins) = instruction {
            let encoded_instruction = encode_instruction(ins, operand1, operand2, line_count);
            if encoded_instruction.is_none() {
                continue;
            }
            if verify(ins, operand1, operand2, line_count) {
                write_to_file = false;
                has_err = true;
            }
            encoded_instructions.extend(&encoded_instruction.unwrap().to_be_bytes());
            if CONFIG.verbose || CONFIG.debug {
                println!("Instruction: {:016b}", encoded_instruction.unwrap());
            }
        } else {
            OtherError(
                format!("not enough lines to encode instruction {}", line).as_str(),
                line_count,
                None,
            )
            .perror();
            process::exit(1);
        }

        line_count += 1;
    }

    if has_err {
        eprintln!("{}", "Exiting...".red());
        process::exit(1);
    }

    if CONFIG.debug {
        print_subroutine_map();
    }

    match &CONFIG.output {
        Some(output_file) if write_to_file => {
            write_encoded_instructions_to_file(output_file, &encoded_instructions)?;
        }
        _ => eprintln!("Did not write to output file"),
    }

    Ok(())
}

fn process_includes(input: &String) -> io::Result<Vec<String>> {
    let include_regex = Regex::new(r#"^\s*#include\s+"([^"]+)""#).unwrap();
    let mut included_lines = Vec::new();
    let file = File::open(input)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let content = match line {
            Ok(content) => content,
            Err(e) => {
                LineLessError(format!("error while reading from file: {}", e).as_str()).perror();
                process::exit(1);
            }
        };

        if content.trim().starts_with("#include") {
            if let Some(captures) = include_regex.captures(content.trim()) {
                let include_file = captures[1].to_string();
                if let Ok(included) = read_include_file(&include_file) {
                    included_lines.extend(included);
                } else {
                    LineLessError(
                        format!("could not read included file: {}", include_file).as_str(),
                    )
                    .perror();
                    process::exit(1);
                }
            }
            continue;
        }

        included_lines.push(content);
    }

    Ok(included_lines)
}

fn read_include_file(file_name: &str) -> io::Result<Vec<String>> {
    let mut included_lines = Vec::new();
    let reader = io::BufReader::new(File::open(file_name)?);

    for line in reader.lines() {
        match line {
            Ok(content) => included_lines.push(content),
            Err(e) => {
                LineLessError(format!("error while reading from include file: {}", e).as_str())
                    .perror()
            }
        }
    }
    Ok(included_lines)
}

fn write_encoded_instructions_to_file(
    filename: &str,
    encoded_instructions: &[u8],
) -> io::Result<()> {
    if CONFIG.debug || CONFIG.verbose {
        println!("{}", "Wrote to file.".green());
    }
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // no tests
}
