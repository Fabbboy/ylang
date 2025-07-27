module.exports = grammar({
  name: 'sable',

  extras: $ => [ /\s/, $.comment ],

  rules: {
    program: $ => repeat($.function_decl),

    function_decl: $ => seq(
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

    parameter: $ => seq($.identifier, ':', $.type),

    type: $ => seq($.identifier, optional($.pointer_suffix)),

    pointer_suffix: $ => repeat1('*'),

    block_or_semi: $ => choice($.block, ';'),

    block: $ => seq('{', repeat($.statement), '}'),

    statement: $ => choice($.variable_decl, $.expression_stmt),

    variable_decl: $ => seq(
      'var',
      $.identifier,
      optional(seq(':', $.type)),
      '=',
      $.expression,
      ';'
    ),

    expression_stmt: $ => seq($.expression, ';'),

    expression: $ => $.assignment,

    assignment: $ => choice(
      seq($.identifier, '=', $.expression),
      $.additive
    ),

    additive: $ => seq(
      $.multiplicative,
      repeat(seq(choice('+', '-'), $.multiplicative))
    ),

    multiplicative: $ => seq(
      $.primary,
      repeat(seq(choice('*', '/'), $.primary))
    ),

    primary: $ => choice(
      $.literal,
      $.identifier,
      seq('(', $.expression, ')')
    ),

    literal: $ => choice($.integer_literal, $.float_literal),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    integer_literal: $ => /\d+/, 

    float_literal: $ => /\d+\.\d+/, 

    comment: $ => token(choice(
      seq('//', /[^\n]*/),
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
    ))
  }
});
