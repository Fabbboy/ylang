# Sable

This repository contains the sources for the Sable programming language.

## Tree-sitter grammar

A basic [Tree-sitter](https://tree-sitter.github.io/) grammar lives in
`tree-sitter-sable`. After running `npm install` inside this directory you can
run `npx tree-sitter generate` to build the parser. Editors like VSCode can use
this grammar for syntax highlighting.
