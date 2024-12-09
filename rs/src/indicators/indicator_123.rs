use numpy::PyReadonlyArray1;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
pub fn indicator_123<'py>(
    py: Python<'py>,
    window: usize,
    highs: PyReadonlyArray1<f64>,
    lows: PyReadonlyArray1<f64>,
    maxs: PyReadonlyArray1<f64>,
    mins: PyReadonlyArray1<f64>,
) -> PyResult<Py<PyAny>> {
    let length = highs.len().unwrap();
    let mut point_1 = vec![f64::NAN; length];
    let mut point_2 = vec![f64::NAN; length];
    let mut point_3 = vec![f64::NAN; length];

    let mut direction = -1;
    let mut trigger_1: f64 = 0.0;
    let mut trigger_2: f64 = 0.0;
    let mut trigger_3: f64 = 0.0;

    let mut state = 0;

    for i in window..length - 1 {
        // Detect Point 1 (Swing High or Swing Low)
        let high = *highs.get(i).unwrap();
        let low = *lows.get(i).unwrap();
        let prev_high = *highs.get(i - 1).unwrap();
        let prev_low = *lows.get(i - 1).unwrap();
        let max = *maxs.get(i).unwrap();
        let min = *mins.get(i).unwrap();

        if state == 0 {
            if high == max {
                direction = 1;
                trigger_1 = high;
                state = 1;
            } else if low == min {
                direction = 2;
                trigger_1 = low;
                state = 1;
            }
        } else if state == 1 {
            // We are working with green candles
            if direction == 1 {
                // // Uptrend retracement
                if high > trigger_1 || low > prev_low {
                    trigger_1 = high;
                } else if low < prev_low {
                    trigger_2 = low;
                    state = 2;
                } else {
                    state = 0;
                }
            } else if direction == 2 {
                // Downtrend retracement
                if low < trigger_1 || high < prev_high {
                    trigger_1 = low;
                } else if high > prev_high {
                    trigger_2 = high;
                    state = 2;
                } else {
                    state = 0;
                }
            }
        } else if state == 2 {
            // We are working with Red candles
            if direction == 1 {
                // Uptrend retracement
                if high > trigger_1 {
                    state = 0;
                } else if high > prev_high && low > prev_low {
                    state = 3;
                    trigger_3 = high;
                } else {
                    trigger_2 = trigger_2.min(low);
                }
            } else if direction == 2 {
                // Downtrend retracement
                if low < trigger_1 {
                    state = 0;
                } else if low < prev_low && high < prev_high {
                    state = 3;
                    trigger_3 = low;
                } else {
                    trigger_2 = trigger_2.max(high);
                }
            }
        } else if state == 3 {
            // We are working with Blue candles
            if direction == 1 {
                // Uptrend retracement
                if high > trigger_1 || low < trigger_2 {
                    state = 0;
                    trigger_1 = 0.0;
                    trigger_2 = 0.0;
                } else {
                    trigger_3 = high;
                }
            } else if direction == 2 {
                // Downtrend retracement
                if low < trigger_1 || high > trigger_2 {
                    state = 0;
                    trigger_1 = 0.0;
                    trigger_2 = 0.0;
                } else {
                    trigger_3 = low;
                }
            }
        }

        if state > 0 && trigger_1 != 0.0 {
            point_1[i] = trigger_1;
        }
        if state > 1 && trigger_2 != 0.0 {
            point_2[i] = trigger_2;
        }
        if state > 2 && trigger_3 != 0.0 {
            point_3[i] = trigger_3;
        }
    }

    // // Create a dictionary to return
    let dict = PyDict::new_bound(py);

    // Insert key-value pairs
    dict.set_item("point_1", point_1)?;
    dict.set_item("point_2", point_2)?;
    dict.set_item("point_3", point_3)?;

    Ok(dict.into_py(py)) // Return the dictionary as a Python object
}
