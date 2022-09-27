namespace D220922;

public class Solution
{
    // 1480. 一维数组的动态和
    // https://leetcode.cn/problems/running-sum-of-1d-array/
    public int[] RunningSum(int[] nums)
    {
        var N = nums.Length;
        var result = new int[N];
        result[0] = nums[0];
        for (int i = 1; i < N; ++i)
        {
            result[i] = result[i - 1] + nums[i];
        }
        return result;
    }

    // 724. 寻找数组的中心下标
    // https://leetcode.cn/problems/find-pivot-index/
    public int PivotIndex(int[] nums)
    {
        var sum = nums.Sum();
        int ls = 0;
        for (int i = 0; i < nums.Length; i++)
        {
            int left = sum - nums[i];
            if (left % 2 == 0 && ls == left / 2)
            {
                return i;
            }
            ls += nums[i];
        }
        return -1;
    }

    // 1652. 拆炸弹
    // https://leetcode.cn/problems/defuse-the-bomb/
    public int[] Decrypt(int[] code, int k)
    {
        int N = code.Length;
        var result = new int[N];
        int sign = k > 0 ? 1 : -1;
        for (int i = 0; i < N; i++)
        {
            for (int j = 1; j <= k * sign; j++)
            {
                int idx = (i + j * sign + N) % N;
                result[i] += code[idx];
            }
        }
        return result;
    }

    // 788. 旋转数字
    // https://leetcode.cn/problems/rotated-digits/solution/
    private int[] check = { 2, 2, 1, 0, 0, 1, 1, 0, 2, 1 };
    public int RotatedDigits(int n)
    {
        int result = 0;
        for (int i = 1; i <= n; ++i)
        {
            var si = i.ToString();
            var atLeast1 = false;
            var not347 = true;
            foreach (var c in si)
            {
                var idx = c - '0';
                if (check[idx] == 0)
                {
                    not347 = false;
                }
                else if (check[idx] == 1)
                {
                    atLeast1 = true;
                }
            }
            if (atLeast1 && not347)
            {
                ++result;
            }
        }
        return result;
    }

    // 面试题 17.19. 消失的两个数字
    // https://leetcode.cn/problems/missing-two-lcci/
    public int[] MissingTwo(int[] nums)
    {
        int xorN = 0;
        for (int i = 1; i <= nums.Length + 2; i++)
        {
            xorN ^= i;
        }
        int xorNums = xorN;
        foreach (var num in nums)
        {
            xorNums ^= num;
        }
        int least1 = (xorNums == int.MinValue) ? xorNums : xorNums & (-xorNums);  // 防止-xorNums溢出

        var result = new int[2] { 0, 0 };
        foreach (var num in nums)
        {
            if ((num & least1) != 0)
            {
                result[0] ^= num;
            }
            else
            {
                result[1] ^= num;
            }
        }
        for (int i = 0; i <= nums.Length + 2; i++)
        {
            if ((i & least1) != 0)
            {
                result[0] ^= i;
            }
            else
            {
                result[1] ^= i;
            }
        }
        return result;
    }

    // 面试题 01.02. 判定是否互为字符重排
    // https://leetcode.cn/problems/check-permutation-lcci/
    public bool CheckPermutation(string s1, string s2)
    {
        var h1 = new Dictionary<char, int>();
        foreach (var c in s1)
        {
            var count = h1.GetValueOrDefault(c, 0);
            h1[c] = count + 1;
        }
        foreach (var c in s2)
        {
            if (!h1.ContainsKey(c))
            {
                return false;
            }
            h1[c] -= 1;
        }
        return !h1.Values.Any(v => v != 0);
    }

    // 面试题 17.09. 第 k 个数
    // https://leetcode.cn/problems/get-kth-magic-number-lcci/
    public int GetKthMagicNumber(int k)
    {
        var inits = new int[]{3,5,7};
        var pq = new PriorityQueue<long, long>();
        var exists = new HashSet<long>();
        pq.Enqueue(1, 1);
        exists.Add(1);
        long magic = 0;
        for(int i = 0; i < k; i++) {
            magic = pq.Dequeue();
            foreach(var f in inits) {
                var m = f * magic;
                if(exists.Add(m)) {
                    pq.Enqueue(m, m);
                }
            }
        }
        return (int)magic;
    }
}