# . By least significant number i.e 1st from the right
# [2, 233, 35, 43, 1, 4] => 2, 3, 5, 3, 1, 4
# radix1 => [[], [1], [2], [233, 43], [4], [35],..[]]
# 2nd least significant number i.e 2nd from the right
# [01, 02, 43, 233, 04, 35,..[]] => 0, 0, 4, 3, 0, 3
# radix2 => [[1,2, 4], [], [233, 35], [43], [], [],..[]]
# [001, 002, 004, 035, 233, 043] => 0, 0, 0, 0, 2, 0
# 3rd least significant number i.e 3rd from the right
# radix3 => [[1, 2, 4, 35, 43], [], [233], [], [], [],..[]]
# [1, 2, 4, 35, 43, 233]
def radix_for_loop(arr):
    n = len(arr)
    max_num = max(arr)
    iter_count = len(str(int(max_num)))

    for i in range(iter_count):
        divisor = 10**i  # 10^0 = 1, 10^1 = 10
        radix = [[] for i in range(10)]
        while len(arr) > 0:
            item = arr.pop()
            radix_index = (item // divisor) % 10
            radix[radix_index].append(item)

        for arr_el in radix:
            while len(arr_el) > 0:
                el = arr_el.pop()
                arr.append(el)

    return arr


print("WITH FOR LOOP")
arx = radix_for_loop([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = radix_for_loop([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = radix_for_loop([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)


def radix_while_loop(arr):
    n = len(arr)
    divisor = 1
    max_num = max(arr)

    while max_num // divisor > 0:
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
        while len(arr) > 0:
            el = arr.pop()
            radix_index = (el // divisor) % 10
            radix[radix_index].append(el)

        for item_arr in radix:
            while len(item_arr) > 0:
                arr.append(item_arr.pop())
        divisor *= 10
    return arr


print("WITH WHILE LOOP")
arx = radix_while_loop([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = radix_while_loop([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = radix_while_loop([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
