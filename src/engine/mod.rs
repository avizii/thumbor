mod photon;
pub use photon::*;

use crate::pb::Spec;
use image::ImageOutputFormat;

pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);

    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransForm<T> {
    fn transform(&mut self, op: T);
}
