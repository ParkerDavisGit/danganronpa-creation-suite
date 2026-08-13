use pyo3::prelude::*;


#[pyfunction]
fn gello() -> String {
    "Gello Gorld".to_string()
}

fn gorld(direction: i32) -> String {
    if direction > 0i32 {
        "Big".to_string()
    }
    else {
        "Small".to_string()
    }
}

#[pymodule]
fn danganronpa_creation_suite(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gello, m)?)?;

    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gorld_test() {
        assert_eq!(gello(), "Gello Gorld".to_string());
    }
}