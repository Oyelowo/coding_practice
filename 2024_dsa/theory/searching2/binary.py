def binary_search(arr, target):
    left = 0
    right = len(arr) - 1

    while left < right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid

        if arr[mid] > target:
            right -= 1
        else:
            left += 1

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
