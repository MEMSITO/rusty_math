import numpy as np
import array
import rusty_math as ruth
import math
import time
import random
print("Дробные массивы\n")
# Создаём данные
n = 1_000_000
data_arr = array.array('d', (random.uniform(0.1, 500.9) for _ in range(n)))
data_list_np = np.array(data_arr).astype(np.float64)
data_list_f64 = ruth.array(data_arr.tolist())
data_list_f64 = data_list_f64
del data_arr
print(type(data_list_f64))
print(type(data_list_np))
print(f"Размер массива: {n:,} элементов\n")

# ==================== Тесты ====================

def test_my_fsum():
    start = time.perf_counter()
    result = ruth.fsum(data_list_f64)
    end = time.perf_counter()
    print(f"ruth.fsum  → {result:.6f} | время: {(end-start)*1000:10.5f} мс")



def test_numpy():
    start = time.perf_counter()
    result = data_list_np.sum()
    end = time.perf_counter()
    print(f"numpy.sum       → {result:.6f} | время: {(end-start)*1000:10f} мс")



for _ in range(5):
    print(f"--- Запуск {_+1} ---")
    test_my_fsum()
    test_numpy()
    print()

del data_list_np
del data_list_f64
print("Целочисленный массивы\n")

# Создаём данные
data_list = []
data_arr = array.array('i', (random.randint(1, 500) for _ in range(n)))
data_list_np = np.array(data_arr)
data_list_i128 = ruth.array(data_arr.tolist())
data_list_i128 = data_list_i128
del data_arr
print(type(data_list_i128))
print(type(data_list_np))
print(f"Размер массива: {n:,} элементов\n")

# ==================== Тесты ====================

def test_my_sum():
    start = time.perf_counter()
    result = ruth.sum(data_list_i128)
    end = time.perf_counter()
    print(f"ruth.sum  → {result} | время: {(end-start)*1000:10.5f} мс")

def test_numpy():
    start = time.perf_counter()
    result = data_list_np.sum()
    end = time.perf_counter()
    print(f"numpy.sum       → {result} | время: {(end-start)*1000:10f} мс")



for _ in range(5):
    print(f"--- Запуск {_+1} ---")
    test_my_sum()
    test_numpy()
del data_list_np
del data_list_i128
print("Тест работы метода array")

test_arr = [0,2,3.0,"1gfg"]
t_arr = ruth.array(test_arr)

print(t_arr)

test_arr = [0,2,3.0,27, 56]
t_arr = ruth.array(test_arr)

print(t_arr)

test_arr = [1,2,3,4,5,6,7,8,9]
t_arr = ruth.array(test_arr)

print(t_arr)

test_arr = [1.0,7.0,8.0,9.0]
t_arr = ruth.array(test_arr)

print(t_arr)

print("Тест работы метода append")
# float

list1 = ruth.array([10, 23])
print(list1)
