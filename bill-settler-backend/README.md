# Bill Settler Backend 
This is the Rust workspace constituting the backend of the Bill Settler project.

## Database
This library is responsible for managing low-level database access and CRUD operations.

### Database Macro
A utility macro library that easily derives the traits necessary for common structs to be insertable and retrievable from the database.

## Models
These models are stored in the database and are basically wrappers for Gremlin structures (vertices and edges).

## Services
Underlying logic (Model creation, relation(edges) management)

# TODO
+ Add a unique_props trait and macro derivations to ensure the uniqueness of vertices and edges.
+ Modify the macros to use helper attributes instead of hardcoded values (id, source_id). Necessary for unique props derivation.
+ Implement missing models (transfer, curency, group_person, person_expense etc.).
+ Think heavily about splitting the bill and transitioning between a splitting schema to the absolute values (take into account a possible currency exchange).
+ Implement missing services 
+ Implement API
+ Think about a database for messaging.
