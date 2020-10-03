from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct StylePoint_i32:
    int32_t x;
    int32_t y;

  ctypedef struct StylePoint_f32:
    float x;
    float y;

  cdef enum:
    Foo_i32,
    Bar_i32,
    Baz_i32,
    Bazz_i32,
  ctypedef uint8_t StyleFoo_i32_Tag;

  ctypedef struct StyleFoo_Body_i32:
    StyleFoo_i32_Tag tag;
    int32_t x;
    StylePoint_i32 y;
    StylePoint_f32 z;

  ctypedef struct StyleBar_Body_i32:
    StyleFoo_i32_Tag tag;
    int32_t _0;

  ctypedef struct StyleBaz_Body_i32:
    StyleFoo_i32_Tag tag;
    StylePoint_i32 _0;

  ctypedef union StyleFoo_i32:
    StyleFoo_i32_Tag tag;
    StyleFoo_Body_i32 foo;
    StyleBar_Body_i32 bar;
    StyleBaz_Body_i32 baz;

  ctypedef enum StyleBar_i32_Tag:
    Bar1_i32,
    Bar2_i32,
    Bar3_i32,
    Bar4_i32,

  ctypedef struct StyleBar1_Body_i32:
    int32_t x;
    StylePoint_i32 y;
    StylePoint_f32 z;
    int32_t (*u)(int32_t);

  ctypedef struct StyleBar2_Body_i32:
    int32_t _0;

  ctypedef struct StyleBar3_Body_i32:
    StylePoint_i32 _0;

  ctypedef struct StyleBar_i32:
    StyleBar_i32_Tag tag;
    StyleBar1_Body_i32 bar1;
    StyleBar2_Body_i32 bar2;
    StyleBar3_Body_i32 bar3;

  ctypedef struct StylePoint_u32:
    uint32_t x;
    uint32_t y;

  ctypedef enum StyleBar_u32_Tag:
    Bar1_u32,
    Bar2_u32,
    Bar3_u32,
    Bar4_u32,

  ctypedef struct StyleBar1_Body_u32:
    int32_t x;
    StylePoint_u32 y;
    StylePoint_f32 z;
    int32_t (*u)(int32_t);

  ctypedef struct StyleBar2_Body_u32:
    uint32_t _0;

  ctypedef struct StyleBar3_Body_u32:
    StylePoint_u32 _0;

  ctypedef struct StyleBar_u32:
    StyleBar_u32_Tag tag;
    StyleBar1_Body_u32 bar1;
    StyleBar2_Body_u32 bar2;
    StyleBar3_Body_u32 bar3;

  cdef enum:
    Baz1,
    Baz2,
    Baz3,
  ctypedef uint8_t StyleBaz_Tag;

  ctypedef struct StyleBaz1_Body:
    StyleBaz_Tag tag;
    StyleBar_u32 _0;

  ctypedef struct StyleBaz2_Body:
    StyleBaz_Tag tag;
    StylePoint_i32 _0;

  ctypedef union StyleBaz:
    StyleBaz_Tag tag;
    StyleBaz1_Body baz1;
    StyleBaz2_Body baz2;

  cdef enum:
    Taz1,
    Taz2,
    Taz3,
  ctypedef uint8_t StyleTaz_Tag;

  ctypedef struct StyleTaz1_Body:
    StyleBar_u32 _0;

  ctypedef struct StyleTaz2_Body:
    StyleBaz _0;

  ctypedef struct StyleTaz:
    StyleTaz_Tag tag;
    StyleTaz1_Body taz1;
    StyleTaz2_Body taz2;

  void foo(const StyleFoo_i32 *foo,
           const StyleBar_i32 *bar,
           const StyleBaz *baz,
           const StyleTaz *taz);
