module.exports = grammar({
  name: 'celestia_property_list',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  rules: {
    source_file: $ => repeat($._declaration),

    comment: $ => token(seq('#', /.*/)),

    _declaration: $ => choice(
      $.configuration_block,
      $.ssc_declaration,
      $.stc_declaration,
      $.dsc_declaration,
      $.virtual_texture_block,
      $.named_block,
      $.anonymous_block,
      $.property_entry,
    ),

    configuration_block: $ => prec(4, seq(
      field('type', 'Configuration'),
      field('body', $.property_group),
    )),

    ssc_declaration: $ => prec(3, seq(
      optional(field('disposition', choice('Add', 'Replace', 'Modify'))),
      optional(field('kind', choice('Body', 'ReferencePoint', 'SurfaceObject', 'AltSurface', 'Location'))),
      field('name', $.string),
      field('parent', $.string),
      field('body', $.property_group),
    )),

    stc_declaration: $ => prec(2, seq(
      optional(field('disposition', choice('Add', 'Replace', 'Modify'))),
      optional(field('kind', choice('Star', 'Barycenter'))),
      optional(field('catalog_number', $.number)),
      field('name', $.string),
      field('body', $.property_group),
    )),

    dsc_declaration: $ => prec(3, seq(
      field('kind', choice('Galaxy', 'Globular', 'Nebula', 'OpenCluster', 'galaxy', 'globular', 'nebula', 'opencluster')),
      field('name', $.string),
      field('body', $.property_group),
    )),

    virtual_texture_block: $ => prec(4, seq(
      field('type', 'VirtualTexture'),
      field('body', $.property_group),
    )),

    named_block: $ => prec(1, seq(
      field('name', $.string),
      field('body', $.property_group),
    )),

    anonymous_block: $ => prec(0, $.property_group),

    property_group: $ => seq(
      '{',
      repeat($.property_entry),
      '}',
    ),

    property_entry: $ => prec(0, seq(
      field('key', choice($.identifier, $.string)),
      optional(field('unit', $.unit)),
      field('value', $._value),
    )),

    unit: $ => seq('<', $.identifier, '>'),

    _value: $ => choice(
      $.string,
      $.number,
      $.boolean,
      $.array,
      $.property_group,
      $.identifier,
    ),

    array: $ => seq(
      '[',
      repeat($._value),
      ']',
    ),

    string: $ => token(seq(
      '"',
      repeat(choice(
        /[^"\\\r\n]+/,
        seq('\\', /./)
      )),
      '"'
    )),

    number: $ => token(seq(
      optional('-'),
      /\d+/,
      optional(seq('.', /\d+/)),
      optional(seq(/[eE]/, optional(/[+-]/), /\d+/))
    )),

    boolean: $ => choice('true', 'false'),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_\-\.\/]*/,
  }
});
