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
    Remove {
        #[arg(value_hint=clap::ValueHint::Other)]
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
    let connection = db::connect_to_db();
    let _ = db::create_table(&connection);

    match &cli.command {
        Commands::List {
            section,
            sort_by_section,
        } => {
            let mut cmd = Cli::command();
            let books = match db::get_books(&connection) {
                Ok(bks) => bks,
                Err(e) => match e {
                    db::GetBooksError::NoBooks => {
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
            if !help::is_pdf(path) {
                cmd.error(clap::error::ErrorKind::InvalidValue, "The file is not PDF!")
                    .exit();
            }
            let (is_correct, good_path) = help::is_correct_path(path);
            if !is_correct {
                cmd.error(clap::error::ErrorKind::InvalidValue, "Invalid path!")
                    .exit();
            }
            let bk = book::Book::init(
                name.clone(),
                good_path.unwrap().to_str().unwrap().to_string(),
                section.clone(),
            );
            match db::create_book(&connection, &bk) {
                Ok(_) => println!("The book has been created!"),
                Err(err) => match err {
                    db::CreateBookError::BookWithNameExists => {
                        println!("The book with this name already exists!")
                    }
                    _ => println!("An error occured while creating the book!"),
                },
            }
        }
        Commands::Remove { name } => {
            let mut cmd = Cli::command();
            match db::remove_book(&connection, name) {
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
            let bk_res = db::get_book(&connection, name);
            if let Ok(bk) = bk_res {
                let path = bk.path;
                process::Command::new("open")
                    .args(["-a", "Skim", path.as_str()])
                    .output()
                    .expect("error while opening the file with Skim");
            } else {
                let mut cmd = Cli::command();
                cmd.error(
                    ErrorKind::InvalidValue,
                    "the book with this name doesn't exist",
                )
                .exit();
            }
        }
    }
}
