use book_lib::{book, db, help};
use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "Book CLI")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long)]
        section: Option<String>,
        #[arg(long, action)]
        sort_by_section: bool,
    },
    Add {
        #[arg(value_hint=clap::ValueHint::Other)]
        name: String,
        #[arg(value_hint=clap::ValueHint::FilePath)]
        path: String,
        #[arg(value_hint=clap::ValueHint::Other)]
        section: Option<String>,
    },
    #[command(trailing_var_arg = true)]
    Remove {
        #[arg(value_hint = clap::ValueHint::Other, trailing_var_arg = true)]
        #[arg(required = true)]
        #[arg(num_args = 1..)]
        name: String,
    },
    Open {
        #[arg(value_hint=clap::ValueHint::Other)]
        name: String,
    },
}

fn print_by_section(books_by_sec: Vec<(String, Vec<book::Book>)>) {
    for el in books_by_sec {
        println!("Section: [{0}]", el.0);
        book::print_books(el.1, 4);
        println!();
    }
}
fn main() {
    let cli = Cli::parse();
    let connection = book_lib::db::setup();

    match &cli.command {
        Commands::List {
            section,
            sort_by_section,
        } => {
            let mut cmd = Cli::command();
            let books = match book_lib::get_books(&connection) {
                Ok(bks) => bks,
                Err(e) => match e {
                    book_lib::GetBooksError::NoBooks => {
                        cmd.error(clap::error::ErrorKind::InvalidValue, "There's no books!")
                            .exit();
                    }
                    err => {
                        cmd.error(
                            clap::error::ErrorKind::InvalidValue,
                            format!("Couldn't procceed due to this error: {0}", err),
                        )
                        .exit();
                    }
                },
            };
            if let Some(section) = section {
                let sorted_by_section = help::get_books_with_section(books, section);
                println!("Section: [{0}]", section);
                book::print_books(sorted_by_section, 4);
            } else if *sort_by_section {
                let sorted_by_section = book::sort_books_by_section(books);
                print_by_section(sorted_by_section);
            } else {
                book::print_books(books, 0);
            }
        }
        Commands::Add {
            name,
            path,
            section,
        } => {
            let mut cmd = Cli::command();
            let bk = book::Book::init(name.clone(), path.to_string(), section.clone(), false);
            match book_lib::create_book(&connection, &bk) {
                Ok(_) => println!("The book has been created!"),
                Err(err) => {
                    cmd.error(clap::error::ErrorKind::InvalidValue, err).exit();
                }
            }
        }
        Commands::Remove { name } => {
            let mut cmd = Cli::command();
            match book_lib::remove_book(&connection, name) {
                Ok(_) => println!("The book has been removed successfully!"),
                Err(err) => {
                    let err_mess = format!(
                        "The book couldn't have been removed due to this error: {0}",
                        err
                    );
                    cmd.error(clap::error::ErrorKind::InvalidValue, err_mess.as_str())
                        .exit();
                }
            }
        }
        Commands::Open { name } => {
            let mut cmd = Cli::command();
            match book_lib::open_book(&connection, name) {
                Ok(_) => {}
                Err(err) => {
                    cmd.error(ErrorKind::InvalidValue, err).exit();
                }
            }
        }
    }
}
