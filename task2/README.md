# Task 2

## Features

- **Create a new book**
- **Read a list of all books**
- **Read a specific book by ID**
- **Update an existing book by ID**
- **Delete a book by ID**

## Requirements

- Rust
- Cargo
- Actix-web

## Setup

1 **Build the project:**
   ```sh
   cargo build
   ```

2 **Run the server:**
   ```sh
   cargo run
   ```

## API Endpoints


### **Get all books**
Request:
```json
{"action": "get_books", "data":{}}
```

Response:
```json
{
  "status": "ok",
  "books": [
    {"id": 1, "title": "Book Title", "author": "Author Name", "year": 2022}
  ]
}
```

### **Get a specific book by ID**
Request:
```json
{
  "action": "get_book",
  "data": {"id": 1}
}
```
Response (Success):
```json
{
  "status": "ok",
  "book": {"id": 1, "title": "Book Title", "author": "Author Name", "year": 2022}
}
```
Response (Error):
```json
{
  "status": "error",
  "message": "Book not found"
}
```

### **Add a new book**
Request:
```json
{
  "action": "add_book",
  "data": {
    "title": "New Book",
    "author": "New Author",
    "year": 2025
  }
}
```
Response:
```json
{
  "status": "ok",
  "id": 42
}
```

### **Update an existing book by ID**
Request:
```json
{
  "action": "update_book",
  "data": {
    "id": 1,
    "title": "Updated Title",
    "author": "Updated Author",
    "year": 2026
  }
}
```
Response (Success):
```json
{
  "status": "ok"
}
```
Response (Error):
```json
{
  "status": "error",
  "message": "Book not found"
}
```

### **Delete a book by ID**
Request:
```json
{
  "action": "delete_book",
  "data": {"id": 1}
}
```
Response (Success):
```json
{
  "status": "ok"
}
```
Response (Error):
```json
{
  "status": "error",
  "message": "Book not found"
}
```
```

