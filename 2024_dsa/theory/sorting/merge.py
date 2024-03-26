def merge_arrays(left_arr, right_arr):
    result = []
    pointer_i = 0
    pointer_j = 0

    # [1,3, |5] , [ 2, |4, 6]
    # i=2, j=1
    # [1,2, 2]
    while pointer_i < len(left_arr) and pointer_j < len(right_arr):
        if left_arr[pointer_i] < right_arr[pointer_j]:
            result.append(left_arr[pointer_i])
            pointer_i += 1
        else:
            result.append(right_arr[pointer_j])
            pointer_j += 1

    result.extend(left_arr[pointer_i:])
    result.extend(right_arr[pointer_j:])

    return result


def merge_sort_recursion(arr):
    n = len(arr)
    if n <= 1:
        return arr
    mid = n // 2

    left = merge_sort_recursion(arr[:mid])
    right = merge_sort_recursion(arr[mid:])
    return merge_arrays(left, right)


def merge_sort_iterative_while_loop(arr):
    n = len(arr)
    steps = 1
    while steps < n:
        for i in range(0, n, 2 * steps):
            left = arr[i : i + steps]
            right = arr[i + steps : i + 2 * steps]
            merged = merge_arrays(left, right)
            for x, el in enumerate(merged):
                arr[i + x] = el
        steps *= 2
    return arr


print("ITERATIVE WHILE LOOP")
arx = merge_sort_iterative_while_loop([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = merge_sort_iterative_while_loop([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = merge_sort_iterative_while_loop([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)


def merge_sort_iterative_for_loop(arr):
    n = len(arr)
    steps = 1
    for i in range(0, n, steps):
        for j in range(0, n, 2 * steps):
            left = arr[j : j + steps]
            right = arr[j + steps : j + 2 * steps]
            merged = merge_arrays(left, right)
            for x, el in enumerate(merged):
                arr[j + x] = el
        steps *= 2
    return arr


print("ITERATIVE FOR LOOP")
arx = merge_sort_iterative_for_loop([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = merge_sort_iterative_for_loop([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = merge_sort_iterative_for_loop([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)

print("RECURSION")
arx = merge_sort_recursion([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = merge_sort_recursion([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = merge_sort_recursion([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
