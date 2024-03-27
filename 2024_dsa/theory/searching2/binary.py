def binary_search(arr, target):
    n = len(arr)
    low = 0
    high = n
    mid = (low + high) // 2

    while low < high:
        if arr[mid] == target:
            return mid

        if arr[mid] > target:
            high -= 1
            mid = (low + high) // 2
        else:
            low += 1
            mid = (low + high) // 2

    return -1


# arx = binary_search([3, 56, 78, 2, 1, 4, 6], 78)
# print(arx)
# arx = binary_search([10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 4)
# print(arx)
arx = binary_search([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 18)
print(arx)
arx = binary_search([1, 2, 3, 4, 4, 6, 7, 8, 9, 10], 8)
print(arx)
arx = binary_search([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3)
print(arx)
