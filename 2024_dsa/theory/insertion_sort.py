def insertion_sort(arr):
    n = len(arr)
    # [4,3, 2]
    for i in range(n):
        curr_minimum_index = i
        shifted_el = arr[i]
        for j in range(i, -1, -1):
            if arr[j] > shifted_el:
                arr[j + 1] = arr[j]
                curr_minimum_index = j

        arr[curr_minimum_index] = shifted_el

    return arr


arx = insertion_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = insertion_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = insertion_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
