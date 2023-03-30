# Bill Settler Backend 
This is the Rust workspace constituting the backend of the Bill Settler project.

## Database
This library is responsible for managing low-level database access and CRUD operations.

### Database Macro
A utility macro library that easily derives the traits necessary for common structs to be insertable and retrievable from the database.

## Models
These models are stored in the database and are basically wrappers for Gremlin structures (vertices and edges).
