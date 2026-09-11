use glium;
use obj::*;
use crate::vector::*;
use crate::drawable::*;
use std::num;
use std::any::Any;
use uuid::Uuid;

pub trait Collidable<V, I> where V: glium::vertex::Vertex, I: glium::index::Index {
    fn get_position(&self) -> Vector { new_vector(&[0.0, 0.0, 0.0]) } 
    fn get_size_offset(&self) -> Vector { new_vector(&[0.0, 0.0, 0.0]) }
    fn get_size(&self) -> Vector { new_vector(&[0.0, 0.0, 0.0]) }
    fn recalculate_size(&mut self) {}
    fn get_anchored(&self) -> bool { false }
    fn set_pos(&mut self, pos: Vector) {}
    fn get_id(&self) -> Uuid { Uuid::new_v4() }
    fn on_recalculate_size<'a>(&self, draw_info: &DrawInfo<V, I>, vertices: &'a Vec<TexturedVertex>) -> Vector {
        let mut min = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let mut max = Vector {
            x: 0.0, 
            y: 0.0, 
            z: 0.0,
        };

        for v in vertices {
            let new_position = apply_matrix(v.position, draw_info.get_matrix());

            if new_position[0] < min.x {
                min.x = new_position[0];
            } else if new_position[0] > max.x {
                max.x = new_position[0];
            }

            if new_position[1] < min.y {
                min.y = new_position[1];
            } else if new_position[1] > max.y {
                max.y = new_position[1];
            }

            if new_position[2] < min.z {
                min.z = new_position[2];
            } else if new_position[2] > max.z {
                max.z = new_position[2];
            }
        }

        /*if min.x < 0.0 { size_offset.x = min.x }
        if min.y < 0.0 { size_offset.y = min.y }
        if min.z < 0.0 { size_offset.z = min.z }*/

        max.add(
            &Vector {
                x: -min.x,
                y: -min.y,
                z: -min.z,
            }
        );
        max
    }
}

pub fn resolve_collision(a_pos: Vector, a_size: Vector, b_pos: Vector, b_size: Vector) -> (Vector, Vector) {
    let mut a_increment = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut b_increment = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    if a_pos.x <= b_pos.x + b_size.x &&
       a_pos.x + a_size.x >= b_pos.x &&
       a_pos.y <= b_pos.y + b_size.y &&
       a_pos.y + a_size.y >= b_pos.y &&
       a_pos.z <= b_pos.z + b_size.z &&
       a_pos.z + a_size.z >= b_pos.z {
        if (a_pos.x - b_pos.x).abs() < (a_pos.y - b_pos.y).abs() &&
            (a_pos.x - b_pos.x).abs() < (a_pos.z - b_pos.z).abs() {
            if a_pos.x > b_pos.x {
                a_increment = new_vector(&[b_size.x - (a_pos.x - b_pos.x), 0.0, 0.0]);
            } else {
                b_increment = new_vector(&[a_size.x - (b_pos.x - a_pos.x), 0.0, 0.0]);
            }
        } else if (a_pos.y - b_pos.y).abs() < (a_pos.x - b_pos.x).abs() &&
                   (a_pos.y - b_pos.y).abs() < (a_pos.z - b_pos.z).abs() {
            if a_pos.y > b_pos.y {
                a_increment = new_vector(&[0.0, b_size.y - (a_pos.y - b_pos.y), 0.0]);
            } else {
                b_increment = new_vector(&[0.0, a_size.y - (b_pos.y - a_pos.y), 0.0]);
            }
        } else {
            if a_pos.z > b_pos.z {
                a_increment = new_vector(&[0.0, 0.0, b_size.z - (a_pos.z - b_pos.z)]);
            } else {
                b_increment = new_vector(&[0.0, 0.0, a_size.z - (b_pos.z - a_pos.z)]);
            }
        }
    }

    return (a_increment, b_increment);
}
