use crate::book;
use rusqlite::{params, Connection, Result};

pub enum CreateBookError {
    BookWithNameExists,
    Other,
}

pub struct DbConfig {
    pub path_to_db: String,
}

impl Default for DbConfig {
    fn default() -> DbConfig {
        DbConfig {
            path_to_db: "/Users/dobbikov/.config/book-cli/books.db".to_string(),
        }
    }
}

pub fn connect_to_db() -> Connection {
    let config: DbConfig = Default::default();
    let db = Connection::open(config.path_to_db);
    match db {
        Ok(conn) => conn,
        Err(mess) => panic!("An error occured! {}", mess),
    }
}

pub fn create_table(conn: &Connection) -> Result<bool, bool> {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS books(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            section TEXT
            )",
        (),
    ) {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

pub fn create_book(conn: &Connection, bk: &book::Book) -> Result<bool, CreateBookError> {
    let bk_res = get_book(conn, &bk.name);
    if bk_res.is_ok() {
        return Err(CreateBookError::BookWithNameExists);
    }
    match conn.execute(
        "INSERT INTO books (name, path, section) VALUES (?, ?, ?)",
        params![bk.name, bk.path, bk.section],
    ) {
        Ok(_) => Ok(true),
        Err(_) => Err(CreateBookError::Other),
    }
}

pub enum RemoveBookError {
    BookDoesNotExist,
    Other,
}

impl std::fmt::Display for RemoveBookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveBookError::BookDoesNotExist => write!(f, "this book doesn't exist"),
            RemoveBookError::Other => write!(f, "unexpected error in removing book"),
        }
    }
}

fn remove_book_from_db(conn: &Connection, name: &String) -> Result<usize> {
    conn.execute("DELETE FROM books WHERE name = ?", params![name])
}
pub fn remove_book(conn: &Connection, name: &String) -> Result<book::Book, RemoveBookError> {
    let bk_res = get_book(conn, name);
    if let Ok(bk) = bk_res {
        let del_res = remove_book_from_db(conn, name);
        if del_res.is_ok() {
            Ok(bk)
        } else {
            Err(RemoveBookError::Other)
        }
    } else {
        Err(RemoveBookError::BookDoesNotExist)
    }
}

pub enum GetBookError {
    QueryError(rusqlite::Error),
    EmptyList,
    NoneElement,
    TableOrBookDoesnotExist,
}

pub fn get_book(conn: &Connection, name: &String) -> Result<book::Book, GetBookError> {
    let stmt = conn.prepare(format!("SELECT * FROM books WHERE name = '{0}'", name).as_str());
    if let Ok(mut stmt_res) = stmt {
        match stmt_res.query_map([], |row| {
            let _: u32 = row.get(0)?;
            Ok(book::Book {
                name: row.get(1)?,
                path: row.get(2)?,
                section: row.get(3)?,
            })
        }) {
            Ok(mut book_iter) => {
                if let Some(bk_) = book_iter.next() {
                    if let Ok(bk) = bk_ {
                        Ok(bk)
                    } else {
                        Err(GetBookError::NoneElement)
                    }
                } else {
                    Err(GetBookError::EmptyList)
                }
            }
            Err(e) => Err(GetBookError::QueryError(e)),
        }
    } else {
        Err(GetBookError::TableOrBookDoesnotExist)
    }
}

pub enum GetBooksError {
    BookOrTableDoesnotExist,
    NoBooks,
}

impl std::fmt::Display for GetBooksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetBooksError::BookOrTableDoesnotExist => {
                write!(f, "Table doesn't exist or there's no books")
            }
            GetBooksError::NoBooks => write!(f, "There's no books!"),
        }
    }
}

pub fn get_books(conn: &Connection) -> Result<Vec<book::Book>, GetBooksError> {
    let stmt = conn.prepare("SELECT * FROM books");
    if let Ok(mut stmt_res) = stmt {
        match stmt_res.query_map([], |row| {
            let _: u32 = row.get(0)?;
            Ok(book::Book {
                name: row.get(1)?,
                path: row.get(2)?,
                section: row.get(3)?,
            })
        }) {
            Ok(book_iter) => {
                let mut res: Vec<book::Book> = Vec::new();
                for bk_ in book_iter.flatten() {
                    res.push(bk_);
                }
                Ok(res)
            }
            Err(_) => Err(GetBooksError::NoBooks),
        }
    } else {
        Err(GetBooksError::BookOrTableDoesnotExist)
    }
}
