# autodep-cli

This CLI application is designed to manage and update Please build files in Node projects, particularly in a monorepo setup. It focuses on analyzing and modifying build dependencies within these projects, with support for JavaScript, TypeScript, and potentially other languages in the future.

## Project Overview

The tool aims to provide an automated way to handle dependency management in Node projects, ensuring efficient and up-to-date build configurations.

### Key Features

- View and manage imports in Node files.
- Handle Please build files with a focus on preserving formatting and comments.
- Support for Autodep YAML configuration management.

## Development Phases

### Phase 1: Planning and Setup

1. ~~Define project scope and objectives.~~
2. ~~Select and research Rust crates for CLI handling, file parsing, and configuration management.~~
3. ~~Set up the initial project structure and version control.~~

### Phase 2: Core Development

4. Implement configuration file handling.
5. Develop file parsers for Node and Please build files.
6. Set up the command-line interface.

### Phase 3: Feature Implementation

7. Build core features for file viewing, analysis, and manipulation.
8. Integrate logging and verbose mode.

### Phase 4: Testing and Refinement

9. Write unit and integration tests.
10. Conduct manual testing and debugging.
11. Optimize performance and refine the code.

### Phase 5: Documentation and Finalization

12. Document the application and create a README.
13. Prepare the application for release.

### Phase 6: Future Planning

14. Outline potential extensions, such as dependency graph visualization.

## Technologies Used

- Rust
- `clap`/`structopt` for CLI
- `serde` and `config` for configuration handling
- `rustpython-parser` for parsing Python-like Please build files
- `swc` for parsing JavaScript/TypeScript

## Author

Sam Folorunsho
