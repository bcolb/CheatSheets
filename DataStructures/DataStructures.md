# Data Structures Cheat Sheet
A few notes on commonly used data structures and their time and space complexities.

## Lists
A list is a sequence of values where the same value may occur more than once.

### Array List
| Operation | Time Complexity (worst) |
|-----------|-------------------------|
| index     | O(1)                    |
| printList | O(N)                    |
| insertion | O(N)                    |
| deletion  | O(N)                    |
| search    | O(N)                    |
space complexity: O(N)

**Java**

[ArrayList - Java API](http://docs.oracle.com/javase/7/docs/api/java/util/ArrayList.html)

```java
import java.util.ArrayList;
```
```java
ArrayList<String> al = new ArrayList<String>();
```

### Singly Linked List

| Operation | Time Complexity (worst) |
|-----------|-------------------------|
| index     | O(N)                    |
| printList | O(N)                    |
| insertion | O(1)                    |
| deletion  | O(1)                    |
| search    | O(N)                    |
space complexity: O(N)

### Doubly Linked List

| Operation | Time Complexity (worst) |
|-----------|-------------------------|
| index     | O(N)                    |
| printList | O(N)                    |
| insertion | O(1)                    |
| deletion  | O(1)                    |
| search    | O(N)                    |
space complexity: O(N)

**Java**

[LinkedList - Java API](http://docs.oracle.com/javase/7/docs/api/java/util/LinkedList.html)

```java
import java.util.LinkedList;
```
```java
LinkedList<String> ll = new LinkedList<String>();
```

## Stacks (LIFO)
A stack is a list with the restriction that insertions and deletions can be performed in only one position, the end of the list aka **the top**. Stack operations include **push** (insert), **pop** (remove/delete), and **top** (aka peek, see the top element). 

**Common Uses**
* Balancing Symbols
* Postfix Expressions
* Method Calls (Program Stack)

| Operation | Time Complexity |
|-----------|-----------------|
| Push      | O(1)            |
| Pop       | O(1)            |

**Java**

[Stack - Java API](http://docs.oracle.com/javase/7/docs/api/java/util/Stack.html)
```java
import java.util.Stack;
```
```java
Stack<String> st = new Stack<String>();
st.push("!");
st.push("world");n
st.push(" ");
st.push("Hello");
```

## Queues (FIFO)
Queues are lists (like stacks) with insertion being done at one end and deletion being done at the other (FIFO - First In, Last Out).

**Note:** Can be implemented with a Linked List, Array List, or Array (Circular Queue).

**Operations**
* enqueue - inserts an element at the end of the list (the rear).
* dequeue - deletes and returns the element at the start of the list (the front).

| Operation | Time Complexity |
|-----------|-----------------|
| enqueue   | O(1)            |
| dequeue   | O(1)            |

**Java**

[Queue Interface - Java API](http://docs.oracle.com/javase/7/docs/api/java/util/Queue.html)

[Priority Queue - Java API](http://docs.oracle.com/javase/7/docs/api/java/util/PriorityQueue.html)

## Trees

A tree is a collection of nodes, which can be empty, but otherwise consists of a root node (r) and zero or more nonempty subtrees each of whose roots are connect by a directed edge from the root (r). The root of each subtree is a child of r, and r is the parent of each subtree root.

**recursive definition**

A tree is a collection of N nodes, one of which is the root, and N-1 edges.

**Terminology**
* Root Node - The first node in the tree, does not have a parent.
* Parent Node - The root node of some subtree or tree, with at least one child node.
* Leaves - Nodes with no children.
* Siblings - Nodes with the same parent node.
* Grandparent - parent of a parent
* Grandchild - child of a child
* Path - a sequence of nodes such that n(i) is the parent of n(i+1). The length is the number of edges on the path. In a tree there is exactly one path from the root to each node.
* Depth - the depth of a node is the length of the unique path fromo the root to the node.
* Height - The height of a node is the length of the longpest path from the node to a leaf - with all leaves are at height 0. The height of a tree is the height of its root.
* Ancestor/Descendant - if there is a path from node 1 to node 2, node 1 is an ancestor or node 2, and node 2 is a decendant of node 1. If they are not equal then they are proper ancestors/descendants.

**Uses**
* File Systems
* Evaluate Arithmetic Expressions

**Note:** Can be implemented with a Linked List.

**Tree Traversals**
* preorder traversal - work at anode is performed before its children are processed. (corresponds to prefix)
* postorder traversal - the work at a node is performed after its children are evaluated. (corresponds to postfix)
* inorder traverals - left, node, right (corresponds to infix)

A binary tree is a tree where each node can have at most two children. A Tree consists of a root and two subtrees (which could be empty).

### Binary Search Tree
A tree structure where each node has at most two children, where the *left child* is less than the current node, and the *right child* is greater than the current node. 

### AVL Tree
An AVL Tree is a balanced binary search tree. It achieves a better worst case run-time by imposing a balance condition after every insert or delete operation, such that a leaf can have a depth at most one more than any other leaf.

### Splay Tree
A splay tree is a binary search tree that uses splaying to reduce the time complexity of operations done in sequence.

### B-Tree
A B-tree uses more than two branches, or subtrees in order to reduce the overall depth of the search tree. This can be useful in certain applications as it decreases the number of disk accesses necessary to perform a search.

## Resources

1. [Data Structures And Algorithm Analysis in Java 3rd Edition by Mark Allen Weiss](http://www.amazon.com/Data-Structures-Algorithm-Analysis-Java/dp/0132576279/ref=sr_1_1?ie=UTF8&qid=1425517283&sr=8-1&keywords=Data+Structures+and+Algorithms+analysis+in+java+mark+allen)
2. [Big O Cheat Sheet](http://bigocheatsheet.com/)