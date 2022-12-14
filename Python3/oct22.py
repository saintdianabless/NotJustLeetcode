from bisect import bisect_right
from collections import Counter, deque
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

    def shortestBridge(self, grid: List[List[int]]) -> int:
        '''
            # 934. 最短的桥
            https://leetcode.cn/problems/shortest-bridge/
        '''
        R = len(grid)
        C = len(grid[0])
        dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)]
        for x, row in enumerate(grid):
            for y, e in enumerate(row):
                if e != 1:
                    continue
                island = []
                grid[x][y] = -1
                q = deque([(x, y)])
                while len(q) != 0:
                    pos = q.popleft()
                    island.append((pos[0], pos[1]))
                    for dir in dirs:
                        nx = pos[0] + dir[0]
                        ny = pos[1] + dir[1]
                        if nx >= 0 and nx < R and ny >= 0 and ny < C and grid[
                                nx][ny] == 1:
                            grid[nx][ny] = -1
                            q.append((nx, ny))
                step = 0
                while True:
                    last_land = island
                    island = []
                    for x, y in last_land:
                        for dir in dirs:
                            nx = x + dir[0]
                            ny = y + dir[1]
                            if nx >= 0 and nx < R and ny >= 0 and ny < C:
                                if grid[nx][ny] == 1:
                                    return step
                                if grid[nx][ny] == 0:
                                    grid[nx][ny] = -1
                                    island.append((nx, ny))
                    step += 1

    def shortestSubarray(self, nums: List[int], k: int) -> int:
        '''
            # 862. 和至少为 K 的最短子数组
            https://leetcode.cn/problems/shortest-subarray-with-sum-at-least-k/
        '''
        N = len(nums)
        result = 0xffffffff
        prefix_sums = [0] * (N + 1)
        for i in range(N):
            prefix_sums[i + 1] = prefix_sums[i] + nums[i]
        q = []
        idx = 0
        for i, prefix_sum in enumerate(prefix_sums):
            while len(q) > idx and prefix_sum - prefix_sums[q[idx]] >= k:
                result = min(result, i - q[idx])
                idx += 1
            while len(q) != 0 and prefix_sums[q[-1]] >= prefix_sum:
                q.pop()
            q.append(i)
        return result if result < N + 1 else -1

    def arraySign(self, nums: List[int]) -> int:
        '''
            # 1822. 数组元素积的符号
            https://leetcode.cn/problems/sign-of-the-product-of-an-array/
        '''
        sign = 1
        for num in nums:
            if num == 0:
                return 0
            if num < 0:
                sign = -sign
        return sign

    def sumSubarrayMins(self, arr: List[int]) -> int:
        '''
            # 907. 子数组的最小值之和
            https://leetcode.cn/problems/sum-of-subarray-minimums/
        '''
        MOD = 10**9 + 7
        result = 0
        N = len(arr)
        stk = []
        lbound = [0] * N
        rbound = [0] * N
        for i in range(N):
            while stk and arr[i] <= arr[stk[-1]]:
                stk.pop()
            lbound[i] = i - (stk[-1] if stk else -1)
            stk.append(i)
        stk = []
        for i in range(N-1, -1, -1):
            while stk and arr[i] < arr[stk[-1]]:
                stk.pop()
            rbound[i] = (stk[-1] if stk else N) - i
            stk.append(i)
        for i in range(N):
            result = (result + lbound[i] * rbound[i] * arr[i]) % MOD
        return result
            



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


s = Solution()
s.shortestBridge([[1, 1, 1, 1, 1], [1, 0, 0, 0, 1], [1, 0, 1, 0, 1],
                  [1, 0, 0, 0, 1], [1, 1, 1, 1, 1]])
