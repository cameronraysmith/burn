// Import the shared macro
use crate::include_models;
include_models!(sin);

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Tensor, TensorData, Tolerance, ops::FloatElem};

    type Backend = burn_ndarray::NdArray<f32>;
    type FT = FloatElem<Backend>;

    #[test]
    fn sin() {
        let device = Default::default();
        let model: sin::Model<Backend> = sin::Model::new(&device);

        let input = Tensor::<Backend, 4>::from_floats([[[[1.0, 4.0, 9.0, 25.0]]]], &device);

        let output = model.forward(input);
        let expected = TensorData::from([[[[0.8415f32, -0.7568, 0.4121, -0.1324]]]]);

        output
            .to_data()
            .assert_approx_eq::<FT>(&expected, Tolerance::rel_abs(1e-4, 1e-4));
    }
}
