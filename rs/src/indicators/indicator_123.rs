use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_polars::PySeries;

#[pyfunction]
pub fn indicator_123<'py>(
    py: Python<'py>,
    window: usize,
    highs: PyReadonlyArray1<f64>,
    lows: PyReadonlyArray1<f64>,
    max: PyReadonlyArray1<f64>,
    min: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyAny>> {
    let length = highs.len().unwrap();
    let mut point1_indices = vec![false; length];
    let mut point2_indices = vec![false; length];
    let mut point3_indices = vec![false; length];
    let mut order_values = vec![f64::NAN; length];

    let mut direction = -1;
    let mut trigger_1: &f64 = &0.0;
    let mut trigger_2: &f64 = &0.0;
    let mut trigger_3: &f64 = &0.0;

    let mut state = 0;

    for i in window..length - 1 {
        // Detect Point 1 (Swing High or Swing Low)
        let high = highs.get(i).unwrap();
        let low = lows.get(i).unwrap();
        let prev_high = highs.get(i - 1).unwrap();
        let prev_low = lows.get(i - 1).unwrap();
        // println!("{i}: {state}");

        if state == 0 {
            if high == max.get(i).unwrap() {
                direction = 1;
                trigger_1 = high;
                point1_indices[i] = true;
                state = 1;
            } else if low == min.get(i).unwrap() {
                direction = 2;
                trigger_1 = low;
                point1_indices[i] = true;
                state = 1;
            }
        } else if state == 1 {
            // We are working with green candles
            if direction == 1 {
                // // Uptrend retracement
                if high > trigger_1 {
                    trigger_1 = high;
                    point1_indices[i] = true;
                } else if low < prev_low {
                    trigger_2 = low;
                    point2_indices[i] = true;
                    state = 2;
                } else {
                    state = 0;
                }
            } else if direction == 2 {
                // Downtrend retracement
                if low < trigger_1 {
                    trigger_1 = low;
                    point1_indices[i] = true;
                } else if high > prev_high {
                    trigger_2 = high;
                    point2_indices[i] = true;
                    state = 2;
                } else {
                    state = 0;
                }
            }
        } else if state == 2 {
            // We are working with Red candles
            if direction == 1 {
                // // Uptrend retracement
                if high > trigger_1 {
                    state = 0;
                } else if high > prev_high {
                    state = 3;
                    point3_indices[i] = true;
                } else {
                    trigger_2 = low;
                    point2_indices[i] = true;
                }
            } else if direction == 2 {
                // Downtrend retracement
                if low < trigger_1 {
                    state = 0;
                } else if low < prev_low {
                    state = 3;
                    point3_indices[i] = true;
                } else {
                    trigger_2 = high;
                    point2_indices[i] = true;
                }
            }
        } else if state == 3 {
            // We are working with Blue candles
            if direction == 1 {
                // Uptrend retracement
                if high > trigger_2 || low < trigger_2 {
                    state = 0;
                } else {
                    point3_indices[i] = true;
                }
            } else if direction == 2 {
                // Downtrend retracement
                if low < trigger_1 || high > trigger_2 {
                    state = 0;
                } else {
                    point3_indices[i] = true;
                }
            }
        }
    }

    // // Create a dictionary to return
    let dict = PyDict::new_bound(py);

    // Insert key-value pairs
    dict.set_item("point_1", point1_indices)?;
    dict.set_item("point_2", point2_indices)?;
    dict.set_item("point_3", point3_indices)?;
    dict.set_item("order_values", order_values)?;

    Ok(dict.into_py(py)) // Return the dictionary as a Python object
}
