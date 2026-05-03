use pyo3::prelude::*;
use rayon::prelude::*;
//Классы(Массивы)
#[pyclass(name = "f64Array", module = "rusty_math")]
struct RustFloatArray {
    data: Vec<f64>,
}
#[pyclass(name = "IntArray", module = "rusty_math")]
struct RustIntArray {
    data: Vec<i128>,
}

#[pyclass(name = "StringArray", module = "rusty_math")]
struct RustStringArray {
    data: Vec<String>,
}
//Методы Классов(Массивы)
//Float
#[pymethods]
impl RustFloatArray {
    #[new]
    fn new(data: Vec<f64>) -> Self {
        RustFloatArray { data }
    }

    fn __repr__(&self) -> String {
        format!("RustFloatArray({:?})", self.data)
    }

    fn to_list(&self) -> Vec<f64> {
        self.data.clone()
    }
    #[getter]
    fn data(&self) -> Vec<f64> {
        self.data.clone()
    }

    // Метод для суммирования
    fn sum_farray(&self) -> f64 {
        let slice = &self.data;

        // Разбиваем на параллельные части
        let num_chunks = rayon::current_num_threads()*4 ;
        let chunk_size = (slice.len() / num_chunks).max(256);

        let total: f64 = slice
            .par_chunks(chunk_size)
            .map(|chunk| {
                // Оптимизированная сумма для каждого чанка
                let chunks_256 = chunk.chunks_exact(256);
                let remainder = chunks_256.remainder();
                let mut sums = [0.0f64; 64];

                for chunk_256 in chunks_256 {
                    sums[0] += chunk_256[0] + chunk_256[1] + chunk_256[2] + chunk_256[3];
                    sums[1] += chunk_256[4] + chunk_256[5] + chunk_256[6] + chunk_256[7];
                    sums[2] += chunk_256[8] + chunk_256[9] + chunk_256[10] + chunk_256[11];
                    sums[3] += chunk_256[12] + chunk_256[13] + chunk_256[14] + chunk_256[15];
                    sums[4] += chunk_256[16] + chunk_256[17] + chunk_256[18] + chunk_256[19];
                    sums[5] += chunk_256[20] + chunk_256[21] + chunk_256[22] + chunk_256[23];
                    sums[6] += chunk_256[24] + chunk_256[25] + chunk_256[26] + chunk_256[27];
                    sums[7] += chunk_256[28] + chunk_256[29] + chunk_256[30] + chunk_256[31];
                    sums[8] += chunk_256[32] + chunk_256[33] + chunk_256[34] + chunk_256[35];
                    sums[9] += chunk_256[36] + chunk_256[37] + chunk_256[38] + chunk_256[39];
                    sums[10] += chunk_256[40] + chunk_256[41] + chunk_256[42] + chunk_256[43];
                    sums[11] += chunk_256[44] + chunk_256[45] + chunk_256[46] + chunk_256[47];
                    sums[12] += chunk_256[48] + chunk_256[49] + chunk_256[50] + chunk_256[51];
                    sums[13] += chunk_256[52] + chunk_256[53] + chunk_256[54] + chunk_256[55];
                    sums[14] += chunk_256[56] + chunk_256[57] + chunk_256[58] + chunk_256[59];
                    sums[15] += chunk_256[60] + chunk_256[61] + chunk_256[62] + chunk_256[63];
                    sums[16] += chunk_256[64] + chunk_256[65] + chunk_256[66] + chunk_256[67];
                    sums[17] += chunk_256[68] + chunk_256[69] + chunk_256[70] + chunk_256[71];
                    sums[18] += chunk_256[72] + chunk_256[73] + chunk_256[74] + chunk_256[75];
                    sums[19] += chunk_256[76] + chunk_256[77] + chunk_256[78] + chunk_256[79];
                    sums[20] += chunk_256[80] + chunk_256[81] + chunk_256[82] + chunk_256[83];
                    sums[21] += chunk_256[84] + chunk_256[85] + chunk_256[86] + chunk_256[87];
                    sums[22] += chunk_256[88] + chunk_256[89] + chunk_256[90] + chunk_256[91];
                    sums[23] += chunk_256[92] + chunk_256[93] + chunk_256[94] + chunk_256[95];
                    sums[24] += chunk_256[96] + chunk_256[97] + chunk_256[98] + chunk_256[99];
                    sums[25] += chunk_256[100] + chunk_256[101] + chunk_256[102] + chunk_256[103];
                    sums[26] += chunk_256[104] + chunk_256[105] + chunk_256[106] + chunk_256[107];
                    sums[27] += chunk_256[108] + chunk_256[109] + chunk_256[110] + chunk_256[111];
                    sums[28] += chunk_256[112] + chunk_256[113] + chunk_256[114] + chunk_256[115];
                    sums[29] += chunk_256[116] + chunk_256[117] + chunk_256[118] + chunk_256[119];
                    sums[30] += chunk_256[120] + chunk_256[121] + chunk_256[122] + chunk_256[123];
                    sums[31] += chunk_256[124] + chunk_256[125] + chunk_256[126] + chunk_256[127];
                    sums[32] += chunk_256[128] + chunk_256[129] + chunk_256[130] + chunk_256[131];
                    sums[33] += chunk_256[132] + chunk_256[133] + chunk_256[134] + chunk_256[135];
                    sums[34] += chunk_256[136] + chunk_256[137] + chunk_256[138] + chunk_256[139];
                    sums[35] += chunk_256[140] + chunk_256[141] + chunk_256[142] + chunk_256[143];
                    sums[36] += chunk_256[144] + chunk_256[145] + chunk_256[146] + chunk_256[147];
                    sums[37] += chunk_256[148] + chunk_256[149] + chunk_256[150] + chunk_256[151];
                    sums[38] += chunk_256[152] + chunk_256[153] + chunk_256[154] + chunk_256[155];
                    sums[39] += chunk_256[156] + chunk_256[157] + chunk_256[158] + chunk_256[159];
                    sums[40] += chunk_256[160] + chunk_256[161] + chunk_256[162] + chunk_256[163];
                    sums[41] += chunk_256[164] + chunk_256[165] + chunk_256[166] + chunk_256[167];
                    sums[42] += chunk_256[168] + chunk_256[169] + chunk_256[170] + chunk_256[171];
                    sums[43] += chunk_256[172] + chunk_256[173] + chunk_256[174] + chunk_256[175];
                    sums[44] += chunk_256[176] + chunk_256[177] + chunk_256[178] + chunk_256[179];
                    sums[45] += chunk_256[180] + chunk_256[181] + chunk_256[182] + chunk_256[183];
                    sums[46] += chunk_256[184] + chunk_256[185] + chunk_256[186] + chunk_256[187];
                    sums[47] += chunk_256[188] + chunk_256[189] + chunk_256[190] + chunk_256[191];
                    sums[48] += chunk_256[192] + chunk_256[193] + chunk_256[194] + chunk_256[195];
                    sums[49] += chunk_256[196] + chunk_256[197] + chunk_256[198] + chunk_256[199];
                    sums[50] += chunk_256[200] + chunk_256[201] + chunk_256[202] + chunk_256[203];
                    sums[51] += chunk_256[204] + chunk_256[205] + chunk_256[206] + chunk_256[207];
                    sums[52] += chunk_256[208] + chunk_256[209] + chunk_256[210] + chunk_256[211];
                    sums[53] += chunk_256[212] + chunk_256[213] + chunk_256[214] + chunk_256[215];
                    sums[54] += chunk_256[216] + chunk_256[217] + chunk_256[218] + chunk_256[219];
                    sums[55] += chunk_256[220] + chunk_256[221] + chunk_256[222] + chunk_256[223];
                    sums[56] += chunk_256[224] + chunk_256[225] + chunk_256[226] + chunk_256[227];
                    sums[57] += chunk_256[228] + chunk_256[229] + chunk_256[230] + chunk_256[231];
                    sums[58] += chunk_256[232] + chunk_256[233] + chunk_256[234] + chunk_256[235];
                    sums[59] += chunk_256[236] + chunk_256[237] + chunk_256[238] + chunk_256[239];
                    sums[60] += chunk_256[240] + chunk_256[241] + chunk_256[242] + chunk_256[243];
                    sums[61] += chunk_256[244] + chunk_256[245] + chunk_256[246] + chunk_256[247];
                    sums[62] += chunk_256[248] + chunk_256[249] + chunk_256[250] + chunk_256[251];
                    sums[63] += chunk_256[252] + chunk_256[253] + chunk_256[254] + chunk_256[255];
                }

                let mut partial_sum: f64 = sums.iter().sum();
                for &x in remainder {
                    partial_sum += x;
                }
                partial_sum
            })
            .sum();

        total
    }
}
//Int
#[pymethods]
impl RustIntArray {
    #[new]
    fn new(data: Vec<i128>) -> Self {
        RustIntArray { data }
    }

