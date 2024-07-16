#ifndef ASSERT
#define ASSERT(_e, ...) if ((!(_e))) {fprintf(stderr, __VA_ARGS__); exit(1);}
#endif

#ifndef min
#define min(_a, _b) ({ __typeof__(_a) __a = (_a), __b = (_b); __a < __b ? __a : __b; })
#endif

#ifndef max
#define max(_a, _b) ({ __typeof__(_a) __a = (_a), __b = (_b); __a > __b ? __a : __b; })
#endif

#ifndef clamp
#define clamp(_x, _mi, _ma) (min(max(_x, _mi), _ma))
#endif