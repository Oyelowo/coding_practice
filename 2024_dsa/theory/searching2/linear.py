def linear_search(arr, target):
    for i, el in enumerate(arr):
        if el == target:
            return i

    return -1


arx = linear_search([3, 56, 78, 2, 1, 4, 6], 78)
print(arx)
arx = linear_search([10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 4)
print(arx)
arx = linear_search([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 18)
print(arx)
