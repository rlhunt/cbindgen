#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#if (defined(PLATFORM_WIN) || defined(M_32))
enum BarType {
  A,
  B,
  C,
};
typedef uint32_t BarType;
#endif

#if (defined(PLATFORM_UNIX) && defined(X11))
enum FooType {
  A,
  B,
  C,
};
typedef uint32_t FooType;
#endif

typedef struct ConditionalField {
  #if defined(X11)
  int32_t field
  #endif
  ;
} ConditionalField;

#if (defined(PLATFORM_UNIX) && defined(X11))
typedef struct FooHandle {
  FooType ty;
  int32_t x;
  float y;
} FooHandle;
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
typedef struct BarHandle {
  BarType ty;
  int32_t x;
  float y;
} BarHandle;
#endif

void cond(ConditionalField a);

#if (defined(PLATFORM_UNIX) && defined(X11))
void root(FooHandle a);
#endif

#if (defined(PLATFORM_WIN) || defined(M_32))
void root(BarHandle a);
#endif
