# project-root

A small utility to find project root.

## Installation

```
$ cargo install --force --git https://github.com/ul/project-root.git
```

## Usage

```
$ project-root <NAME>...
```

where `<NAME>...` is project root markers in priority order. Search starts from the current
directory. When successful, `project-root` will print the path to the project root. Otherwise it
will just exit with the error code 1.

For example

```
$ project-root lerna.json package.json .git
```

will try to find `lerna.json` while traversing up to the filesystem root in case current directory
is inside of a Lerna-managed monorepo. If it fails, `project-root` will test for `package.json` then
if it's a regular npm package. And as the last resort it will try to find Git repository root.


