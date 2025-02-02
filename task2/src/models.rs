#[derive(Clone)]
pub struct Book {
    title: String,
    author: String,
    year: u32,
}

impl Book {
    pub fn new(title: String, author: String, year: u32) -> Self {
        Self { title, author, year }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_year(&self) -> u32 {
        self.year
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    pub fn set_year(&mut self, year: u32) {
        self.year = year;
    }
}

pub trait BookStorage {
    fn add_book(&mut self, book: Book) -> u32;
    fn get_book(&self, id: u32) -> Option<&Book>;
    fn get_books(&self) -> Vec<Book>;
    fn delete_book(&mut self, id: u32) -> bool;
}

pub struct InMemoryBookStorage {
    books: HashMap<u32, Book>,
    next_id: u32,
}

impl InMemoryBookStorage {
    pub fn new() -> Self {
        Self {
            books: HashMap::new(),
            next_id: 1,
        }
    }
}

impl BookStorage for InMemoryBookStorage {
    fn add_book(&mut self, book: Book) -> u32 {
        let id = self.next_id;
        self.books.insert(id, book);
        self.next_id += 1;
        id
    }

    fn get_book(&self, id: u32) -> Option<&Book> {
        self.books.get(&id)
    }

    fn get_books(&self) -> Vec<Book> {
        self.books.values().cloned().collect()
    }

    fn delete_book(&mut self, id: u32) -> bool {
        self.books.remove(&id).is_some()
    }
}


pub struct BookService<T: BookStorage> {
    storage: T,
}

impl<T: BookStorage> BookService<T> {
    pub fn new(storage: T) -> Self {
        Self { storage }
    }

    pub fn add_book(&mut self, title: String, author: String, year: u32) -> u32 {
        let book = Book::new(title, author, year);
        self.storage.add_book(book)
    }

    pub fn list_books(&self) -> Vec<Book> {
        self.storage.get_books()
    }
}


