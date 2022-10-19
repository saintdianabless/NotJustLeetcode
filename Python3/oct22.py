from collections import Counter
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
        dp = [[0,0] for _ in range(L+1)]
        dp[0][1] = 1
        for i in range(1, L + 1):
            for d in digits:
                if d == s[i - 1]:
                    dp[i][1] = dp[i-1][1]
                elif d < s[i - 1]:
                    dp[i][0] += dp[i-1][1]
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
                return count[1-i]
            count[i] -= 1
        return 0

    def kthGrammar(self, n: int, k: int) -> int:
        '''
            # 779. 第K个语法符号
            https://leetcode.cn/problems/k-th-symbol-in-grammar/
        '''
        if k == 1:
            return 0
        if k > (1 << (n-2)):
            return 1 ^ self.kthGrammar(n-1, k - (1 << (n-2)))
        return self.kthGrammar(n-1, k)