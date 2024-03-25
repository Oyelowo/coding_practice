def quick_sort(arr):
    pivot = arr[-1]
    n = len(arr)

    for i in range(n):
        left_min = i
        # for j in range(i, n):
        #     if arr[i] <arr[pivot]:
        #
        #     []


arx = quick_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = quick_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = quick_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
