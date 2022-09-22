// https://leetcode.cn/contest/biweekly-contest-87/
namespace Contest87;

public class Solution
{
    private readonly int[] days = { 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365 };
    public int CountDaysTogether(string arriveAlice, string leaveAlice, string arriveBob, string leaveBob)
    {
        return Math.Max(0, 1 + Math.Min(DateToDay(leaveAlice), DateToDay(leaveBob)) - Math.Max(DateToDay(arriveAlice), DateToDay(arriveBob)));
    }
    private int DateToDay(string date)
    {
        var dat = date.Split('-');
        var (m, d) = (int.Parse(dat[0]), int.Parse(dat[1]));
        if (m == 1)
        {
            return d;
        }
        else
        {
            return days[m - 2] + d;
        }
    }

    public int MatchPlayersAndTrainers(int[] players, int[] trainers)
    {
        Array.Sort(players);
        Array.Sort(trainers);
        int result = 0;
        int i = 0;
        int j = 0;
        while (i < players.Length && j < trainers.Length)
        {
            if (players[i] <= trainers[j])
            {
                ++i; ++j; ++result;
            }
            else
            {
                ++j;
            }
        }
        return result;
    }

    public int[] SmallestSubarrays(int[] nums)
    {
        int N = nums.Length;
        var result = new int[N];
        for (int i = 0; i < N; i++)
        {
            result[i] = 1;
            var e = nums[i];
            for (int j = i - 1; j >= 0; --j)
            {
                if ((nums[j] | e) == nums[j])
                {
                    break;
                }
                nums[j] |= e;
                result[j] = i - j + 1;
            }
        }
        return result;
    }
    
    public long MinimumMoney(int[][] transactions)
    {
        long totalCost = 0;
        long money = 0;
        foreach(var tx in transactions) {
            if(tx[0] > tx[1]) {
                totalCost += tx[0] - tx[1];
            }
        }
        foreach(var tx in transactions) {
            if(tx[0] > tx[1]) {
                totalCost -= tx[0] - tx[1];
            }
            money = Math.Max(money, totalCost + tx[0]);
            if(tx[0] > tx[1]) {
                totalCost += tx[0] - tx[1];
            }
        }
        return money;
    }
}