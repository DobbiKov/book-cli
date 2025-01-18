use std::cmp::Ordering;

#[derive(Clone)]
pub struct Book {
    pub path: String,
    pub name: String,
    pub section: Option<String>,
}

impl Book {
    pub fn init(name: String, path: String, section: Option<String>) -> Book {
        Book {
            path,
            name,
            section,
        }
    }
}

fn extract_file_name_from_path(path: &str) -> String {
    if !path.contains('/') {
        path.to_string()
    } else {
        path.split("/").last().unwrap().to_string()
    }
}

fn print_book(bk: Book) {
    let res = format!(
        "{0}[{1}]: {2}",
        bk.name,
        if let Some(sec) = bk.section {
            sec
        } else {
            "".to_string()
        },
        extract_file_name_from_path(&bk.path)
    );
    print!("{}", res);
}
pub fn print_books(books: Vec<Book>, indent: u16) {
    for bk in books {
        (0..indent).for_each(|_i| {
            print!(" ");
        });
        print_book(bk);
        println!();
    }
}

pub fn sort_books_by_section(books: Vec<Book>) -> Vec<(String, Vec<Book>)> {
    let mut bks = books;
    let mut res: Vec<(String, Vec<Book>)> = Vec::new();
    fn compare(bk1: &Book, bk2: &Book) -> Ordering {
        bk1.section.cmp(&bk2.section)
    }
    bks.sort_by(compare);
    let mut temp_res: Vec<Book> = Vec::new();
    let mut curr_section: String = "".to_string();

    for bk in bks {
        let mut sec: String = "".to_string();
        if let Some(_sec) = &bk.section {
            sec = _sec.clone();
        }

        if sec != curr_section {
            if !temp_res.is_empty() {
                res.push((curr_section, temp_res));
                temp_res = Vec::new();
            }
            curr_section = sec
        }
        temp_res.push(bk);
    }
    res.push((curr_section, temp_res));
    res
}
