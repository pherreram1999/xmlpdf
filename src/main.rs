use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::exit;

mod haru;
mod layout;
mod xml;

use layout::LayoutEngine;
use xml::parse_xml_pdf;

#[derive(Parser, Debug)]
#[command(
    name = "pdfparser",
    author = "Antigravity",
    version = "0.1.0",
    about = "High-performance XML to PDF generator powered by Rust & Libharu"
)]
struct CliArgs {
    /// Input XML file path. If omitted, reads from standard input (stdin).
    #[arg(short = 'i', long = "input")]
    input: Option<PathBuf>,

    /// Output PDF file path. If omitted, writes binary PDF stream to standard output (stdout).
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

fn main() {
    let args = CliArgs::parse();

    // 1. Read XML input string from stdin or file
    let xml_content = match read_input(&args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("[Error] Reading input failed: {}", e);
            exit(1);
        }
    };

    // 2. Parse XML into PDF AST
    let pdf_ast = match parse_xml_pdf(&xml_content) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("[Error] XML parsing failed: {}", e);
            exit(1);
        }
    };

    // 3. Render AST using Libharu Layout Engine
    let engine = match LayoutEngine::new(pdf_ast) {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("[Error] Layout engine initialization failed: {}", e);
            exit(1);
        }
    };

    let pdf_bytes = match engine.render() {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("[Error] PDF rendering failed: {}", e);
            exit(1);
        }
    };

    // 4. Output PDF binary bytes to stdout or file
    if let Err(e) = write_output(&args.output, &pdf_bytes) {
        eprintln!("[Error] Writing PDF output failed: {}", e);
        exit(1);
    }
}

fn read_input(input_path: &Option<PathBuf>) -> io::Result<String> {
    let mut buffer = String::new();
    if let Some(path) = input_path {
        let mut file = File::open(path)?;
        file.read_to_string(&mut buffer)?;
    } else {
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}

fn write_output(output_path: &Option<PathBuf>, pdf_bytes: &[u8]) -> io::Result<()> {
    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        file.write_all(pdf_bytes)?;
    } else {
        let mut stdout = io::stdout().lock();
        stdout.write_all(pdf_bytes)?;
        stdout.flush()?;
    }
    Ok(())
}
