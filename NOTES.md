# Overview

This CLI application will initially be responsible for Node projects in a monorepo setup. The code may be written in JavaScript or TypeScript, with some `.css`, `.scss` and `.json` files. Some build rules may have Golang files as dependencies, but I am not concerned with them just yet.

# Requirements
- I will need this to run against any repository containing a valid Node project. These may also be a part of a yarn workspace setup.
- I need to have access to all the imports and module names in a project, primarily because I will need access to the full filepath to read files and create files relative to their location
- I want to design this so it can easily be set up to be triggered on save in any Text Editor or IDE. I will be handling integration later.
- I will need to be able to parse the Please build files; The Please DSL is a somewhat strict subset of the Python 3 grammar, so a python3 parser will be necessary and sufficient
- I may also need a parser for JavaScript and TypeScript files, unless there is another way to reliably extract a list of imports and their relative paths.
- I want to provide some logging if the situation demands
- I want to manage the behaviour of the CLI app using YAML config files, or TOML, but I also want config to be passable via the command line.
- YAML config should be “mergeable” with parent configs, via an `extends` clause, similar to how a `.tsconfig` file works
- I want to keep this separate from Please, so would prefer not to call any `plz` commands directly. I may extend the CLI app to support other build systems in future
- I want to parse the Please build files, make modifications, and write back to disk whilst preserving comments and any formatting (within reason)

# Functionality to cover
- View all imports in a Node file as they are declared
- View all imports as absolute paths as absolute relative to the project root
- View a single build rule as defined in a target Please file
- View the location of the nearest build rule to a target Node file
- Expand any glob declarations in build rules to ensure all dependencies are captured
- Allow the user to specify whether the CLI app should update the nearest parent build file, or create one within the immediate directory if one does not exist
- Minor control over how dependencies are written (canonicalised, shorthand, etc.)
- Perhaps a “verbose” mode to serve as tracing for debugging purposes
- Perform the dependency update process on a single target file
- Perform the dependency update process recursively on an entire directory
- Provide insights and metrics on imports and build files
- Provide an “untangle” action to recreate all build files in a directory to ensure the most efficient dependency graph
- Possibly allow users to define “templates” for their custom build rules to be used when creating new rules or new files
- The ability to “prune” all unused build rules in a build file

The nature of these build files is such that I need to be aware of the `name`, `src(s)`, `deps` and `visibility` of each build rule, so I will have to provide a section in config for users to configure these. The application should have sensible defaults so the simplest cases are covered without too much fiddling.

Eventually, it would be great to visualise the dependency graph, but this is not important for now
