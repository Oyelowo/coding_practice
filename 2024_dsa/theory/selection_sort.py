def selection_sort(arr):
    n = len(arr)
    for i in range(n):
        min_index = i
        for j in range(i + 1, n):
            if arr[j] < arr[min_index]:
                min_index = j
        arr[i], arr[min_index] = arr[min_index], arr[i]
    return arr


arx = selection_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = selection_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = selection_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
