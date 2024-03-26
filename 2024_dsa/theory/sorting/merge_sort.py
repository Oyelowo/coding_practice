def merge_sort(arr):
    arr = [7, 6, 5, 4, 3, 2, 1]
    if len(arr) <= 1:
        return arr

    mid = len(arr) // 2
    leftHalf = arr[:mid]
    rightHalf = arr[mid:]

    sortedLeft = {
        arr = [7,6,5]
        if len(arr) <= 1:
            return arr

        mid = len(arr) // 2 # 1
        leftHalf = arr[:mid] # [7]
        rightHalf = arr[mid:] # [6,5]

        sortedLeft = {
            arr = [7]
            if len(arr) <= 1:
                return [7]
        } # [7]
        sortedRight = {
            arr = [6,5]
            if len(arr) <= 1:
                return arr

            mid = len(arr) // 2
            leftHalf = arr[:mid] # [6]
            rightHalf = arr[mid:] # [5]

            sortedLeft = {
                arr = [6]
                if len(arr) <= 1:
                    return arr
            } [6]
            sortedRight = {
                arr = [5]
                if len(arr) <= 1:
                    return arr
            } [5]

            return merge(sortedLeft, sortedRight) # merge([6], [5]) => [5, 6]
        } # [5, 6]

        return merge(sortedLeft, sortedRight)

        
    }
    sortedRight = merge_sort(rightHalf)

    return merge(sortedLeft, sortedRight)

    # arr = [5, 4, 2, 1]
    # if len(arr) <= 1:
    #     return arr
    #
    # mid = len(arr) // 2
    # leftHalf = arr[:mid]
    # rightHalf = arr[mid:]
    #
    # sortedLeft = merge_sort(leftHalf)
    # sortedRight = merge_sort(rightHalf)
    #
    # return merge(sortedLeft, sortedRight)
    #
