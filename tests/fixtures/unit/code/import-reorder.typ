/// typstyle: reorder_import_items

// Single import with mixed styles in random order
#import "module.typ": xyz, func as renamed, h.i.j, a.b.c, widget as tool, m.n, z.y.x, beta, p.q as custom, alpha, x.y as short, gamma, j.k.l as util

// Multiple imports from same module, should ideally be merged and reordered
#import "common.typ": format
#import "common.typ": layout as l, render
#import "common.typ": style.color, transform as t
#import "common.typ": data.table, data.chart as graph

// Imports with special names and deep nesting
#import "special.typ": _hidden, a.b.c.d.e.f, _private.func as _f, x-y-z as x_y_z, very.deep.nested.module.function as fn

// Conflicting names that should be preserved
#import "math.typ": sum
#import "stats.typ": sum as statistical_sum

// Empty and single item imports
#import "empty.typ":
#import "single.typ": only_item
#import "wildcard.typ": *

// Case 1: Import with comments between items - should not reorder
#import "commented.typ": (bbb, aaa,
  // This comment should prevent reordering
)
#import "commented.typ": ddd,  /* Another comment type */ ccc

// Case 2: Import with duplicate item names - should not reorder
// Same name imported twice
#import "duplicates.typ": helper, widget, helper, formatter

// Same name imported directly and via alias
#import "alias-duplicates.typ": config, settings as config, template

// Two aliases with the same target name
#import "same-alias.typ": render, format as util, transform as util, aliases
