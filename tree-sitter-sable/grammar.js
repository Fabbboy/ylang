/**
 * @file Sable is a compiled language inspired by C, Rust and Swift
 * @author Fabrice
 * @license Apache-2.0
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
// @ts-ignore - Tree-sitter DSL functions are injected at runtime

module.exports = grammar({
  name: "sable",

  extras: $ => [
    /\s/,
    $.comment,
  ],

  rules: {
    source_file: $ => repeat($.function_declaration),

    function_declaration: $ => seq(
      'func',
      $.identifier,
      '(',
      optional($.parameter_list),
      ')',
      ':',
      $.type,
      $.block_or_semi
    ),

    parameter_list: $ => seq(
      $.parameter,
      repeat(seq(',', $.parameter))
    ),

    parameter: $ => seq(
      $.identifier,
      ':',
      $.type
    ),

    type: $ => seq(
      $.identifier,
      optional($.pointer_suffix)
    ),

    pointer_suffix: $ => repeat1('*'),

    block_or_semi: $ => choice(
      $.block,
      ';'
    ),

    block: $ => seq(
      '{',
      repeat($.statement),
      '}'
    ),

    statement: $ => choice(
      $.variable_declaration,
      $.expression_statement,
    ),

    variable_declaration: $ => seq(
      'var',
      $.identifier,
      optional(seq(':', $.type)),
      '=',
      $.expression,
      ';'
    ),

    expression_statement: $ => seq(
      $.expression,
      ';'
    ),

    expression: $ => $.assignment,

    assignment: $ => choice(
      seq($.identifier, '=', $.expression),
      $.additive
    ),

    additive: $ => prec.left(1, seq(
      $.multiplicative,
      repeat(seq(
        choice('+', '-'),
        $.multiplicative
      ))
    )),

    multiplicative: $ => prec.left(2, seq(
      $.primary,
      repeat(seq(
        choice('*', '/'),
        $.primary
      ))
    )),

    primary: $ => choice(
      $.literal,
      $.identifier,
      seq('(', $.expression, ')')
    ),

    literal: $ => choice(
      $.integer_literal,
      $.float_literal
    ),

    integer_literal: $ => /[0-9]+/,

    float_literal: $ => /[0-9]+\.[0-9]+/,

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    comment: $ => token(
      choice(
        seq('//', /[^\r\n]*/),
        seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
      )
    )
  }
});

