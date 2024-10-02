/// This is a signal that does xyz
/// Params are
#[pyclass()]
#[derive(Debug, Clone)]
pub struct Signal {
    pub index: i32,
}

#[pymethods]
impl Signal {
    /// This is a signal that does xyz
    #[new]
    #[pyo3(text_signature = "(index)")]
    fn new(index: i32) -> Self {
        Signal { index }
    }

    /// the self argument should be written $self
    // #[pyo3(text_signature = "($self, e, f)")]
    #[pyo3(signature = (e, f))]
    fn my_method(&self, e: i32, f: i32) -> i32 {
        e + f
    }

    /// MEthod that does this. storm is here
    // #[pyo3(signature = (test= Signal{index: 1}, num=10))]
    // #[pyo3(text_signature = "(num=10, test:Signal)")]
    fn method(&mut self, test: Signal, num: i32) {
        self.index = num;
    }
}
