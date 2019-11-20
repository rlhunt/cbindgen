#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <uchar.h>

enum Foo_Tag {
  A,
};

struct A_Body {
  float _0[20];
};

struct Foo {
  enum Foo_Tag tag;
  union {
    struct A_Body a;
  };
};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void root(struct Foo a);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
