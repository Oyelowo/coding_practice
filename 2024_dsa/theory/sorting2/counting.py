# [ 3,1,2]
# [0, 1, 1, 1]
def counting_sort(arr):
    max_num = max(arr)
    counter = [0] * max_num + 1
    new_arr = []
    for el in arr:
        counter[el] += 1

    for i, count in enumerate(counter):
        for j in range(count):
            arr[i + j] = i
            # new_arr.append(i)

    return arr


arx = counting_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = counting_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = counting_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
