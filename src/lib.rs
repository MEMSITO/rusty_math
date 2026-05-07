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
trait Appendable {
    type Item;
    fn append(&mut self, item: Self::Item);
    fn extend(&mut self, items: Vec<Self::Item>);
}
trait NumericSum {
    type Output;
    fn sum(&self) -> Self::Output;
}
trait ArrayElementsOperations: Sized {
    type Output;
    fn add(&self, other: &Self) -> PyResult<Self::Output>;
    fn sub(&self, other: &Self) -> PyResult<Self::Output>;
    fn mul(&self, other: &Self) -> PyResult<Self::Output>;
    fn div(&self, other: &Self) -> PyResult<Self::Output>;
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

impl<T: Clone> Appendable for RustyArray<T> {
    type Item = T;
    fn append(&mut self, item: T) {
        self.data.push(item);
    }
    fn extend(&mut self, items: Vec<T>) {
        self.data.extend(items);
    }
}

// Сумма для float
impl NumericSum for RustyArray<f64> {
    type Output = f64;
    fn sum(&self) -> f64 {
        if self.data.is_empty() { return 0.0; }
        let num_chunks = rayon::current_num_threads() * 4;
        let chunk_size = (self.data.len() / num_chunks).max(1024);

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

impl ArrayElementsOperations for RustyArray<f64> {
    type Output = RustyArray<f64>;
    fn add(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<f64> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Ok(RustyArray::new(result))
    }
    fn sub(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<f64> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a - b)
            .collect();

        Ok(RustyArray::new(result))
    }
    fn mul(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<f64> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a * b)
            .collect();

        Ok(RustyArray::new(result))
    }
    fn div(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<f64> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a / b)
            .collect();

        Ok(RustyArray::new(result))
    }
}

// Сумма для int
impl NumericSum for RustyArray<i128> {
    type Output = i128;
    fn sum(&self) -> i128 {
        if self.data.is_empty() { return 0; }
        let num_chunks = rayon::current_num_threads() * 4;
        let chunk_size = (self.data.len() / num_chunks).max(1024);

        self.data.par_chunks(chunk_size)
            .map(|chunk| {
                let mut sum = 0i128;
                let mut i = 0;
                while i + 4 <= chunk.len() {
                    sum += chunk[i] + chunk[i+1] + chunk[i+2] + chunk[i+3] ;
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

impl ArrayElementsOperations for RustyArray<i128> {
    type Output = RustyArray<i128>;
    fn add(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<i128> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a + b)
            .collect();

        Ok(RustyArray::new(result))
    }
    fn sub(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<i128> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a - b)
            .collect();

        Ok(RustyArray::new(result))
    }
    fn mul(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<i128> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a * b)
            .collect();

        Ok(RustyArray::new(result))
    }
    fn div(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}", self.data.len(), other.data.len())
            ));
        }

        let result: Vec<i128> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(&a, &b)| a / b)
            .collect();

        Ok(RustyArray::new(result))
    }
}

