#ifndef VEC_H
#define VEC_H

#include "types.h"
#include "utils.h"
#include <math.h>

typedef struct {
    f32 x, y, z, w;
} vec4;

// Vectors have homogeneous coordinate w set to 1, for now;
vec4 v4_add(vec4 t1, vec4 t2);
vec4  v4_sub(vec4 t1, vec4 t2);
int v4_equals(vec4 t1, vec4 t2);
f32 v4_dot(vec4 u, vec4 v);
vec4 v4_cross(vec4 u, vec4 v);
vec4 v4_scalar_mut(vec4 v, f32 t);
f32 v4_mag(vec4 v);
vec4 v4_norml(vec4 v);
vec4 v4_new(f32 x, f32 y, f32 z, f32 w);

#endif
