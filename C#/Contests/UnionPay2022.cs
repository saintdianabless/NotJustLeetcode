// https://leetcode.cn/contest/cnunionpay2022/
namespace UnionPay2022;

public class ListNode
{
    public int val;
    public ListNode? next;
    public ListNode(int x) { val = x; }
}

public class Solution
{
    public int StoredEnergy(int storeLimit, int[] power, int[][] supply)
    {
        supply = supply.Append(new int[] { 1000000, 0, 0 }).ToArray();
        int stored = 0;
        int k = 0;
        for (int i = 0; i < supply.Length - 1; ++i)
        {
            int curBeg = supply[i][0];
            int nextBeg = supply[i + 1][0];
            int curMin = supply[i][1];
            int curMax = supply[i][2];
            for (int j = curBeg; j < nextBeg; ++j)
            {
                if (k == power.Length)
                {
                    return stored;
                }
                int p = power[k];
                if (p < curMin)
                {
                    stored = Math.Max(0, stored - curMin + p);
                }
                else if (p > curMax)
                {
                    stored = Math.Min(storeLimit, stored + p - curMax);
                }
                ++k;
            }
        }
        return 0;
    }

    public int[] ExplorationSupply(int[] station, int[] pos)
    {
        int M = station.Length;
        int N = pos.Length;
        var result = new int[N];
        for (int i = 0; i < N; ++i)
        {
            var p = pos[i];
            var s = Array.BinarySearch(station, p);
            if (s >= 0)
            {
                result[i] = s;
            }
            else if (-s > M)
            {
                result[i] = M - 1;
            }
            else if (s == -1)
            {
                result[i] = 0;
            }
            else
            {
                var pre = station[-s - 2];
                var post = station[-s - 1];
                if ((p - pre) <= (post - p))
                {
                    result[i] = -s - 2;
                }
                else
                {
                    result[i] = -s - 1;
                }
            }
        }
        return result;
    }

    public ListNode ReContruct(ListNode head)
    {
        ListNode dummy = new ListNode(0) { next = head };
        var preNode = dummy;
        var curNode = head;
        while (curNode != null)
        {
            if (curNode.val % 2 == 0)
            {
                preNode.next = curNode.next;
                curNode = preNode.next;
            }
            else
            {
                var next = curNode.next;
                preNode = curNode;
                curNode = next;
            }
        }
        return dummy.next;
    }
}

public class VendingMachine
{
    private Dictionary<string, List<Tuple<int, int, int>>> items = new();  // price, expiredAt, count
    private Dictionary<string, int> customers = new();
    public VendingMachine() { }

    public void AddItem(int time, int number, string item, int price, int duration)
    {
        var lst = items.GetValueOrDefault(item, new List<Tuple<int, int, int>>());
        lst.Add(Tuple.Create(
            price, time + duration, number
        ));
        items[item] = lst;
    }

    public long Sell(int time, string customer, string item, int number)
    {
        if (!items.ContainsKey(item))
        {
            return -1;
        }
        var goods = items[item];
        goods.RemoveAll(good => (good.Item2 < time || good.Item3 == 0));
        var totalGoods = goods.Sum(good => good.Item3);
        if (totalGoods < number)
        {
            return -1;
        }
        var discount = customers.GetValueOrDefault(customer, 100);
        customers[customer] = Math.Max(discount - 1, 70);

        long totalPrice = 0;  // !
        goods.Sort();
        for (int i = 0; i < goods.Count; ++i)
        {
            if (number == 0)
            {
                break;
            }
            var info = goods[i];
            var sold = Math.Min(info.Item3, number);
            totalPrice += (long)sold * info.Item1;
            number -= sold;
            goods[i] = Tuple.Create(info.Item1, info.Item2, info.Item3 - sold);
        }
        var total = (long)totalPrice * discount;  // !
        var r = total % 100 == 0 ? 0 : 1;
        return total / 100 + r;
    }
}