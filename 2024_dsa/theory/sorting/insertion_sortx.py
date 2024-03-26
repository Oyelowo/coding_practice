def insertion_sort(arr):
    n = len(arr)
    for i in range(1, n):
        curr_val = arr[i]
        insertion_index = i

        # [3, 56, 78,  | 2, 1, 4, 6]
        for j in range(i - 1, -1, -1):
            if arr[j] > curr_val:
                arr[j + 1] = arr[j]
                insertion_index = j
            else:
                break
        arr[insertion_index] = curr_val

    return arr


arx = insertion_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
# arx = insertion_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
# print(arx)
# arx = insertion_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
# print(arx)

# import math
#
# # from math import ceil
#
#
# def insertion_sort(arr):
#     # right_arr = arr
#     # sorted_arr = [arr.pop()]
#     n = len(arr)
#
#     for i in range(1, n):
#         curr_el = arr.pop(i)
#
#         min_index = i
#         for j in range(i - 1, -1, -1):
#             if arr[j] > curr_el:
#                 min_index = j
#         arr[min_index] = curr_el
#         # arr.insert(min_index, curr_el)
#
#     return arr
#
#
# # arx = find_min([3, 56, 78, 2, 1, 4, 6])
# arx = insertion_sort([3, 56, 78, 2, 1, 4, 6])
# print(arx)

# def insertion_sort_brute_force(arr):
#     # right_arr = arr
#     # sorted_arr = [arr.pop()]
#     n = len(arr)
#
#     for i in range(1, n):
#         curr_el = arr.pop(i)
#
#         min_index = i
#         for j in range(i - 1, -1, -1):
#             if arr[j] > curr_el:
#                 min_index = j
#         arr.insert(min_index, curr_el)
#
#     return arr
#
#
# # arx = find_min([3, 56, 78, 2, 1, 4, 6])
# arx = insertion_sort_brute_force([3, 56, 78, 2, 1, 4, 6])
# print(arx)