impl ArrayElementsOperations for RustyArray<String> {
    type Output = RustyArray<String>;
    fn add(&self, other: &Self) -> PyResult<Self::Output> {
        if self.data.len() != other.data.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Arrays must have the same length: {} != {}",
                        self.data.len(), other.data.len())
            ));
        }

        let result: Vec<String> = self.data.par_iter()
            .zip(other.data.par_iter())
            .map(|(a, b)| format!("{}{}", a, b))   // поэлементная конкатенация
            .collect();

        Ok(RustyArray::new(result))
    }
    fn sub(&self, _other: &Self) -> PyResult<Self::Output> {
        Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
            "Unsupported action. Strings cannot be subtracted from each other."
        ))
    }
    fn mul(&self, _other: &Self) -> PyResult<Self::Output> {
        Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
            "Unsupported action. Strings cannot be multiplied from each other."
        ))
    }
    fn div(&self, _other: &Self) -> PyResult<Self::Output> {
        Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
            "Unsupported action. Strings cannot be divided from each other."
        ))
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
    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| x + scalar)
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.add(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only add FloatRustyArray + FloatRustyArray"
        ))
    }
    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        self.__add__(other)
    }
    fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // 1. Вычитание скаляра: arr - scalar
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| x - scalar)  // ✅ вычитание
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // 2. Вычитание массива: arr - other_arr
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.sub(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only subtract scalar or FloatRustyArray from FloatRustyArray"
        ))
    }

    fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // 1. Вычитание из скаляра: scalar - arr
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| scalar - x)  // ✅ обратный порядок: scalar - x
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // 2. Вычитание из массива: other_arr - self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = other_array.sub(&self.0)?;  // ✅ other - self
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only subtract scalar or FloatRustyArray from FloatRustyArray"
        ))
    }
    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // 1. Сначала проверяем скаляр
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| x * scalar)
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // 2. Потом проверяем другой массив
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.mul(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only multiply FloatRustyArray by scalar (float) or another FloatRustyArray"
        ))
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // Для скаляра умножение коммутативно: scalar * arr == arr * scalar
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| x * scalar)  // или scalar * x, результат тот же
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // Для массива: other_array * self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = other_array.mul(&self.0)?;  // other * self
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only multiply FloatRustyArray by scalar (float) or another FloatRustyArray"
        ))
    }
    fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // 1. Деление на скаляр: arr / scalar
        if let Ok(scalar) = other.extract::<f64>() {
            if scalar == 0.0 {
                return Err(PyErr::new::<pyo3::exceptions::PyZeroDivisionError, _>(
                    "division by zero"
                ));
            }
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| x / scalar)
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // 2. Деление на массив: arr / other_arr
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.div(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only divide FloatRustyArray by scalar (float) or another FloatRustyArray"
        ))
    }

    fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // 1. Деление скаляра на массив: scalar / arr
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| {
                    if x == 0.0 {
                        f64::INFINITY  // или можете вернуть ошибку
                    } else {
                        scalar / x  // ✅ скаляр / элемент
                    }
                })
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // 2. Деление массива на массив: other_arr / self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let other_array = RustyArray::new(other_vec);
                let result = other_array.div(&self.0)?;  // ✅ other / self
                return Ok(Py::new(other.py(), PyRustyFloatArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only divide FloatRustyArray by scalar (float) or another FloatRustyArray"
        ))
    }
    fn to_list(&self) -> Vec<f64> { self.0.to_list() }
    #[getter] fn data(&self) -> Vec<f64> { self.0.data() }
    fn sum_farray(&self) -> f64 { self.0.sum() }
    fn to_int(&self) -> PyRustyIntArray {
        let int_data: Vec<i128> = self.0.data().iter().map(|&x| x as i128).collect();
        PyRustyIntArray(RustyArray::new(int_data))
    }
    fn to_string(&self) -> PyRustyStringArray {
        let string_data: Vec<String> = self.0.data().iter().map(|x| x.to_string()).collect();
        PyRustyStringArray(RustyArray::new(string_data))
    }
    fn append(&mut self, item: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(list) = item.cast::<PyList>() {
            let values: Vec<f64> = list.extract()?;
            self.0.extend(values);
        } else if let Ok(val) = item.extract::<f64>() {
            self.0.append(val);
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected float or list of floats"
            ));
        }
        Ok(())
    }
}

