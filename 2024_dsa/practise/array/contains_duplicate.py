from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        lookup = {}
        for n in nums:
            lookup[n] = lookup.get(n, 0) + 1
            print(lookup)
            print(lookup.get(n, 0))
            if lookup.get(n, 0) > 1:
                return True
        return False


dup = Solution().containsDuplicate(
    [
        1,
        2,
        3,
        4,
        5,
        6,
        6,
    ]
)

print(dup)
