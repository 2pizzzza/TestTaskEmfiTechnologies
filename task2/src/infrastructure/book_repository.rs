use crate::domain::book::Book;
use std::collections::HashMap;

pub trait BookStorage: Send + Sync {
    fn add_book(&mut self, book: Book) -> u32;
    fn get_book(&self, id: u32) -> Option<Book>;
    fn get_books(&self) -> Vec<Book>;
    fn delete_book(&mut self, id: u32) -> bool;
    fn update_book(&mut self, id: u32, book: Book) -> Option<Book>;
}

pub struct Storage {
    books: HashMap<u32, Book>,
    next_id: u32,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            next_id: 1,
        }
    }
}

impl BookStorage for Storage {
    fn add_book(&mut self, mut book: Book) -> u32 {
        let id = self.next_id;
        book.id = Some(id);
        self.books.insert(id, book);
        self.next_id += 1;
        id
    }

    fn get_book(&self, id: u32) -> Option<Book> {
        self.books.get(&id).cloned()
    }

    fn get_books(&self) -> Vec<Book> {
        self.books.values().cloned().collect()
    }

    fn delete_book(&mut self, id: u32) -> bool {
        self.books.remove(&id).is_some()
    }

    fn update_book(&mut self, id: u32, mut book: Book) -> Option<Book> {
        if self.books.contains_key(&id) {
            book.id = Some(id);
            self.books.insert(id, book.clone());
            Some(book)
        } else {
            None
        }
    }
}
