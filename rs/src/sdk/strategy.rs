use pyo3::{prelude::*, types::PyType};

// use super::ohlc::OHLC;
use chrono::{DateTime, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[pyclass(get_all, set_all)]
#[derive(Debug)]
pub struct Test {
    pub mine: Decimal,
}

#[pyclass(get_all, frozen)]
#[derive(Debug)]
pub struct Foo {
    pub inner: Decimal,
    pub test: Py<Test>,
}

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Bar {
    pub foo: Py<Foo>,
}

#[pymethods]
impl Bar {
    #[new]
    fn new() -> PyResult<Bar> {
        Python::with_gil(|py| {
            let foo: Py<Foo> = Py::new(
                py,
                Foo {
                    inner: Decimal::from_f64(24.1).unwrap(),
                    test: Py::new(
                        py,
                        Test {
                            mine: Decimal::from_f64(24.1).unwrap(),
                        },
                    )?,
                },
            )?;
            Ok(Bar { foo })
        })
    }

    fn test(&self) {
        let val = self.foo.get().inner;
        println!("Za val is this {:?}", val);
        // Python::with_gil(|py| {
        //     let val = self.foo.borrow(py);
        //     println!("Za val is this {:?}", val);
        // });
    }
}

#[pyclass(get_all)]
#[derive(Debug)]
pub struct OHLC {
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[pyclass(get_all, subclass)]
#[derive(Debug)]
pub struct Strategy {
    inner: Py<OHLC>,
    val: Decimal,
}

#[pymethods]
impl Strategy {
    #[new]
    fn new() -> PyResult<Strategy> {
        Python::with_gil(|py| {
            let ohlc: Py<OHLC> = Py::new(
                py,
                OHLC {
                    // date: DateTime::from_timestamp(241012314 / 1000, 0).expect("Invalid timestamp"),
                    open: Decimal::from_f64(24.1).unwrap(),
                    high: Decimal::from_f64(24.1).unwrap(),
                    low: Decimal::from_f64(24.1).unwrap(),
                    close: Decimal::from_f64(24.1).unwrap(),
                    volume: Decimal::from_f64(24.1).unwrap(),
                },
            )?;
            Ok(Strategy {
                inner: ohlc,
                val: Decimal::from_f64(24.1).unwrap(),
            })
        })
    }

    pub fn test(&self) {
        println!("Za val is this {:?}", self.val);
    }

    #[classmethod]
    pub fn major(cls: &Bound<'_, PyType>) -> PyResult<i32> {
        Ok(10)
    }
}

#[pyclass(get_all)]
#[derive(Debug)]
pub struct BT {
    pub strategy: Py<Strategy>,
}

#[pymethods]
impl BT {
    #[new]
    fn new(strategy: Py<Strategy>) -> PyResult<BT> {
        Ok(BT { strategy })
    }

    fn test(&self) {
        Python::with_gil(|py| {
            // Access the Python class object of Strategy
            self.strategy
                .call_method_bound(py, "major", (), None)
                .unwrap();
            let val = self.strategy.borrow(py).val;
            println!("Za val is this {:?}", val);
        });
    }
}
