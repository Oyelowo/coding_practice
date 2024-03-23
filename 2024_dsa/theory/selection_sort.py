import math

# from math import ceil


def selection_sort(arr):
    n = len(arr)
    for i in range(n):
        minimum_index = i
        for j in range(i + 1, n):
            if arr[j] < arr[minimum_index]:
                minimum_index = j
        arr[i], arr[minimum_index] = arr[minimum_index], arr[i]
    return arr


# arx = find_min([3, 56, 78, 2, 1, 4, 6])
arx = selection_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)


# def selection_sort2(arr):
#     sorted_array = []
#     unsorted_array = arr
#     for i in range(len(arr)):
#         minimum = find_min2(unsorted_array)
#         unsorted_array = [num for num in unsorted_array if num != minimum]
#         sorted_array.append(minimum)
#     return sorted_array
#
#
# def find_min2(arr):
#     if len(arr) == 0:
#         return None
#
#     current_minimum = arr[0]
#     for n in arr:
#         if n < current_minimum:
#             current_minimum = n
#
#     return current_minimum