#[pymethods]
impl PyRustyIntArray {
    fn __repr__(&self) -> String { self.0.__repr__() }
    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyIntArray>> {
        if let Ok(scalar) = other.extract::<i128>() {
            let result_data: Vec<i128> = self.0.data().iter()
                .map(|&x| x + scalar)
                .collect();
            return Ok(Py::new(other.py(), PyRustyIntArray(RustyArray::new(result_data)))?);
        }
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.add(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyIntArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only add IntRustyArray + IntRustyArray"
        ))
    }
    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyIntArray>> {
        self.__add__(other)
    }
    fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyIntArray>> {
        // 1. Вычитание скаляра: arr - scalar
        if let Ok(scalar) = other.extract::<i128>() {
            let result_data: Vec<i128> = self.0.data().iter()
                .map(|&x| x - scalar)  // ✅ вычитание
                .collect();
            return Ok(Py::new(other.py(), PyRustyIntArray(RustyArray::new(result_data)))?);
        }

        // 2. Вычитание массива: arr - other_arr
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.sub(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyIntArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only subtract scalar or IntRustyArray from IntRustyArray"
        ))
    }

    fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyIntArray>> {
        // 1. Вычитание из скаляра: scalar - arr
        if let Ok(scalar) = other.extract::<i128>() {
            let result_data: Vec<i128> = self.0.data().iter()
                .map(|&x| scalar - x)  // ✅ обратный порядок: scalar - x
                .collect();
            return Ok(Py::new(other.py(), PyRustyIntArray(RustyArray::new(result_data)))?);
        }

        // 2. Вычитание из массива: other_arr - self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let other_array = RustyArray::new(other_vec);
                let result = other_array.sub(&self.0)?;  // ✅ other - self
                return Ok(Py::new(other.py(), PyRustyIntArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only subtract scalar or IntRustyArray from IntRustyArray"
        ))
    }
    fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyIntArray>> {
        // 1. Проверяем скаляр
        if let Ok(scalar) = other.extract::<i128>() {
            let result_data: Vec<i128> = self.0.data().iter()
                .map(|&x| x * scalar)
                .collect();
            return Ok(Py::new(other.py(), PyRustyIntArray(RustyArray::new(result_data)))?);
        }

        // 2. Проверяем другой массив
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.mul(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyIntArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only multiply IntRustyArray by scalar (int) or another IntRustyArray"
        ))
    }

    fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyIntArray>> {
        // Коммутативно для скаляра
        if let Ok(scalar) = other.extract::<i128>() {
            if scalar == 0{
                return Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
                    "Division by zero is prohibited"
                ));
            } else {
                let result_data: Vec<i128> = self.0.data().iter()
                    .map(|&x| x * scalar)
                    .collect();
                return Ok(Py::new(other.py(), PyRustyIntArray(RustyArray::new(result_data)))?);
            }
        }

        // Для массива: other * self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let other_array = RustyArray::new(other_vec);
                let result = other_array.mul(&self.0)?;
                return Ok(Py::new(other.py(), PyRustyIntArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only multiply IntRustyArray by scalar (int) or another IntRustyArray"
        ))
    }
    fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // Возвращает FloatRustyArray, а не IntRustyArray!

        // 1. Деление на скаляр: arr / scalar
        if let Ok(scalar) = other.extract::<f64>() {  // Принимаем и float
            if scalar == 0.0{
                return Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
                    "Division by zero is prohibited"
                ));
            } else {
                let result_data: Vec<f64> = self.0.data().iter()
                    .map(|&x| x as f64 / scalar)
                    .collect();
                return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
            }
        }

        // 2. Деление на массив: arr / other_arr
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let result_data: Vec<f64> = self.0.data().iter()
                    .zip(other_vec.iter())
                    .map(|(&a, &b)| a as f64 / b as f64)
                    .collect();
                return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only divide IntRustyArray by scalar (int/float) or another IntRustyArray"
        ))
    }

    fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyFloatArray>> {
        // 1. Деление скаляра на массив: scalar / arr
        if let Ok(scalar) = other.extract::<f64>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| scalar / x as f64)
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // Также принимаем целый скаляр
        if let Ok(scalar) = other.extract::<i128>() {
            let result_data: Vec<f64> = self.0.data().iter()
                .map(|&x| scalar as f64 / x as f64)
                .collect();
            return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
        }

        // 2. Деление массива на массив: other_arr / self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<i128>>() {
                let result_data: Vec<f64> = other_vec.iter()
                    .zip(self.0.data().iter())
                    .map(|(&a, &b)| a as f64 / b as f64)
                    .collect();
                return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
            }

            // Если другой массив — FloatRustyArray
            if let Ok(other_vec) = other_data.extract::<Vec<f64>>() {
                let result_data: Vec<f64> = other_vec.iter()
                    .zip(self.0.data().iter())
                    .map(|(&a, &b)| a / b as f64)
                    .collect();
                return Ok(Py::new(other.py(), PyRustyFloatArray(RustyArray::new(result_data)))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only divide scalar (int/float) or array by IntRustyArray"
        ))
    }
    fn to_list(&self) -> Vec<i128> { self.0.to_list() }
    #[getter] fn data(&self) -> Vec<i128> { self.0.data() }
    fn sum_array(&self) -> i128 { self.0.sum() }

    fn to_float(&self) -> PyRustyFloatArray {
        let float_data: Vec<f64> = self.0.data().iter().map(|&x| x as f64).collect();
        PyRustyFloatArray(RustyArray::new(float_data))
    }

    fn to_string(&self) -> PyRustyStringArray {
        let string_data: Vec<String> = self.0.data().iter().map(|x| x.to_string()).collect();
        PyRustyStringArray(RustyArray::new(string_data))
    }
    fn append(&mut self, item: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(list) = item.cast::<PyList>() {
            let values: Vec<i128> = list.extract()?;
            self.0.extend(values);
        } else if let Ok(val) = item.extract::<i128>() {
            self.0.append(val);
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected int or list of ints"
            ));
        }
        Ok(())
    }
}

