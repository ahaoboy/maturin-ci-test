use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod ci_test {
    use pyo3::prelude::*;
    use rquickjs_core::{Context,   Result, Runtime };

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        let rt = Runtime::new().unwrap();
        rt.set_memory_limit(u32::MAX as usize);
        rt.set_max_stack_size(u32::MAX as usize);
        let ctx = Context::full(&rt).unwrap();
        ctx.with(|ctx| -> Result<()> {
            ctx.eval("1+1")?
        }).unwrap();
        Ok((a + b).to_string())
    }
}
