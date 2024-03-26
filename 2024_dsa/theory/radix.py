# [3, 17, 12, 382]
# radix: [[], [],[]..[]] => 10 empty arrays
def radix_sort(arr):
    radix = [
        [],
        [],
        [],
        [],
        [],
        [],
        [],
        [],
        [],
        [],
    ]
    radix = [[] for i in range(10)]

    divisor = 1
    max_num = max(arr)
    n = len(arr)

    while (max_num // divisor) > 0:
        for _ in range(n):
            # while len(arr) > 0:
            item = arr.pop()
            print(item)
            # print(item)
            rad_index = (item // divisor) % 10
            radix[rad_index].append(item)

        for rad in radix:
            while len(rad) > 0:
                ele = rad.pop()
                arr.append(ele)
        divisor *= 10
    return arr


arx = radix_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = radix_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = radix_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
