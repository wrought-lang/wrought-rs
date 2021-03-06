
use std::path::PathBuf;

use clap::Clap;
use codespan_reporting::{
    files::SimpleFile,
    diagnostic::{
        Diagnostic,
        Label
    },
};

pub mod ast;
pub mod lexer;
pub mod parser;

use lexer::tokenize;

#[derive(Clap, Debug)]
struct Arguments {
    #[clap(subcommand)]
    command: Command
}

#[derive(Clap, Debug)]
enum Command {
    Compile(Compile),
    CheckLex(CheckLex)
}

#[derive(Clap, Debug)]
struct Compile {
    #[clap(long)]
    input_path: PathBuf

}

#[derive(Clap, Debug)]
struct CheckLex {
    #[clap(long)]
    input_path: PathBuf

}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Compile(_) => {
            println!("Not yet implemented!");
        },
        Command::CheckLex(checklex_command) => {
            if let Some(_) = check_lex(checklex_command) {
                println!("Finished");
            } else {
                println!("Ended before finishing");
            }
        }
    }
}

fn check_lex(args: CheckLex) -> Option<()> {
    let file_name = args.input_path.file_name()?.to_string_lossy().to_string();
    let file_string = std::fs::read_to_string(args.input_path).ok()?;

    let simple_file = SimpleFile::new(file_name, file_string);

    match tokenize(simple_file) {
        Ok(token_data) => {
            for token_m in token_data.tokens {
                let diagnostic = Diagnostic::help()
                    .with_message(format!("Interpretted as {:?}", token_m.value))
                    .with_labels(vec![
                        Label::primary((), token_m.span).with_message("matched input")
                    ]);
                print_diagnostic(&token_data.file, diagnostic);
            }
        },
        Err(token_errors) => {
            for error_span in token_errors.errors {
                let diagnostic = Diagnostic::error()
                    .with_message("Could not parse token")
                    .with_labels(vec![
                        Label::primary((), error_span).with_message("lexing error")
                    ]);
                print_diagnostic(&token_errors.file, diagnostic);
            }
        }
    }

    Some(())
}

fn print_diagnostic(file: &SimpleFile<String, String>, diagnostic: Diagnostic<()>) {
    use codespan_reporting::term;
    use codespan_reporting::term::termcolor::{ ColorChoice, StandardStream };

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    let _ = term::emit(&mut writer.lock(), &config, file, &diagnostic);
}

