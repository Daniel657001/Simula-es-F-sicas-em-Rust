use crate::particula::Particula;

pub fn forca_gravitacional(p1: &Particula, p2: &Particula, r: f64) -> f64{
    let forca_gravity = (6.67e-11 * p1.massa * p2.massa) / r.powi(2);
    forca_gravity
}