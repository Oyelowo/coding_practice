def insertion_sort(arr):
    n = len(arr)
    for i in range(n):
        current_min_index = i
        currently_popped = arr[i]
        for j in range(i - 1, -1, -1):
            if arr[j] > currently_popped:
                arr[j + 1] = arr[j]
                current_min_index = j

        arr[current_min_index] = currently_popped
    return arr


arx = insertion_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = insertion_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = insertion_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)