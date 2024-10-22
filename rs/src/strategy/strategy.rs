use crate::sdk::position::Positions;
use numpy::ToPyArray;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::types::PyList;
use pyo3::types::PyType;
use pyo3_polars::PyDataFrame;

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Strategy {
    pub df: PyDataFrame,
    pub data: Py<PyDict>,
    pub equity: Py<PyList>,
    pub floating_equity: Py<PyList>,
    pub positions: Positions,
}

#[pymethods]
impl Strategy {
    #[new]
    fn new(df: PyDataFrame) -> PyResult<Strategy> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            let col_names = df.0.get_column_names();
            for (i, col) in df.0.get_columns().iter().enumerate() {
                if col.dtype() == &DataType::Float64 {
                    let col_values: Vec<f64> = col
                        .f64()
                        .unwrap()
                        .into_iter()
                        .collect::<Option<Vec<f64>>>()
                        .unwrap();
                    dict.set_item(col_names[i].to_string(), col_values.to_pyarray_bound(py))?;
                } else if col.dtype() == &DataType::Int64 {
                    let col_values: Vec<i64> = col
                        .i64()
                        .unwrap()
                        .into_iter()
                        .collect::<Option<Vec<i64>>>()
                        .unwrap();
                    dict.set_item(col_names[i].to_string(), col_values.to_pyarray_bound(py))?;
                } else if col.dtype() == &DataType::Boolean {
                    let col_values: Vec<bool> = col
                        .bool()
                        .unwrap()
                        .into_iter()
                        .collect::<Option<Vec<bool>>>()
                        .unwrap();
                    dict.set_item(col_names[i].to_string(), col_values.to_pyarray_bound(py))?;
                } else {
                    // Handle unsupported data types or skip
                    eprintln!(
                        "Unsupported column type for column: {}, {:?}",
                        col_names[i],
                        col.dtype()
                    );
                    continue;
                }
            }

            Ok(Strategy {
                df,
                data: dict.into(),
                equity: PyList::new_bound(py, vec![0]).into(),
                floating_equity: PyList::new_bound(py, vec![0]).into(),
                positions: Positions::new(),
            })
        })
    }

    #[classmethod]
    #[allow(unused_variables)]
    pub fn on_candle(cls: &Bound<'_, PyType>) -> PyResult<i32> {
        Ok(10)
    }

    #[classmethod]
    #[allow(unused_variables)]
    pub fn _on_candle(cls: &Bound<'_, PyType>) -> PyResult<i32> {
        Ok(10)
    }
}
