use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub available: bool,
}

pub struct Library {
    pub books: HashMap<u32, Book>,
    pub borrowed_books: HashMap<u32, Book>, // Tracks borrowed books
    pub next_id: u32,
}

impl Library {
    // Initializes the Library with pre-defined books
    pub fn new() -> Library {
        let mut library = Library {
            books: HashMap::new(),
            borrowed_books: HashMap::new(),
            next_id: 1,
        };

        // Pre-defined books
        library.add_book("The Rust Programming Language", "Steve Klabnik and Carol Nichols");
        library.add_book("Clean Code", "Robert C. Martin");
        library.add_book("The Pragmatic Programmer", "Andrew Hunt and David Thomas");
        library.add_book("Design Patterns", "Erich Gamma, Richard Helm, Ralph Johnson, John Vlissides");

        library
    }

    // Adds a new book to the library
    pub fn add_book(&mut self, title: &str, author: &str) {
        let book = Book {
            id: self.next_id,
            title: title.to_string(),
            author: author.to_string(),
            available: true,
        };
        self.books.insert(self.next_id, book);
        self.next_id += 1;
    }

    // Lists all available books in the library
    pub fn list_books(&self) {
        if self.books.is_empty() {
            println!("No books available in the library.");
        } else {
            println!("\nAvailable Books:");
            for book in self.books.values() {
                if book.available {
                    println!("ID: {} \nTitle: {} \nAuthor: {}\n----------------------\n", book.id, book.title, book.author);
                }
            }
        }
    }

    // Lists all borrowed books
    pub fn list_borrowed_books(&self) {
        if self.borrowed_books.is_empty() {
            println!("No books have been borrowed.");
        } else {
            println!("\nBorrowed Books:");
            for book in self.borrowed_books.values() {
                println!("ID: {}, Title: {}, Author: {}", book.id, book.title, book.author);
            }
        }
    }

    // Allows borrowing a book by its ID
    pub fn borrow_book(&mut self, id: u32) {
        if let Some(book) = self.books.get_mut(&id) {
            if book.available {
                book.available = false;
                self.borrowed_books.insert(id, book.clone()); 
                println!("You have successfully borrowed '{}'.", book.title);
            } else {
                println!("The book '{}' is currently not available.", book.title);
            }
        } else {
            println!("Book with ID {} does not exist.", id);
        }
    }

    // Allows returning a borrowed book
    pub fn return_book(&mut self, id: u32) {
        if let Some(book) = self.books.get_mut(&id) {
            if !book.available {
                book.available = true;
                self.borrowed_books.remove(&id); 
                println!("You have successfully returned '{}'.", book.title);
            } else {
                println!("The book '{}' was not borrowed.", book.title);
            }
        } else {
            println!("Book with ID {} does not exist.", id);
        }
    }
}
