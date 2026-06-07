#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u32,
    read: bool,
}

impl Book {
    // Associated function — called as Book::new(...)
    fn new(title: &str, author: &str, year: u32) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            read: false,
        }
    }

    // &self — borrows immutably, just reads
    fn summary(&self) -> String {
        format!("\"{}\" by {} ({})", self.title, self.author, self.year)
    }

    // &mut self — borrows mutably, changes state
    fn mark_read(&mut self) {
        self.read = true;
    }

    fn is_read(&self) -> bool {
        self.read
    }
}

#[derive(Debug)]
struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    fn new(name: &str) -> Self {
        Library {
            name: name.to_string(),
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn unread_count(&self) -> usize {
        self.books.iter().filter(|b| !b.is_read()).count()
    }

    fn list_unread(&self) {
        println!("Unread books in {}:", self.name);
        for book in self.books.iter().filter(|b| !b.is_read()) {
            println!("  - {}", book.summary());
        }
    }

    // Returns a reference — ownership stays in Library
    fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }
}

fn main() {
    let mut library = Library::new("My Shelf");

    library.add_book(Book::new("The Rust Programming Language", "Steve Klabnik", 2019));
    library.add_book(Book::new("Programming Rust", "Jim Blandy", 2021));
    library.add_book(Book::new("Rustonomicon", "Various", 2023));

    // Mark one as read
    library.books[0].mark_read();

    library.list_unread();
    println!("\nUnread count: {}", library.unread_count());

    let results = library.find_by_author("Jim Blandy");
    println!("\nBooks by Jim Blandy:");
    for book in results {
        println!("  - {}", book.summary());
    }
}
