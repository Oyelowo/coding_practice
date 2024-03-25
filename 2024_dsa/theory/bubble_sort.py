def bubble_sort(arr):
    n = len(arr) - 1
    for i in range(n):
        for j in range(0, n - i):
            if arr[j + 1] < arr[j]:
                arr[j + 1], arr[j] = arr[j], arr[j + 1]
    return arr


arx = bubble_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = bubble_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = bubble_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
