use cgmath::{InnerSpace, Point3, Vector3, Zero};

#[derive(Debug, Copy, Clone)]
struct Plane {
    normal: Vector3<f32>,
    distance: f32,
}

pub struct Frustum {
    planes: [Plane; 6],
}

impl Frustum {
    pub fn from_view_projection(projection_matrix: &[[f32; 4]; 4]) -> Self {
        let m = projection_matrix;
        let mut planes = [Plane {
            normal: Vector3::zero(),
            distance: 0.0,
        }; 6];

        for i in 0..6 {
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let row = i / 2;
            let normal = Vector3::new(
                m[0][3] + m[0][row] * sign,
                m[1][3] + m[1][row] * sign,
                m[2][3] + m[2][row] * sign,
            );
            let distance = m[3][3] + m[3][row] * sign;
            let inv_len = 1.0 / normal.magnitude();

            planes[i] = Plane {
                normal: normal * inv_len,
                distance: distance * inv_len,
            }
        }

        Self { planes }
    }

    pub fn intersects_aabb(&self, aabb: (Point3<f32>, Point3<f32>)) -> bool {
        let (min, max) = aabb;
        for plane in &self.planes {
            let px = if plane.normal.x >= 0.0 { max.x } else { min.x };
            let py = if plane.normal.y >= 0.0 { max.y } else { min.y };
            let pz = if plane.normal.z >= 0.0 { max.z } else { min.z };

            if plane.normal.x * px + plane.normal.y * py + plane.normal.z * pz + plane.distance
                <= 0.0
            {
                return false;
            }
        }
        true
    }
}
