use pyo3::prelude::*;
use pyo3::types::{PyList, PyAny, PyInt, PyFloat, PyString};
use rayon::prelude::*;
use pyo3::IntoPyObjectExt;

// ==================== ОБЩАЯ СТРУКТУРА ====================

#[derive(Clone)]
struct RustyArray<T> {
    data: Vec<T>,
}

// ==================== ТРЕЙТЫ ====================

trait ArrayOps: Sized {
    type Item: Clone + std::fmt::Debug;
    fn new(data: Vec<Self::Item>) -> Self;
    fn data(&self) -> Vec<Self::Item>;
    fn to_list(&self) -> Vec<Self::Item>;
    fn __repr__(&self) -> String;
}

trait NumericSum {
    type Output;
    fn sum(&self) -> Self::Output;
}

// ==================== ИМПЛЕМЕНТАЦИИ ====================

impl<T: Clone + std::fmt::Debug> ArrayOps for RustyArray<T> {
    type Item = T;
    fn new(data: Vec<T>) -> Self {
        RustyArray { data }
    }

    fn data(&self) -> Vec<T> { self.data.clone() }
    fn to_list(&self) -> Vec<T> { self.data.clone() }
    fn __repr__(&self) -> String {
        format!("RustyArray({:?})", self.data)
    }
}

// Сумма для float
impl NumericSum for RustyArray<f64> {
    type Output = f64;
    fn sum(&self) -> f64 {
        if self.data.is_empty() { return 0.0; }
        let num_chunks = rayon::current_num_threads() * 4;
        let chunk_size = (self.data.len() / num_chunks).max(256);

        self.data.par_chunks(chunk_size)
            .map(|chunk| {
                let mut sum = 0.0f64;
                let mut i = 0;
                while i + 4 <= chunk.len() {
                    sum += chunk[i] + chunk[i+1] + chunk[i+2] + chunk[i+3];
                    i += 4;
                }
                while i < chunk.len() {
                    sum += chunk[i];
                    i += 1;
                }
                sum
            })
            .sum()
    }
}

// Сумма для int
impl NumericSum for RustyArray<i128> {
    type Output = i128;
    fn sum(&self) -> i128 {
        if self.data.is_empty() { return 0; }
        let num_chunks = rayon::current_num_threads() * 4;
        let chunk_size = (self.data.len() / num_chunks).max(256);

        self.data.par_chunks(chunk_size)
            .map(|chunk| {
                let mut sum = 0i128;
                let mut i = 0;
                while i + 4 <= chunk.len() {
                    sum += chunk[i] + chunk[i+1] + chunk[i+2] + chunk[i+3];
                    i += 4;
                }
                while i < chunk.len() {
                    sum += chunk[i];
                    i += 1;
                }
                sum
            })
            .sum()
    }
}

// ==================== PYO3 КЛАССЫ ====================

#[pyclass(name = "FloatRustyArray", module = "rusty_math")]
struct PyRustyFloatArray(RustyArray<f64>);

#[pyclass(name = "IntRustyArray", module = "rusty_math")]
struct PyRustyIntArray(RustyArray<i128>);

#[pyclass(name = "StringRustyArray", module = "rusty_math")]
struct PyRustyStringArray(RustyArray<String>);

// ==================== PYMETHODS ====================

#[pymethods]
impl PyRustyFloatArray {
    fn __repr__(&self) -> String { self.0.__repr__() }
    fn to_list(&self) -> Vec<f64> { self.0.to_list() }
    #[getter] fn data(&self) -> Vec<f64> { self.0.data() }
    fn sum_farray(&self) -> f64 { self.0.sum() }
}

#[pymethods]
impl PyRustyIntArray {
    fn __repr__(&self) -> String { self.0.__repr__() }
    fn to_list(&self) -> Vec<i128> { self.0.to_list() }
    #[getter] fn data(&self) -> Vec<i128> { self.0.data() }
    fn sum_array(&self) -> i128 { self.0.sum() }
}

#[pymethods]
impl PyRustyStringArray {
    fn __repr__(&self) -> String { self.0.__repr__() }
    fn to_list(&self) -> Vec<String> { self.0.to_list() }
    #[getter] fn data(&self) -> Vec<String> { self.0.data() }
}

// ==================== ФУНКЦИИ ====================

#[pyfunction]
fn array<'py>(py: Python<'py>, data: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    // Если передали список
    if let Ok(list) = data.cast::<PyList>() {
        if list.is_empty() {
            // По умолчанию возвращаем float массив
            return PyRustyFloatArray(RustyArray::new(vec![])).into_bound_py_any(py);
        }

        // Проверяем, есть ли в массиве строки (имеют приоритет)
        let has_string = list.iter().any(|item| {
            item.is_instance_of::<PyString>()
        });

        // Если есть строки - конвертируем все элементы в строки
        if has_string {
            let string_list: Vec<String> = list.iter()
                .map(|item| {
                    item.str()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|_| item.repr()
                            .map(|r| r.to_string_lossy().into_owned())
                            .unwrap_or_else(|_| "".to_string()))
                })
                .collect();
            return PyRustyStringArray(RustyArray::new(string_list)).into_bound_py_any(py);
        }

        // Берём первый элемент для определения типа
        let first = list.get_item(0)?;

        // Проверяем, является ли первый элемент float
        if first.is_instance_of::<PyFloat>() {
            // Если float - конвертируем всё в f64
            if let Ok(v) = list.extract::<Vec<f64>>() {
                return PyRustyFloatArray(RustyArray::new(v)).into_bound_py_any(py);
            }
        }

        // Проверяем, является ли первый элемент int
        if first.is_instance_of::<PyInt>() {
            // Пробуем конвертировать в целые числа (i128)
            if let Ok(v) = list.extract::<Vec<i128>>() {
                return PyRustyIntArray(RustyArray::new(v)).into_bound_py_any(py);
            }
            // Если не получилось i128, пробуем f64 (на случай смешанных типов)
            if let Ok(v) = list.extract::<Vec<f64>>() {
                return PyRustyFloatArray(RustyArray::new(v)).into_bound_py_any(py);
            }
        }

        // Если не получилось — пробуем строки
        if let Ok(v) = list.extract::<Vec<String>>() {
            return PyRustyStringArray(RustyArray::new(v)).into_bound_py_any(py);
        }
    }

    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
        "Unsupported type. Expected list of int, float or str."
    ))
}

#[pyfunction]
fn fsum(arr: &PyRustyFloatArray) -> f64 {
    arr.sum_farray()
}

#[pyfunction]
fn sum(arr: &PyRustyIntArray) -> i128 {
    arr.sum_array()
}

// ==================== МОДУЛЬ ====================

#[pymodule]
fn rusty_math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRustyFloatArray>()?;
    m.add_class::<PyRustyIntArray>()?;
    m.add_class::<PyRustyStringArray>()?;

    m.add_function(wrap_pyfunction!(array, m)?)?;
    m.add_function(wrap_pyfunction!(fsum, m)?)?;
    m.add_function(wrap_pyfunction!(sum, m)?)?;

    Ok(())
}