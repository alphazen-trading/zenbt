use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PySeries;

#[pyfunction]
pub fn cross_below(arr1: PySeries, arr2: PySeries) -> PyResult<PySeries> {
    let arr1 = arr1.0;
    let arr2 = arr2.0;
    let mut mask = vec![false; arr1.len()];

    for i in 1..arr1.len() {
        if arr1.get(i).unwrap() < arr2.get(i).unwrap()
            && arr1.get(i - 1).unwrap() > arr2.get(i - 1).unwrap()
        {
            mask[i] = true;
        }
    }

    // Convert mask_values to a Polars Series
    let mask_series = Series::new("mask".into(), mask);

    // Wrap as a PySeries and return it
    Ok(PySeries(mask_series))
}
