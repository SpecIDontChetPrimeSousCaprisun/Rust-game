use crate::vector::*;
use std::num;

pub trait Collidable {
    fn get_position(&self) -> Vector { new_vector(&[0.0, 0.0, 0.0]) }
    fn get_size(&self) -> Vector { new_vector(&[0.0, 0.0, 0.0]) }
    fn get_anchored(&self) -> bool { false }
    fn set_pos(&mut self, pos: Vector) {}
}

pub fn resolve_collision(a: &mut impl Collidable, b: &mut impl Collidable) {
    if a.get_anchored() && b.get_anchored() { return; }

    let a_pos = a.get_position();
    let a_size = a.get_size();

    let b_pos = b.get_position();
    let b_size = b.get_size();

    if a_pos.x <= b_pos.x + b_size.x &&
       a_pos.x + a_size.x >= b_pos.x &&
       a_pos.y <= b_pos.y + b_size.y &&
       a_pos.y + a_size.y >= b_pos.y &&
       a_pos.z <= b_pos.z + b_size.z &&
       a_pos.z + a_size.z >= b_pos.z {
        if (a_pos.x - b_pos.x).abs() < (a_pos.y - b_pos.y).abs() &&
            (a_pos.x - b_pos.x).abs() < (a_pos.z - b_pos.z).abs() {
            if a_pos.x > b_pos.x {
                a.set_pos(new_vector(&[b_size.x - (a_pos.x - b_pos.x), 0.0, 0.0]));
            } else {
                b.set_pos(new_vector(&[a_size.x - (b_pos.x - a_pos.x), 0.0, 0.0]));
            }
        } else if (a_pos.y - b_pos.y).abs() < (a_pos.x - b_pos.x).abs() &&
                   (a_pos.y - b_pos.y).abs() < (a_pos.z - b_pos.z).abs() {
            if a_pos.y > b_pos.y {
                a.set_pos(new_vector(&[b_size.y - (a_pos.y - b_pos.y), 0.0, 0.0]));
            } else {
                b.set_pos(new_vector(&[a_size.y - (b_pos.y - a_pos.y), 0.0, 0.0]));
            }
        } else {
            if a_pos.z > b_pos.z {
                a.set_pos(new_vector(&[b_size.z - (a_pos.z - b_pos.z), 0.0, 0.0]));
            } else {
                b.set_pos(new_vector(&[a_size.z - (b_pos.z - a_pos.z), 0.0, 0.0]));
            }
        }
    }
}
