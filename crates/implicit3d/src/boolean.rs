use crate::{
    normal_from_object, BoundingBox, Object, PrimitiveParameters, RealField, ALWAYS_PRECISE,
};
use num_traits::Float;

const FADE_RANGE: f32 = 0.1;
const R_MULTIPLIER: f32 = 1.0;

/// Union create an implict function as the union of its inputs.
#[derive(Clone, Debug)]
pub struct Union<S: RealField> {
    objs: Vec<Box<dyn Object<S>>>,
    r: S,
    exact_range: S, // Calculate smooth transitions over this range
    fade_range: S,  // Fade normal over this fraction of the smoothing range
    bbox: BoundingBox<S>,
}

impl<S: RealField + Float + From<f32>> Union<S> {
    /// Create a union of all the objects in v. The union will be rounded, if r > 0.
    pub fn from_vec(mut v: Vec<Box<dyn Object<S>>>, r: S) -> Option<Box<dyn Object<S>>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let mut bbox = v
                    .iter()
                    .fold(BoundingBox::<S>::neg_infinity(), |union_box, x| {
                        union_box.union(x.bbox())
                    });
                bbox.dilate(r * From::from(0.2f32)); // dilate by some factor of r
                Some(Box::new(Union {
                    objs: v,
                    r,
                    bbox,
                    exact_range: r * From::from(R_MULTIPLIER),
                    fade_range: From::from(FADE_RANGE),
                }))
            }
        }
    }
}

impl<S: RealField + From<f32> + Float> Object<S> for Union<S> {
    fn approx_value(&self, p: &na::Point3<S>, slack: S) -> S {
        let approx = self.bbox.distance(p);
        if approx <= slack {
            rvmin(
                &self
                    .objs
                    .iter()
                    .map(|o| o.approx_value(p, slack + self.r))
                    .collect::<Vec<S>>(),
                self.r,
                self.exact_range,
            )
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn set_parameters(&mut self, p: &PrimitiveParameters<S>) {
        self.exact_range = self.r * p.r_multiplier;
        self.fade_range = p.fade_range;
        for o in &mut self.objs {
            o.set_parameters(p);
        }
    }
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S> {
        // Find the two smallest values with their indices.
        let (v0, v1) = self.objs.iter().enumerate().fold(
            ((0, S::infinity()), (0, S::infinity())),
            |(v0, v1), x| {
                let t = x.1.approx_value(p, From::from(ALWAYS_PRECISE));
                if t < v0.1 {
                    ((x.0, t), v0)
                } else if t < v1.1 {
                    (v0, (x.0, t))
                } else {
                    (v0, v1)
                }
            },
        );
        let one: S = From::from(1f32);
        match Float::abs(v0.1 - v1.1) {
            // if they are close together, calc normal from full object
            diff if diff < (self.exact_range * (one - self.fade_range)) => {
                // else,
                normal_from_object(self, p)
            }
            diff if diff < self.exact_range => {
                let fader = (diff / self.exact_range - one + self.fade_range) / self.fade_range;
                (self.objs[v0.0].normal(p) * fader + normal_from_object(self, p) * (one - fader))
                    .normalize()
            }
            // they are far apart, use the min's normal
            _ => self.objs[v0.0].normal(p),
        }
    }
}

/// Intersect objects.
#[derive(Clone, Debug)]
pub struct Intersection<S: RealField> {
    objs: Vec<Box<dyn Object<S>>>,
    r: S,
    exact_range: S, // Calculate smooth transitions over this range
    fade_range: S,  // Fade normal over this fraction of the smoothing range
    bbox: BoundingBox<S>,
}

impl<S: RealField + Float + From<f32>> Intersection<S> {
    /// Create an intersection of the objects in v. The intersection will be rounded, if r > 0.
    pub fn from_vec(mut v: Vec<Box<dyn Object<S>>>, r: S) -> Option<Box<dyn Object<S>>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let bbox = v
                    .iter()
                    .fold(BoundingBox::<S>::infinity(), |intersection_box, x| {
                        intersection_box.intersection(x.bbox())
                    });
                Some(Box::new(Intersection {
                    objs: v,
                    r,
                    bbox,
                    exact_range: r * From::from(R_MULTIPLIER),
                    fade_range: From::from(FADE_RANGE),
                }))
            }
        }
    }
    /// Create a Difference from Vec. The resulting object is v[0] minus all the other objects.
    /// Minus is implemented as intersection with negation.
    /// The difference will be rounded, if r > 0.
    pub fn difference_from_vec(mut v: Vec<Box<dyn Object<S>>>, r: S) -> Option<Box<dyn Object<S>>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let neg_rest = Negation::from_vec(&v.split_off(1));
                v.extend(neg_rest);
                Intersection::from_vec(v, r)
            }
        }
    }
}

