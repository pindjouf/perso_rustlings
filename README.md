# Exos to learn Rust

## Short Rust CLI Course Outline

- [ ] 1. Basic File I/O Project -- [cw](url) 

    Goal: Write a program that reads from a text file and counts the number of lines, words, and characters in it (similar to the wc command in Unix).
    What You'll Learn: File handling, string manipulation, and iterators in Rust.

- [ ] 2. CLI Argument Parsing with Clap

    Goal: Extend the basic file I/O project to accept command-line arguments for the file path and an optional flag --words to print only the word count.
    What You'll Learn: Working with the clap crate for parsing CLI arguments, structuring CLI programs, and handling optional arguments.

- [ ] 3. Basic Note-Taking CLI

    Goal: Build a simple note-taking app that creates and appends notes to a file. The app should allow you to:
        Add a note
        List all notes
        Search for notes by keyword
    What You'll Learn: Working with file creation, appending data to files, and filtering content based on user input (which will help with searching in Kof).

- [ ] 4. Version Control Simulation

    Goal: Simulate a mini version control system. Create a CLI that:
        Initializes a new project by creating a directory
        Tracks changes in text files (simulating commits)
        Lists commit history
        Shows differences between file versions (basic diff)
    What You'll Learn: Managing directories and files, using structs to represent commits and files, and working with serde for serialization.

- [ ] 5. Directory Structure Management

    Goal: Write a Rust program that can create, delete, and list directories and files based on user input, mirroring how Kof handles its folder structure for notes.
    What You'll Learn: Using Rust’s std::fs for file and directory management, handling errors when directories or files don’t exist, and creating nested structures.

- [ ] 6. Automated Git Operations with Rust

    Goal: Automate simple Git operations (like add, commit, and push) using a Rust CLI that interacts with the Git system. This can be a simple wrapper around the Git command-line tool.
    What You'll Learn: Interacting with external commands using std::process::Command and automating workflows, which is key for building Kof’s note management.

- [ ] 7. Basic Synchronization

    Goal: Build a Rust CLI that syncs files between two directories (local to remote). You can create a mock "server" directory for this.
    What You'll Learn: Implementing file synchronization logic, handling differences between directories, and introducing networking features, which will be key for Kof’s sync functionality.
