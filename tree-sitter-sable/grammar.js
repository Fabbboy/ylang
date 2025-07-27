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

  extras: ($) => [/\s/, $.comment],

  rules: {
    source_file: ($) => repeat($.function_declaration),

    semi: ($) => ";",
    lparent: ($) => "(",
    rparent: ($) => ")",
    lbrace: ($) => "{",
    rbrace: ($) => "}",
    colon: ($) => ":",
    comma: ($) => ",",

    plus: ($) => "+",
    minus: ($) => "-",
    star: ($) => "*",
    slash: ($) => "/",
    equal: ($) => "=",

    var_kw: ($) => "var",
    func_kw: ($) => "func",

    function_declaration: ($) =>
      seq(
        $.func_kw,
        $.identifier,
        $.lparent,
        optional($.parameter_list),
        $.rparent,
        $.colon,
        $.type,
        $.block_or_semi
      ),

    parameter_list: ($) => seq($.parameter, repeat(seq($.comma, $.parameter))),

    parameter: ($) => seq($.identifier, $.colon, $.type),

    type: ($) => seq($.identifier, optional($.pointer_suffix)),

    pointer_suffix: ($) => repeat1($.star),

    block_or_semi: ($) => choice($.block, $.semi),

    block: ($) => seq($.lbrace, repeat($.statement), $.rbrace),

    statement: ($) => choice($.variable_declaration, $.expression_statement),

    variable_declaration: ($) =>
      seq(
        $.var_kw,
        $.identifier,
        optional(seq($.colon, $.type)),
        "=",
        $.expression,
        $.semi
      ),

    expression_statement: ($) => seq($.expression, $.semi),

    expression: ($) => $.assignment,

    assignment: ($) => choice(seq($.identifier, $.equal, $.expression), $.additive),

    additive: ($) =>
      prec.left(
        1,
        seq($.multiplicative, repeat(seq(choice($.plus, $.minus), $.multiplicative)))
      ),

    multiplicative: ($) =>
      prec.left(2, seq($.primary, repeat(seq(choice($.star, $.slash), $.primary)))),

    primary: ($) =>
      choice($.literal, $.identifier, seq($.lparent, $.expression, $.rparent)),

    literal: ($) => choice($.integer_literal, $.float_literal),

    integer_literal: ($) => /[0-9]+/,

    float_literal: ($) => /[0-9]+\.[0-9]+/,

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,
    comment: ($) =>
      token(
        choice(
          seq("//", /[^\r\n]*/),
          seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")
        )
      ),
  },
});