    fn __repr__(&self) -> String {
        format!("RustIntArray({:?})", self.data)
    }

    fn to_list(&self) -> Vec<i128> {
        self.data.clone()
    }
    #[getter]
    fn data(&self) -> Vec<i128> {
        self.data.clone()
    }

    // Метод для суммирования
    fn sum_array(&self) -> i128 {
        let slice = &self.data;

        // Разбиваем на параллельные части
        let num_chunks = rayon::current_num_threads()*4 ;
        let chunk_size = (slice.len() / num_chunks).max(256);

        let total: i128 = slice
            .par_chunks(chunk_size)
            .map(|chunk| {
                // Оптимизированная сумма для каждого чанка
                let chunks_256 = chunk.chunks_exact(256);
                let remainder = chunks_256.remainder();
                let mut sums = [0i128; 64];

                for chunk_256 in chunks_256 {
                    sums[0] += chunk_256[0] + chunk_256[1] + chunk_256[2] + chunk_256[3];
                    sums[1] += chunk_256[4] + chunk_256[5] + chunk_256[6] + chunk_256[7];
                    sums[2] += chunk_256[8] + chunk_256[9] + chunk_256[10] + chunk_256[11];
                    sums[3] += chunk_256[12] + chunk_256[13] + chunk_256[14] + chunk_256[15];
                    sums[4] += chunk_256[16] + chunk_256[17] + chunk_256[18] + chunk_256[19];
                    sums[5] += chunk_256[20] + chunk_256[21] + chunk_256[22] + chunk_256[23];
                    sums[6] += chunk_256[24] + chunk_256[25] + chunk_256[26] + chunk_256[27];
                    sums[7] += chunk_256[28] + chunk_256[29] + chunk_256[30] + chunk_256[31];
                    sums[8] += chunk_256[32] + chunk_256[33] + chunk_256[34] + chunk_256[35];
                    sums[9] += chunk_256[36] + chunk_256[37] + chunk_256[38] + chunk_256[39];
                    sums[10] += chunk_256[40] + chunk_256[41] + chunk_256[42] + chunk_256[43];
                    sums[11] += chunk_256[44] + chunk_256[45] + chunk_256[46] + chunk_256[47];
                    sums[12] += chunk_256[48] + chunk_256[49] + chunk_256[50] + chunk_256[51];
                    sums[13] += chunk_256[52] + chunk_256[53] + chunk_256[54] + chunk_256[55];
                    sums[14] += chunk_256[56] + chunk_256[57] + chunk_256[58] + chunk_256[59];
                    sums[15] += chunk_256[60] + chunk_256[61] + chunk_256[62] + chunk_256[63];
                    sums[16] += chunk_256[64] + chunk_256[65] + chunk_256[66] + chunk_256[67];
                    sums[17] += chunk_256[68] + chunk_256[69] + chunk_256[70] + chunk_256[71];
                    sums[18] += chunk_256[72] + chunk_256[73] + chunk_256[74] + chunk_256[75];
                    sums[19] += chunk_256[76] + chunk_256[77] + chunk_256[78] + chunk_256[79];
                    sums[20] += chunk_256[80] + chunk_256[81] + chunk_256[82] + chunk_256[83];
                    sums[21] += chunk_256[84] + chunk_256[85] + chunk_256[86] + chunk_256[87];
                    sums[22] += chunk_256[88] + chunk_256[89] + chunk_256[90] + chunk_256[91];
                    sums[23] += chunk_256[92] + chunk_256[93] + chunk_256[94] + chunk_256[95];
                    sums[24] += chunk_256[96] + chunk_256[97] + chunk_256[98] + chunk_256[99];
                    sums[25] += chunk_256[100] + chunk_256[101] + chunk_256[102] + chunk_256[103];
                    sums[26] += chunk_256[104] + chunk_256[105] + chunk_256[106] + chunk_256[107];
                    sums[27] += chunk_256[108] + chunk_256[109] + chunk_256[110] + chunk_256[111];
                    sums[28] += chunk_256[112] + chunk_256[113] + chunk_256[114] + chunk_256[115];
                    sums[29] += chunk_256[116] + chunk_256[117] + chunk_256[118] + chunk_256[119];
                    sums[30] += chunk_256[120] + chunk_256[121] + chunk_256[122] + chunk_256[123];
                    sums[31] += chunk_256[124] + chunk_256[125] + chunk_256[126] + chunk_256[127];
                    sums[32] += chunk_256[128] + chunk_256[129] + chunk_256[130] + chunk_256[131];
                    sums[33] += chunk_256[132] + chunk_256[133] + chunk_256[134] + chunk_256[135];
                    sums[34] += chunk_256[136] + chunk_256[137] + chunk_256[138] + chunk_256[139];
                    sums[35] += chunk_256[140] + chunk_256[141] + chunk_256[142] + chunk_256[143];
                    sums[36] += chunk_256[144] + chunk_256[145] + chunk_256[146] + chunk_256[147];
                    sums[37] += chunk_256[148] + chunk_256[149] + chunk_256[150] + chunk_256[151];
                    sums[38] += chunk_256[152] + chunk_256[153] + chunk_256[154] + chunk_256[155];
                    sums[39] += chunk_256[156] + chunk_256[157] + chunk_256[158] + chunk_256[159];
                    sums[40] += chunk_256[160] + chunk_256[161] + chunk_256[162] + chunk_256[163];
                    sums[41] += chunk_256[164] + chunk_256[165] + chunk_256[166] + chunk_256[167];
                    sums[42] += chunk_256[168] + chunk_256[169] + chunk_256[170] + chunk_256[171];
                    sums[43] += chunk_256[172] + chunk_256[173] + chunk_256[174] + chunk_256[175];
                    sums[44] += chunk_256[176] + chunk_256[177] + chunk_256[178] + chunk_256[179];
                    sums[45] += chunk_256[180] + chunk_256[181] + chunk_256[182] + chunk_256[183];
                    sums[46] += chunk_256[184] + chunk_256[185] + chunk_256[186] + chunk_256[187];
                    sums[47] += chunk_256[188] + chunk_256[189] + chunk_256[190] + chunk_256[191];
                    sums[48] += chunk_256[192] + chunk_256[193] + chunk_256[194] + chunk_256[195];
                    sums[49] += chunk_256[196] + chunk_256[197] + chunk_256[198] + chunk_256[199];
                    sums[50] += chunk_256[200] + chunk_256[201] + chunk_256[202] + chunk_256[203];
                    sums[51] += chunk_256[204] + chunk_256[205] + chunk_256[206] + chunk_256[207];
                    sums[52] += chunk_256[208] + chunk_256[209] + chunk_256[210] + chunk_256[211];
                    sums[53] += chunk_256[212] + chunk_256[213] + chunk_256[214] + chunk_256[215];
                    sums[54] += chunk_256[216] + chunk_256[217] + chunk_256[218] + chunk_256[219];
                    sums[55] += chunk_256[220] + chunk_256[221] + chunk_256[222] + chunk_256[223];
                    sums[56] += chunk_256[224] + chunk_256[225] + chunk_256[226] + chunk_256[227];
                    sums[57] += chunk_256[228] + chunk_256[229] + chunk_256[230] + chunk_256[231];
                    sums[58] += chunk_256[232] + chunk_256[233] + chunk_256[234] + chunk_256[235];
                    sums[59] += chunk_256[236] + chunk_256[237] + chunk_256[238] + chunk_256[239];
                    sums[60] += chunk_256[240] + chunk_256[241] + chunk_256[242] + chunk_256[243];
                    sums[61] += chunk_256[244] + chunk_256[245] + chunk_256[246] + chunk_256[247];
                    sums[62] += chunk_256[248] + chunk_256[249] + chunk_256[250] + chunk_256[251];
                    sums[63] += chunk_256[252] + chunk_256[253] + chunk_256[254] + chunk_256[255];
                }

                let mut partial_sum: i128 = sums.iter().sum();
                for &x in remainder {
                    partial_sum += x;
                }
                partial_sum
            })
            .sum();

        total
    }
}
//Python-функции для Float Массивов
#[pyfunction]
#[inline(always)]
/* Приведение float массива к типу f64 */
fn farray(py_list: Vec<f64>) -> RustFloatArray {
    RustFloatArray::new(py_list)
}

#[pyfunction]
#[inline(always)]
fn fsum(arr: &RustFloatArray) -> f64 {
    arr.sum_farray()
}
//Python-функции для Int Массивов

#[pyfunction]
#[inline(always)]
fn sum(arr: &RustIntArray) -> i128 {
    arr.sum_array()
}
#[pyfunction]
#[inline(always)]
/* Приведение int массива к типу i128 */
fn array(py_list: Vec<i128>) -> RustIntArray {
    RustIntArray::new(py_list)
}


#[pymodule]
fn rusty_math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //float
    m.add_function(wrap_pyfunction!(fsum, m)?)?;
    m.add_function(wrap_pyfunction!(farray, m)?)?;
    //int
    //float
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    m.add_function(wrap_pyfunction!(array, m)?)?;


    Ok(())
}