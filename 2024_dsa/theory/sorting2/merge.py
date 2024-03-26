def merge(arr1, arr2):
    i1 = i2 = 0
    result = []

    while (i1 < len(arr1)) and (i2 < len(arr2)):
        if arr1[i1] < arr2[i2]:
            result.append(arr1[i1])
            i1 += 1
        else:
            result.append(arr2[i2])
            i2 += 1

    result.extend(arr1[i1:])
    result.extend(arr2[i2:])

    return result


def merge_sort_recursion(arr):
    n = len(arr)
    if n == 1:
        return arr
    mid = n // 2

    left = merge_sort_recursion(arr[:mid])
    right = merge_sort_recursion(arr[mid:])

    return merge(left, right)


print("WITH RECURSION")
arx = merge_sort_recursion([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = merge_sort_recursion([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = merge_sort_recursion([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)


def merge_sort_iteration(arr):
    n = len(arr)
    steps = 1

    while steps < n:
        for i in range(0, n, steps * 2):
            left = arr[i : i + steps]
            right = arr[i + steps : i + (steps * 2)]

            merged = merge(left, right)

            for j, el in enumerate(merged):
                arr[i + j] = el

        steps *= 2

    return arr


def merge_sort_iteration2(arr):
    n = len(arr)
    pair = 2
    mid = pair // 2

    while mid < n:
        for i in range(0, n, pair):
            mid = pair // 2
            left = arr[i : i + mid]
            right = arr[i + mid : i + (pair)]

            merged = merge(left, right)

            for j, el in enumerate(merged):
                arr[i + j] = el

        pair *= 2

    return arr


print("WITH ITERATION")
arx = merge_sort_iteration([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = merge_sort_iteration([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = merge_sort_iteration([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
