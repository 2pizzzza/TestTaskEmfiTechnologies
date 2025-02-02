use crate::domain::book::Book;
use crate::infrastructure::book_repository::BookStorage;
use std::sync::{Arc, Mutex};


pub struct BookService<T: BookStorage> {
    storage: Arc<Mutex<T>>,
}

impl<T: BookStorage> BookService<T> {
    pub fn new(storage: Arc<Mutex<T>>) -> Self {
        Self { storage }
    }

    pub fn add_book(&self, book: Book) -> u32 {
        let mut storage = self.storage.lock().unwrap();
        storage.add_book(book)
    }

    pub fn get_book(&self, id: u32) -> Option<Book> {
        let storage = self.storage.lock().unwrap();
        storage.get_book(id)
    }

    pub fn get_books(&self) -> Vec<Book> {
        let storage = self.storage.lock().unwrap();
        storage.get_books()
    }

    pub fn delete_book(&self, id: u32) -> bool {
        let mut storage = self.storage.lock().unwrap();
        storage.delete_book(id)
    }

    pub fn update_book(&self, id: u32, book: Book) -> bool {
        let mut storage = self.storage.lock().unwrap();
        storage.update_book(id, book).is_some()
    }
}
