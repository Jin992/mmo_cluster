use crate::game::char::vector_xyz::VectorXYZ;
#[derive(Default)]
pub struct Motion {
    position: VectorXYZ,
    rotation: VectorXYZ,
    velocity: VectorXYZ,
}