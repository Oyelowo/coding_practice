# [4,2,4, 3,1] => Max = 4
# [0, 1, 1 , 1, 2]
# $ [1,2,3,4,4]
def counting_sort(arr):
    n = len(arr)
    highest = max(arr)
    counter = [0] * (highest + 1)
    for el in arr:
        counter[el] += 1

    final = []
    for i, count in enumerate(counter):
        for _ in range(count):
            final.append(i)

    return final


arx = counting_sort([3, 56, 78, 2, 1, 4, 6])
print(arx)
arx = counting_sort([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
print(arx)
arx = counting_sort([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
print(arx)
