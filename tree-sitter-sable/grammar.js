module.exports = grammar({
  name: 'sable',

  extras: $ => [ /\s/, $.comment ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($.function),

    comment: $ => token(seq('//', /.*/)),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    number: $ => choice($.float, $.integer),

    integer: $ => /[0-9]+/,
    float: $ => /[0-9]+\.[0-9]+/,

    type: $ => seq($.type_atom, repeat('*')),
    type_atom: $ => choice(
      $.primitive_type,
      $.identifier
    ),
    primitive_type: $ => choice('i8', 'i16', 'i32', 'f32', 'f64'),

    function: $ => seq(
      'func',
      field('name', $.identifier),
      field('parameters', $.parameter_list),
      ':',
      field('return_type', $.type),
      field('body', choice($.block, seq(';')))
    ),

    parameter_list: $ => seq(
      '(',
      optional(sepBy1(',', $.parameter)),
      ')'
    ),

    parameter: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $.type)
    ),

    block: $ => seq('{', repeat($.statement), '}'),

    statement: $ => choice(
      $.variable_statement,
      $.expression_statement
    ),

    variable_statement: $ => seq(
      'var',
      field('name', $.identifier),
      optional(seq(':', field('type', $.type))),
      '=',
      field('value', $.expression),
      ';'
    ),

    expression_statement: $ => seq($.expression, ';'),

    expression: $ => choice(
      $.assignment,
      $.binary_expression,
      $.identifier,
      $.number,
      $.block,
      $.parenthesized_expression
    ),

    parenthesized_expression: $ => seq('(', $.expression, ')'),

    assignment: $ => prec.right(seq(
      field('left', $.identifier),
      '=',
      field('right', $.expression)
    )),

    binary_expression: $ => choice(
      prec.left(seq($.expression, '+', $.expression)),
      prec.left(seq($.expression, '-', $.expression)),
      prec.left(seq($.expression, '*', $.expression)),
      prec.left(seq($.expression, '/', $.expression))
    )
  }
});

function sepBy1(sep, rule) {
  return seq(rule, repeat(seq(sep, rule)));
}
