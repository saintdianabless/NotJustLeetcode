from typing import List


class Solution:

    def parseBoolExpr(self, expression: str) -> bool:
        stk = []
        for c in expression:
            if c == ',':
                continue
            elif c != ')':
                stk.append(c)
                continue
            t = f = 0
            while stk[-1] != '(':
                if stk.pop() == 't':
                    t += 1
                else:
                    f += 1
            stk.pop()  # '('
            op = stk.pop()
            if op == '&':
                stk.append('t' if f == 0 else 'f')
            elif op == '|':
                stk.append('t' if t != 0 else 'f')
            else:
                stk.append('t' if f != 0 else 'f')
        return stk[-1] == 't'

    def lengthOfLIS(self, nums: List[int]) -> int:
        N = len(nums)
        dp = [1] * N
        result = 1
        for i in range(1, N):
            for j in range(0, i):
                if nums[i] > nums[j]:
                    dp[i] = max(dp[i], dp[j] + 1)
            result = max(result, dp[i])
        return result


if __name__ == '__main__':
    s = Solution()
    s.parseBoolExpr("|(&(t,f,t),!(t))")
    s.lengthOfLIS([10, 9, 2, 5, 3, 7, 101, 18])
