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
    /*
    if CONFIG.cow {
        println!(" _______________________ ");
        println!("< Install Gentoo Linux! >");
        println!(" ----------------------- ");
        println!("        \\  ^__^         ");
        println!("         \\ (oo)\\_______   ");
        println!("           (__)\\       )\\/\\");
        println!("               ||----w |    ");
        println!("               ||     ||    ");
        process::exit(0);
    }
    */
    if CONFIG.debug {
        println!("Main func started."); // bc yk the main function sometimes doesn't start
    }
    let mut lines: Vec<String> = Vec::new();
    let mut has_err: bool = false;
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
        if fs::read_to_string(Path::new(input))?.is_empty() {
            LineLessError(format!("file {} is empty", input).as_str()).perror();
            process::exit(1);
        }
        let file = File::open(Path::new(input))?;
        let reader = io::BufReader::new(file);
        let include_regex = Regex::new(r#"^\s*#include\s+"([^"]+)""#).unwrap();

        for line in reader.lines() {
            match line {
                Ok(content) => {
                    if content.trim().starts_with("#include") {
                        if let Some(captures) = include_regex.captures(content.trim()) {
                            let include_file = captures[1].to_string();
                            if let Ok(included_lines) = read_include_file(&include_file) {
                                lines.extend(included_lines);
                            } else {
                                LineLessError(
                                    format!("could not read included file: {}", include_file)
                                        .as_str(),
                                )
                                .perror();
                                process::exit(1);
                            }
                        }
                    } else {
                        lines.push(content);
                    }
                }
                Err(e) => {
                    LineLessError(format!("error while reading from file: {}", e).as_str())
                        .perror();

                    has_err = true;
                }
            }
    }

    // Clean up lines
    for line in &mut lines {
        *line = line.trim().to_string();
    }

    if CONFIG.verbose | CONFIG.debug {
        println!("{}", "Processing lines:".blue());
        for (index, line) in lines.iter().enumerate() {
            println!("{}: {}", index + 1, line.green());
        }
    }
    let mut encoded_instructions = Vec::new();
    let mut line_count: u32 = 1; // bigger numbers with 32
    let mut write_to_file: bool = true; // defines if we should write to file (duh)
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
            for token in tokens.iter() {
                println!(
                    "{} {}",
                    "Token:".green().bold(),
                    token.to_string().blue().bold()
                );
            }
        }
        if let Some(ins) = instruction {
            let encoded_instruction = encode_instruction(ins, operand1, operand2, line_count);
            if verify(ins, operand1, operand2, line_count) {
                write_to_file = false;
                has_err = true;
            }
            encoded_instructions.extend(&encoded_instruction.to_be_bytes());
            if CONFIG.verbose || CONFIG.debug {
                println!("Instruction: {:016b}", encoded_instruction);
            }
            let ins_str: String = format!("{:016b}", encoded_instruction);
            if CONFIG.debug {
                if let Some(ins) = ins_str.get(0..4) {
                    // fixed length instructions my beloved
                    println!("INS: {}", ins.blue().bold());
                }
                if let Some(dst) = ins_str.get(4..7) {
                    println!("DST: {}", dst.blue().bold());
                }
                if let Some(dtb) = ins_str.get(7..8) {
                    println!("DTB: {}", dtb.blue().bold());
                }
                if let Some(src) = ins_str.get(8..16) {
                    println!("SRC: {}\n", src.blue().bold());
                }
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

        line_count += 1; // line count exists so we can have line number errors
    }
    if has_err {
        eprintln!("{}", "Exiting...".red());
        process::exit(1); // wowzers, amazing
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
fn read_include_file(file_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    // pretty obvious
    if CONFIG.debug | CONFIG.verbose {
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
