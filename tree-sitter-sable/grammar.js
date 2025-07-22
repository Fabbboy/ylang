/**
 * @file Sable is a compiled language inspired by C, Rust and Swift
 * @author Fabrice
 * @license Apache-2.0
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "sable",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
