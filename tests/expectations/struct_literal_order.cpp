#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct ABC {
  float a;
  uint32_t b;
  uint32_t c;
};
static const ABC ABC_abc = { /* .a = */ 1.0, /* .b = */ 2, /* .c = */ 3 };
static const ABC ABC_bac = { /* .a = */ 1.0, /* .b = */ 2, /* .c = */ 3 };
static const ABC ABC_cba = { /* .a = */ 1.0, /* .b = */ 2, /* .c = */ 3 };

struct BAC {
  uint32_t b;
  float a;
  int32_t c;
};
static const BAC BAC_abc = { /* .b = */ 1, /* .a = */ 2.0, /* .c = */ 3 };
static const BAC BAC_bac = { /* .b = */ 1, /* .a = */ 2.0, /* .c = */ 3 };
static const BAC BAC_cba = { /* .b = */ 1, /* .a = */ 2.0, /* .c = */ 3 };

extern "C" {

void root(ABC a1, BAC a2);

} // extern "C"
