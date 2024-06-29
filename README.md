# Notes Rust IO

This project implements a CRUD application similar to a classic notes app, written in Rust. Instead of using a graphical user interface (UI), it operates entirely through a terminal interface.

## Storing data 

The program offers two storage modules, each providing an I/O interface via the terminal for managing CRUD operations on a Note entity:

- JSON File Storage: provides a simple storage method but requires managing a file for each entity within the application.

- SQLite Database Storage, offers more complexity but is more flexible than JSON file storage. It can be slower compared to storing data in JSON files.

## Modules

Each storage module follows a structured format consisting of three submodules:

- file module: manages file operations specific to the module’s data storage method.

- note module: handles CRUD operations for notes. It directly interfaces with the file submodule of its respective storage module.

- io module: provides a user interface for interacting with the project. While it uses the note submodule for actions, it does not interact directly with the file submodule.

When navigating the project’s repository, you will encounter two main modules:

- db module: performs CRUD operations on the Note entity using a
SQLite database named `.notes.sqlite`
- json module: manages CRUD operations on Note entity, storing
data in a JSON file named `.notes.json`

