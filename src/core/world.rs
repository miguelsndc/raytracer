use super::{
    light::{lighting, Material, PointLight},
    object::{IntersectionState, Intersections, Object},
    ray::Ray,
    transforms::Transformations,
};
use crate::primitives::{color::Color, point::Point, tuple::Tuple};

pub struct World {
    objects: Vec<Object>,
    light_sources: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let s1 = Object::sphere().with_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let mut s2 = Object::sphere();
        s2.set_transformation(Transformations::scale(0.5, 0.5, 0.5));
        World {
            objects: vec![s1, s2],
            light_sources: vec![PointLight::new(
                Point::new(-10.0, 10.0, -10.0),
                Color::white(),
            )],
        }
    }
}

impl World {
    pub fn new() -> Self {
        return World {
            objects: vec![],
            light_sources: vec![],
        };
    }

    pub fn objects(&self) -> &Vec<Object> {
        return &self.objects;
    }

    pub fn push_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn push_light_source(&mut self, obj: PointLight) {
        self.light_sources.push(obj);
    }

    pub fn light_sources(&self) -> &Vec<PointLight> {
        return &self.light_sources;
    }

    pub fn intersect_world(&self, ray: &Ray) -> Intersections<'_> {
        let mut intersections = Intersections::new();

        for obj in &self.objects {
            let intersection = obj.intersect(ray);

            if intersection.ok {
                for i in intersection.i.iter() {
                    intersections.push(*i);
                }
            }
        }

        intersections.sort();

        return intersections;
    }

    pub fn shade_hit(&self, state: IntersectionState) -> Color {
        let mut color = Color::black();
        for light in self.light_sources.iter() {
            color += lighting(
                state.object.material(),
                light,
                state.point,
                state.eyev,
                state.normalv,
            );
        }
        return color;
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect_world(&ray);
        let hit = intersections.hit();

        if hit.is_none() {
            return Color::black();
        }

        let hit = hit.unwrap();
        let state = IntersectionState::new(*hit, ray);
        let color = self.shade_hit(state);

        return color;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{light::Material, object::Intersection, transforms::Transformations},
        primitives::vec3::Vec3,
    };

    use super::*;

    #[test]
    fn world_initialization() {
        let world = World::new();

        assert_eq!(world.light_sources().len(), 0);
        assert_eq!(world.objects().len(), 0);
    }

    #[test]
    fn default_world() {
        let light_source = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());
        let s1 = Object::sphere().with_material(Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        });
        let mut s2 = Object::sphere();
        s2.set_transformation(Transformations::scale(0.5, 0.5, 0.5));

        let w = World::default();

        assert_eq!(*w.light_sources.first().unwrap(), light_source);
        assert_eq!(w.objects()[0], s1);
        assert_eq!(w.objects()[1], s2);
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let xs = w.intersect_world(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection_from_outside() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let shape = w.objects().first().unwrap();
        let i = Intersection::new(4.0, shape);
        let comps = IntersectionState::new(i, r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = World::default();
        w.light_sources = vec![PointLight::new(Point::new(0.0, 0.25, 0.0), Color::white())];

        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let comps = IntersectionState::new(i, r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.90408, 0.90408, 0.90408));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::default();
        let mut outer = w.objects[0];
        let mut inner = w.objects[1];
        outer.set_material(Material {
            ambient: 1.0,
            ..outer.material()
        });
        inner.set_material(Material {
            ambient: 1.0,
            ..inner.material()
        });

        w.objects[0] = outer;
        w.objects[1] = inner;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vec3::new(0.0, 0.0, -1.0));
        let c = w.color_at(r);

        assert_eq!(c, inner.material().color);
    }
}
