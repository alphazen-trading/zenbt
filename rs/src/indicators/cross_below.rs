use numpy::{PyArrayDyn, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::Bound;

#[pyfunction]
pub fn cross_below<'py>(
    arr1: &Bound<'py, PyArrayDyn<f64>>,
    arr2: &Bound<'py, PyArrayDyn<f64>>,
    mask: &Bound<'py, PyArrayDyn<bool>>,
) {
    let mut mask = unsafe { mask.as_array_mut() };
    let arr1 = unsafe { arr1.as_array_mut() };
    let arr2 = unsafe { arr2.as_array_mut() };

    for i in 1..arr1.len() {
        if arr1[i] < arr2[i] && arr1[i - 1] > arr2[i - 1] {
            mask[i] = true;
        }
    }
    mask;
}
