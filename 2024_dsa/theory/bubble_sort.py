def bubble_sort(arr):
    # for an array of size n, you only need to iterate max of n-1
    # e.g array of size 3, only needs 2 iterations, 4 needs 3 etc,
    # at worst case, e.g [4,3,2,1], u would only need 3 times iterations to bubble to [1,2,3,4]
    item_sorted = False
    iter_count = len(arr) - 1
    for i in range(iter_count):
        for j in range(0, iter_count - i):
            if arr[j] > arr[j + 1]:
                item_sorted = True
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
        if not item_sorted:
            break
    return arr


arx = bubble_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)

###


# def bubble_sort_desc(arr):
#     iterations = len(arr)
#     for i in range(iterations):
#         for j, n in enumerate(arr):
#             # print(j, n)
#             is_last = j == iterations - 1
#             if is_last:
#                 break
#             # print(j, iterations)
#             curr = n
#             next = arr[j + 1]
#             if curr < next:
#                 arr[j + 1] = curr
#                 arr[j] = next
#     return arr
#
#
# def bubble_sort_asc(arr):
#     iterations = len(arr)
#     for i in range(iterations):
#         for j, n in enumerate(arr):
#             is_last = j == iterations - 1
#             if is_last:
#                 break
#             curr = n
#             next = arr[j + 1]
#             if next < curr:
#                 arr[j + 1] = curr
#                 arr[j] = next
#     return arr
#
#
# arx = bubble_sort_asc([3, 56, 78, 2, 1, 4, 6])
# print(arx)
