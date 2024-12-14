# libanubhav - Library Management System

**libanubhav** is a simple Rust library for managing a collection of books. It provides functionality to add, borrow, return, and list books. The library is designed to be easily integrated into your Rust projects.

## Features

- Add new books to the library
- List available books
- List borrowed books
- Borrow books by ID
- Return borrowed books by ID

## Installation

To use **libanubhav** in your Rust project, follow these steps:

### Step 1: Add `libanubhav` to your project

1. If you don't have a `Cargo.toml` file in your project, create one by initializing your project with Cargo:
   ```bash
   cargo init
   ```

2. Open the `Cargo.toml` file in your project and add `libanubhav` as a dependency:
   ```toml
   [dependencies]
   libanubhav = { path = "path/to/libanubhav" }
   ```
   Replace `"path/to/libanubhav"` with the actual path to the library in your file system. If you are using a public crate, you can use the crate version instead.

### Step 2: Import and use the library

Once the dependency is added, you can use **libanubhav** in your Rust code by importing the library and its components.

Here's how to use **libanubhav** in your project:

1. In your `main.rs` or any other file, import the necessary items:
   ```rust
   use libanubhav::{Library, Book};
   ```

2. Create a `Library` instance and interact with it:
   ```rust
   use std::io::{self, Write};

   fn main() {
       let mut library = Library::new();

       loop {
           println!("\nLibrary Management System");
           println!("1. Add Book");
           println!("2. List Available Books");
           println!("3. List Borrowed Books");
           println!("4. Borrow Book");
           println!("5. Return Book");
           println!("6. Exit");
           print!("Enter your choice: ");
           io::stdout().flush().unwrap();

           let mut choice = String::new();
           io::stdin().read_line(&mut choice).expect("Failed to read line");
           let choice: u32 = match choice.trim().parse() {
               Ok(num) => num,
               Err(_) => continue,
           };

           match choice {
               1 => {
                   let mut title = String::new();
                   let mut author = String::new();

                   print!("Enter book title: ");
                   io::stdout().flush().unwrap();
                   io::stdin().read_line(&mut title).expect("Failed to read line");

                   print!("Enter book author: ");
                   io::stdout().flush().unwrap();
                   io::stdin().read_line(&mut author).expect("Failed to read line");

                   library.add_book(&title.trim(), &author.trim());
               }
               2 => library.list_books(),
               3 => library.list_borrowed_books(),
               4 => {
                   let mut id = String::new();
                   print!("Enter book ID to borrow: ");
                   io::stdout().flush().unwrap();
                   io::stdin().read_line(&mut id).expect("Failed to read line");
                   let id: u32 = match id.trim().parse() {
                       Ok(num) => num,
                       Err(_) => continue,
                   };
                   library.borrow_book(id);
               }
               5 => {
                   let mut id = String::new();
                   print!("Enter book ID to return: ");
                   io::stdout().flush().unwrap();
                   io::stdin().read_line(&mut id).expect("Failed to read line");
                   let id: u32 = match id.trim().parse() {
                       Ok(num) => num,
                       Err(_) => continue,
                   };
                   library.return_book(id);
               }
               6 => break,
               _ => println!("Invalid choice! Please enter a number between 1 and 6."),
           }
       }
   }
   ```

### Step 3: Build and Run

Once the library is set up and you have written your code, you can build and run your project using Cargo.

1. Build the project:
   ```bash
   cargo build
   ```

2. Run the project:
   ```bash
   cargo run
   ```

### Example Usage

Here's an example of how **libanubhav** can be used in a simple command-line application:

- Add books by entering the title and author.
- List available books that are not currently borrowed.
- Borrow and return books by their ID.

### Example Output

```
Library Management System
1. Add Book
2. List Available Books
3. List Borrowed Books
4. Borrow Book
5. Return Book
6. Exit
Enter your choice: 2

Available Books:
ID: 1
Title: The Rust Programming Language
Author: Steve Klabnik and Carol Nichols
----------------------
ID: 2
Title: Clean Code
Author: Robert C. Martin
----------------------
...

Enter your choice: 4
Enter book ID to borrow: 1
You have successfully borrowed 'The Rust Programming Language'.
```

## Predefined Books

When a new `Library` is created, it is initialized with a few predefined books:
- "The Rust Programming Language" by Steve Klabnik and Carol Nichols
- "Clean Code" by Robert C. Martin
- "The Pragmatic Programmer" by Andrew Hunt and David Thomas
- "Design Patterns" by Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides

## License

This project is licensed under the MIT License.

