extern crate nalgebra as na;

mod bessel;
mod focal_point;
pub mod multiple_foci;
mod traits;

pub use acoustic_field_calculator::prelude::*;

pub use bessel::BesselBeam;
pub use focal_point::FocalPoint;
pub use traits::Optimizer;

use na::{ComplexField, Dynamic, Matrix, Normed, VecStorage, U1};

type MatrixXcf = Matrix<Complex, Dynamic, Dynamic, VecStorage<Complex, Dynamic, Dynamic>>;
type MatrixXf = Matrix<Float, Dynamic, Dynamic, VecStorage<Float, Dynamic, Dynamic>>;
type VectorXcf = Matrix<Complex, Dynamic, U1, VecStorage<Complex, Dynamic, U1>>;
type VectorXf = Matrix<Float, Dynamic, U1, VecStorage<Float, Dynamic, U1>>;
