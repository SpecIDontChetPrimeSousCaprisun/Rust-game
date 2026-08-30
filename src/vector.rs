pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn new_vector(values: &[f32; 3]) -> Vector {
   return Vector {
        x: values[0],
        y: values[1],
        z: values[2],
   };
}

impl Vector {
    pub fn clone(target: &Vector) -> Vector {
        return Vector {
            x: target.x,
            y: target.y,
            z: target.z,
        };
    }

    pub fn replace(&mut self, target: &Vector) {
        self.x = target.x;
        self.y = target.y;
        self.z = target.z;
    }
    
    pub fn get_view_matrix(position: &Vector, direction: &Vector, up: &Vector) -> [[f32; 4]; 4] {
        view_matrix(
            position.as_3d_array(),
            direction.as_3d_array(),
            up.as_3d_array(),
        )
    }

    pub fn add(&mut self, b: &Vector) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }

    pub fn mult(&mut self, b: &Vector) {
        self.x *= b.x;
        self.y *= b.y;
        self.z *= b.z;
    }

    pub fn as_3d_array(&self) -> [f32; 3] {
        [
            self.x,
            self.y,
            self.z,
        ]
    }

    pub fn as_2d_array(&self) -> [f32; 2] {
        [
            self.x,
            self.y,
        ]
    }

    pub fn cross_product(&self, b: &Vector) -> Vector {
        new_vector(&[
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.x * b.x
        ])
    }

    pub fn normal(&self) -> Vector {
        let len = self.x * self.x + self.y * self.y + self.z * self.z;
        let len = len.sqrt();
        new_vector(
            &[
                self.x / len,
                self.y / len,
                self.z / len,
            ]
        )
    }
}

fn view_matrix(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> [[f32; 4]; 4] {
    let f = new_vector(&direction).normal().as_3d_array();

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
