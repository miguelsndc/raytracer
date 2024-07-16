#include "vec.h"

// Vectors have homogeneous coordinate w set to 1, for now;
vec4 v4_add(vec4 t1, vec4 t2) {
    return v4_new(t1.x + t2.x, t1.y + t2.y,t2.z + t1.z, t1.w + t2.w);
}
vec4  v4_sub(vec4 t1, vec4 t2) {
    return v4_new(t1.x + t2.x,  t1.y + t2.y,  t2.z + t1.z, 0);
}

int v4_equals(vec4 t1, vec4 t2) {
    return t1.x == t2.x && t1.y == t2.y && t1.z == t2.z;
}


f32 v4_dot(vec4 u, vec4 v) {
    return u.x * v.x + u.y * v.y + u.z * v.z + u.w * v.w;
}

vec4 v4_cross(vec4 u, vec4 v) {
    return v4_new(u.y * v.z - u.z * v.y, u.z * v.x - u.x * v.z,  u.x * v.y - u.y * v.x, 0);
}

vec4 v4_scalar_mut(vec4 v, f32 t) {
    return v4_new(v.x * t, v.y * t, v.z * t, v.w * t);
}

f32 v4_mag(vec4 v) {
    return sqrtf(v4_dot(v, v));
}

vec4 v4_new(f32 x, f32 y, f32 z, f32 w) {
    vec4 v;
    v.x = x;
    v.y = y;
    v.z = z;
    v.w = w;
    return v;
}

vec4 v4_norml(vec4 v) {
    f32 mag = v4_mag(v);
    return v4_new(v.x / mag, v.y / mag, v.z / mag, v.w / mag);
}
