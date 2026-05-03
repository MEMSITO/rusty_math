import numpy as np
import rusty_math
import math
import time
import random
print("Дробные массивы\n")
# Создаём данные
n = 10_000_000
data_list = [random.uniform(1.0, 10000.0) for _ in range(n)]
data_list_np = np.array(data_list).astype(np.float64)
data_list_f64 = rusty_math.farray(data_list)
print(type(data_list_f64))
print(type(data_list_np))
print(f"Размер массива: {n:,} элементов\n")

# ==================== Тесты ====================

def test_my_fsum():
    start = time.perf_counter()
    result = rusty_math.fsum(data_list_f64)
    end = time.perf_counter()
    print(f"rusty_math.fsum  → {result:.6f} | время: {(end-start)*1000:10.5f} мс")



def test_fsum():
    start = time.perf_counter()
    result = math.fsum(data_list)
    end = time.perf_counter()
    print(f"math.fsum       → {result:.6f} | время: {(end-start)*1000:10f} мс")

def test_numpy():
    start = time.perf_counter()
    result = data_list_np.sum()
    end = time.perf_counter()
    print(f"numpy.sum       → {result:.6f} | время: {(end-start)*1000:10f} мс")



for _ in range(5):
    print(f"--- Запуск {_+1} ---")
    test_my_fsum()
    test_fsum()
    test_numpy()
    print()
print("Целочисленный массивы\n")

# Создаём данные
n = 10_000_000
data_list = [random.randint(1, 10000) for _ in range(n)]
data_list_np = np.array(data_list)
data_list_i128 = rusty_math.array(data_list)
print(type(data_list_i128))
print(type(data_list_np))
print(f"Размер массива: {n:,} элементов\n")

# ==================== Тесты ====================

def test_my_sum():
    start = time.perf_counter()
    result = rusty_math.sum(data_list_i128)
    end = time.perf_counter()
    print(f"rusty_math.sum  → {result:.6f} | время: {(end-start)*1000:10.5f} мс")



def test_sum():
    start = time.perf_counter()
    result = sum(data_list)
    end = time.perf_counter()
    print(f"math.sum       → {result:.6f} | время: {(end-start)*1000:10f} мс")

def test_numpy():
    start = time.perf_counter()
    result = data_list_np.sum()
    end = time.perf_counter()
    print(f"numpy.sum       → {result:.6f} | время: {(end-start)*1000:10f} мс")



for _ in range(5):
    print(f"--- Запуск {_+1} ---")
    test_my_sum()
    test_sum()
    test_numpy()