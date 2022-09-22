// 707. 设计链表
// https://leetcode.cn/problems/design-linked-list/
namespace DesignLinkedList;

// no dummy
public class MyLinkedList
{
    class Node
    {
        public int Value { get; set; }
        public Node? Next { get; set; }

        public Node(int val, Node? next = null)
        {
            this.Value = val;
            this.Next = next;
        }
    }

    private Node? head = null;
    public int Length { get; private set; } = 0;

    public MyLinkedList() { }

    private Node? GetNode(int index)
    {
        if (index < 0 || this.Length == 0)
        {
            return null;
        }
        var node = head;
        for (int i = 0; i < index; i++)
        {
            if (node!.Next == null)
            {
                return null;
            }
            node = node.Next;
        }
        return node;
    }

    public int Get(int index)
    {
        return GetNode(index)?.Value ?? -1;
    }

    public void AddAtHead(int val)
    {
        AddAtIndex(0, val);
    }

    public void AddAtTail(int val)
    {
        AddAtIndex(this.Length, val);
    }

    public void AddAtIndex(int index, int val)
    {
        if (index > Length)
        {
            return;
        }
        if (index <= 0)
        {
            head = new Node(val, head);
        }
        else if (index == Length)
        {
            var last = GetNode(index - 1);
            last!.Next = new Node(val);
        }
        else
        {
            var pre = GetNode(index - 1);
            pre!.Next = new Node(val, pre.Next);
        }
        ++Length;
    }

    public void DeleteAtIndex(int index)
    {
        if (index < 0 || index >= this.Length)
        {
            return;
        }
        if (index == 0)
        {
            head = head!.Next;
        }
        else if (index == this.Length - 1)
        {
            var node = GetNode(this.Length - 2);
            node!.Next = null;
        }
        else
        {
            var node = GetNode(index - 1);
            node!.Next = node!.Next!.Next;
        }
        --Length;
    }
}
