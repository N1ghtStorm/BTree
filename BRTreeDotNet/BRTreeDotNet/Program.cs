using System;
using System.Collections;
using System.Collections.Generic;

namespace BRTreeDotNet
{
    class Program
    {
        static void Main(string[] args)
        {
            var tree = new TNode<int>();
            var aaa = -11f;
            var uuu = 12f;
            var bbb = aaa.GetHashCode();
            var yyy = uuu.GetHashCode();
            Console.WriteLine(bbb);
            Console.WriteLine(yyy);
        }
    }

    public class BTree<T> where T : IEquatable<T>
    {
        public TNode<T> RootNode { get; set; }
        public void Insert(T value)
        {
            if (RootNode is null)
            {
                RootNode = new TNode<T>() { Value = value };
                return;
            }

            if (RootNode.Value.GetHashCode() == value.GetHashCode())
            {
                RootNode.Count++;
            }

            if (RootNode.Value.GetHashCode() > value.GetHashCode())
            {
                //RIGHT
            }
            else
            {
                //LEFT
            }
        }

    }

    public class TNode<T> where T : IEquatable<T> 
    { 
        public T Value { get; set; }
        public uint Count { get; set; }
        public TNode<T> Right { get; private set; }
        public TNode<T> Left { get; private set; }
    }
}