impl<S: RealField + From<f32> + Float> Object<S> for Intersection<S> {
    fn approx_value(&self, p: &na::Point3<S>, slack: S) -> S {
        let approx = self.bbox.distance(p);
        if approx <= slack {
            rvmax(
                &self
                    .objs
                    .iter()
                    .map(|o| o.approx_value(p, slack + self.r))
                    .collect::<Vec<S>>(),
                self.r,
                self.exact_range,
            )
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox<S> {
        &self.bbox
    }
    fn set_parameters(&mut self, p: &PrimitiveParameters<S>) {
        self.exact_range = self.r * p.r_multiplier;
        self.fade_range = p.fade_range;
        for o in &mut self.objs {
            o.set_parameters(p);
        }
    }
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S> {
        // Find the two largest values with their indices.
        let (v0, v1) = self.objs.iter().enumerate().fold(
            ((0, S::neg_infinity()), (0, S::neg_infinity())),
            |(v0, v1), x| {
                let t = x.1.approx_value(p, From::from(ALWAYS_PRECISE));
                if t > v0.1 {
                    ((x.0, t), v0)
                } else if t > v1.1 {
                    (v0, (x.0, t))
                } else {
                    (v0, v1)
                }
            },
        );
        let one: S = From::from(1f32);
        match Float::abs(v0.1 - v1.1) {
            // if they are close together, calc normal from full object
            diff if diff < (self.exact_range * (one - self.fade_range)) => {
                // else,
                normal_from_object(self, p)
            }
            diff if diff < self.exact_range => {
                let fader = (diff / self.exact_range - one + self.fade_range) / self.fade_range;
                (self.objs[v0.0].normal(p) * fader + normal_from_object(self, p) * (one - fader))
                    .normalize()
            }
            // they are far apart, use the max' normal
            _ => self.objs[v0.0].normal(p),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Negation<S: RealField> {
    object: Box<dyn Object<S>>,
    infinity_bbox: BoundingBox<S>,
}

impl<S: RealField + Float + From<f32>> Negation<S> {
    pub fn new(o: Box<dyn Object<S>>) -> Self {
        Negation {
            object: o,
            infinity_bbox: BoundingBox::<S>::infinity(),
        }
    }
    pub fn from_vec(v: &[Box<dyn Object<S>>]) -> Vec<Box<dyn Object<S>>> {
        v.iter()
            .map(|o| Box::new(Negation::new(o.clone())) as Box<dyn Object<S>>)
            .collect()
    }
}

impl<S: RealField + From<f32> + Float> Object<S> for Negation<S> {
    fn approx_value(&self, p: &na::Point3<S>, slack: S) -> S {
        -self.object.approx_value(p, slack)
    }
    fn normal(&self, p: &na::Point3<S>) -> na::Vector3<S> {
        let _n1: S = From::from(-1f32);
        self.object.normal(p) * _n1
    }
    fn bbox(&self) -> &BoundingBox<S> {
        &self.infinity_bbox
    }
}

fn rvmin<S: Float + From<f32>>(v: &[S], r: S, exact_range: S) -> S {
    let mut close_min = false;
    let minimum = v.iter().fold(S::infinity(), |min, x| {
        if x < &min {
            if (min - *x) < exact_range {
                close_min = true;
            } else {
                close_min = false;
            }
            *x
        } else {
            if (*x - min) < exact_range {
                close_min = true;
            }
            min
        }
    });
    if !close_min {
        return minimum;
    }
    let min_plus_r = minimum + r;
    let r4 = r / From::from(4f32);
    // Inpired by http://iquilezles.org/www/articles/smin/smin.htm
    let exp_sum = v
        .iter()
        .filter(|&x| x < &min_plus_r)
        .fold(From::from(0f32), |sum: S, x| sum + (-*x / r4).exp());
    Float::ln(exp_sum) * -r4
}

fn rvmax<S: Float + From<f32>>(v: &[S], r: S, exact_range: S) -> S {
    let mut close_max = false;
    let maximum = v.iter().fold(S::neg_infinity(), |max, x| {
        if x > &max {
            if (*x - max) < exact_range {
                close_max = true;
            } else {
                close_max = false;
            }
            *x
        } else {
            if (max - *x) < exact_range {
                close_max = true;
            }
            max
        }
    });
    if !close_max {
        return maximum;
    }
    let max_minus_r = maximum - r;
    let r4 = r / From::from(4f32);
    let exp_sum = v
        .iter()
        .filter(|&x| x > &max_minus_r)
        .fold(From::from(0f32), |sum: S, x| sum + (*x / r4).exp());
    Float::ln(exp_sum) * r4
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::MockObject;

    #[test]
    fn union() {
        let m1 = MockObject::new(1.0, na::Vector3::new(1., 0., 0.));
        let m2 = MockObject::new(2.0, na::Vector3::new(0., 1., 0.));
        let union = Union::from_vec(vec![Box::new(m1), Box::new(m2)], 0.).unwrap();
        assert_ulps_eq!(union.approx_value(&na::Point3::new(0., 0., 0.), 0.), 1.);
        assert_ulps_eq!(
            union.normal(&na::Point3::new(0., 0., 0.)),
            na::Vector3::new(1., 0., 0.)
        );
    }

    #[test]
    fn intersection() {
        let m1 = MockObject::new(1.0, na::Vector3::new(1., 0., 0.));
        let m2 = MockObject::new(2.0, na::Vector3::new(0., 1., 0.));
        let is = Intersection::from_vec(vec![Box::new(m1), Box::new(m2)], 0.).unwrap();
        assert_ulps_eq!(is.approx_value(&na::Point3::new(0., 0., 0.), 0.), 2.);
        assert_ulps_eq!(
            is.normal(&na::Point3::new(0., 0., 0.)),
            na::Vector3::new(0., 1., 0.)
        );
    }

    #[test]
    fn negation() {
        let m = MockObject::new(1.0, na::Vector3::new(1., 0., 0.));
        let n = Negation::from_vec(&[Box::new(m)])[0].clone();
        assert_ulps_eq!(n.approx_value(&na::Point3::new(0., 0., 0.), 0.), -1.);
        assert_ulps_eq!(
            n.normal(&na::Point3::new(0., 0., 0.)),
            na::Vector3::new(-1., 0., 0.)
        );
    }
}