#[pymethods]
impl PyRustyStringArray {
    fn __repr__(&self) -> String { self.0.__repr__() }
    fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyStringArray>> {
        // 1. Сначала проверяем скалярную строку
        if let Ok(scalar_str) = other.extract::<String>() {
            let result_data: Vec<String> = self.0.data().iter()
                .map(|s| format!("{}{}", s, scalar_str))  // конкатенация с каждой строкой
                .collect();
            return Ok(Py::new(other.py(), PyRustyStringArray(RustyArray::new(result_data)))?);
        }

        // 2. Также пробуем &str (на случай, если передана строка в другом формате)
        if let Ok(scalar_str) = other.extract::<&str>() {
            let result_data: Vec<String> = self.0.data().iter()
                .map(|s| format!("{}{}", s, scalar_str))
                .collect();
            return Ok(Py::new(other.py(), PyRustyStringArray(RustyArray::new(result_data)))?);
        }

        // 3. Проверяем другой массив строк
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<String>>() {
                let other_array = RustyArray::new(other_vec);
                let result = self.0.add(&other_array)?;
                return Ok(Py::new(other.py(), PyRustyStringArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only add StringRustyArray + (StringRustyArray or str)"
        ))
    }

    fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyStringArray>> {
        // 1. Скалярная строка слева: "prefix" + arr
        if let Ok(scalar_str) = other.extract::<String>() {
            let result_data: Vec<String> = self.0.data().iter()
                .map(|s| format!("{}{}", scalar_str, s))
                .collect();
            return Ok(Py::new(other.py(), PyRustyStringArray(RustyArray::new(result_data)))?);
        }

        // 2. &str версия
        if let Ok(scalar_str) = other.extract::<&str>() {
            let result_data: Vec<String> = self.0.data().iter()
                .map(|s| format!("{}{}", scalar_str, s))
                .collect();
            return Ok(Py::new(other.py(), PyRustyStringArray(RustyArray::new(result_data)))?);
        }

        // 3. Другой массив строк: other_arr + self
        if let Ok(other_data) = other.getattr("data") {
            if let Ok(other_vec) = other_data.extract::<Vec<String>>() {
                let other_array = RustyArray::new(other_vec);
                let result = other_array.add(&self.0)?;
                return Ok(Py::new(other.py(), PyRustyStringArray(result))?);
            }
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Can only add (StringRustyArray or str) + StringRustyArray"
        ))
    }
    fn __sub__(&self, _other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyStringArray>> {
        Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
            "Unsupported action. Strings cannot be subtracted from each other."
        ))
    }
    fn __mul__(&self, _other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyStringArray>> {
        Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
            "Unsupported action. Strings cannot be multiplied from each other."
        ))
    }
    fn __truediv__(&self, _other: &Bound<'_, PyAny>) -> PyResult<Py<PyRustyStringArray>> {
        Err(PyErr::new::<pyo3::exceptions::PyArithmeticError, _>(
            "Unsupported action. Strings cannot be divide from each other."
        ))
    }

    fn to_list(&self) -> Vec<String> { self.0.to_list() }
    #[getter] fn data(&self) -> Vec<String> { self.0.data() }

    fn to_int(&self) -> PyResult<PyRustyIntArray> {
        let mut int_data = Vec::new();
        for s in self.0.data().iter() {
            match s.parse::<i128>() {
                Ok(val) => int_data.push(val),
                Err(_) => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Cannot convert '{}' to int", s)
                )),
            }
        }
        Ok(PyRustyIntArray(RustyArray::new(int_data)))
    }

    fn to_float(&self) -> PyResult<PyRustyFloatArray> {
        let mut float_data = Vec::new();
        for s in self.0.data().iter() {
            match s.parse::<f64>() {
                Ok(val) => float_data.push(val),
                Err(_) => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Cannot convert '{}' to float", s)
                )),
            }
        }
        Ok(PyRustyFloatArray(RustyArray::new(float_data)))
    }


    fn append(&mut self, item: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(list) = item.cast::<PyList>() {
            let values: Vec<String> = list.extract()?;
            self.0.extend(values);
        } else if let Ok(val) = item.extract::<String>() {
            self.0.append(val);
        } else if let Ok(val) = item.extract::<&str>() {
            self.0.append(val.to_string());
        } else {
            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "Expected str or list of str"
            ));
        }
        Ok(())
    }
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