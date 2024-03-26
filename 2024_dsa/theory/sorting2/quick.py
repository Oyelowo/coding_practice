def pivot_point(arr, low, high):
    pivot = arr[high]
    i = low - 1
    for j in range(low, high):
        if arr[j] < pivot:
            i += 1
            arr[i], arr[j] = arr[j], arr[i]

    arr[i + 1], arr[high] = arr[high], arr[i + 1]

    return i + 1


def quick_sort(arr, low=0, high=None):
    if high is None:
        high = len(arr) - 1

    if low < high:
        pivot_index = pivot_point(arr, low, high)

        quick_sort(arr, low, pivot_index - 1)
        quick_sort(arr, pivot_index + 1, high)

    return arr


arx = quick_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = quick_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = quick_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
