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
}