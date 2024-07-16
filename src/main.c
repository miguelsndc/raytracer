#include <stdio.h>
#include "types.h"
#include "utils.h"
#include "vec.h"

typedef struct  {
    vec4 pos, vel;
} projectile;

typedef struct {
    f32 x, y, z;
} point;

typedef struct {
    vec4 gravity, wind;
} scene;

void tick(scene env, projectile proj) {
    projectile p;
    p.pos = v4_add(p.pos, p.vel);
    p.vel = v4_add(proj.vel, v4_add(env.gravity, env.wind));
    return p;
}

int main() {
}