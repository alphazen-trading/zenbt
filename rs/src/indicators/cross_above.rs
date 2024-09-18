use pyo3::prelude::*;

#[pyfunction]
pub fn cross_above(arr1: Vec<f64>, arr2: Vec<f64>) -> Vec<bool> {
    let mut cross_above_mask = vec![false; arr1.len()];

    for i in 1..arr1.len() {
        if arr1[i] > arr2[i] && arr1[i - 1] < arr2[i - 1] {
            cross_above_mask[i] = true;
        }
    }
    cross_above_mask
}
