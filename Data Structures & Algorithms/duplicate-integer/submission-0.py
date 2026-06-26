class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        value = set()
        for num in nums:
            if num in value:
                return True
            else:
                value.add(num)
        return False