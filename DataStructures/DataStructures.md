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

## Resources

1. [Data Structures And Algorithm Analysis in Java 3rd Edition by Mark Allen Weiss](http://www.amazon.com/Data-Structures-Algorithm-Analysis-Java/dp/0132576279/ref=sr_1_1?ie=UTF8&qid=1425517283&sr=8-1&keywords=Data+Structures+and+Algorithms+analysis+in+java+mark+allen)
2. [Big O Cheat Sheet](http://bigocheatsheet.com/)