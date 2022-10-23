from bisect import bisect_right
from collections import Counter
from math import inf
from typing import List


class Solution:

    def atMostNGivenDigitSet(self, digits: List[str], n: int) -> int:
        '''
            # 902. 最大为 N 的数字组合
            https://leetcode.cn/problems/numbers-at-most-n-given-digit-set/
        '''
        N = len(digits)
        s = str(n)
        L = len(s)
        dp = [[0, 0] for _ in range(L + 1)]
        dp[0][1] = 1
        for i in range(1, L + 1):
            for d in digits:
                if d == s[i - 1]:
                    dp[i][1] = dp[i - 1][1]
                elif d < s[i - 1]:
                    dp[i][0] += dp[i - 1][1]
                else:
                    break
            if i > 1:
                dp[i][0] += dp[i - 1][0] * N + N

        return sum(dp[L])

    def countStudents(self, students: List[int], sandwiches: List[int]) -> int:
        '''
            # 1700. 无法吃午餐的学生数量
            https://leetcode.cn/problems/number-of-students-unable-to-eat-lunch/
        '''
        count = Counter(students)
        for i in sandwiches:
            if count[i] == 0:
                return count[1 - i]
            count[i] -= 1
        return 0

    def kthGrammar(self, n: int, k: int) -> int:
        '''
            # 779. 第K个语法符号
            https://leetcode.cn/problems/k-th-symbol-in-grammar/
        '''
        if k == 1:
            return 0
        if k > (1 << (n - 2)):
            return 1 ^ self.kthGrammar(n - 1, k - (1 << (n - 2)))
        return self.kthGrammar(n - 1, k)

    def jobScheduling(self, startTime: List[int], endTime: List[int],
                      profit: List[int]) -> int:
        '''
            # 1235. 规划兼职工作
            https://leetcode.cn/problems/maximum-profit-in-job-scheduling/
        '''
        N = len(startTime)
        jobs = list(range(N))
        jobs.sort(key=lambda job: endTime[job])
        dp = [0] * (N + 1)

        for i in range(0, N):
            cur_job = jobs[i]
            k = bisect_right(jobs,
                             startTime[cur_job],
                             hi=i,
                             key=lambda job: endTime[job])
            dp[i + 1] = max(dp[i], dp[k] + profit[cur_job])

        return dp[N]

    def mergeAlternately(self, word1: str, word2: str) -> str:
        '''
            # 1768. 交替合并字符串
            https://leetcode.cn/problems/merge-strings-alternately/
        '''
        M, N = len(word1), len(word2)
        result = [None] * (M + N)
        idx = 0
        m = max(M, N)
        for i in range(m):
            if i < M:
                result[idx] = word1[i]
                idx += 1
            if i < N:
                result[idx] = word2[i]
                idx += 1
        return ''.join(result)

    def partitionDisjoint(self, nums: List[int]) -> int:
        '''
            # 915. 分割数组
            https://leetcode.cn/problems/partition-array-into-disjoint-intervals/
        '''
        N = len(nums)
        cm = lm = nums[0]
        pos = 0
        for i in range(1, N - 1):
            cm = max(cm, nums[i])
            if nums[i] < lm:
                lm, pos = cm, i
        return pos + 1


class StockSpanner:
    '''
        # 901. 股票价格跨度
        https://leetcode.cn/problems/online-stock-span/
    '''

    def __init__(self):
        self.stk = [(-1, inf)]
        self.curIdx = -1

    def next(self, price: int) -> int:
        self.curIdx += 1
        while self.stk[-1][1] <= price:
            self.stk.pop()
        self.stk.append((self.curIdx, price))
        return self.curIdx - self.stk[-2][0]
