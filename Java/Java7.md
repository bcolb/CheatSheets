# Java 7 Cheat Sheet
This is not a *getting started* guide, just a quick-reference for those that are a bit rusty in Java or that haven't coded in it recently (hence why I'm writing this).

## Contents
* [Summary](#summary)
* [Hello World](#helloworld)
* [Primitive Data Types (i.e. int, char)](#primitive)
* [Other Data Types (i.e. String)](#otherdatatypes)
* [Reference Data Types (i.e. arrays)](#referencedatatypes)

## Summary <a name="summary"></a>
Java is a class-based **object-oriented** programming language. It is designed to let engineers *write once, run anywhere*. Java programs can be run on any system or platform that supports Java (*most* do), without needing to be recompiled. 

Java programs compile to **Java bytecode**, which can then run on any **Java Virtual Machine** (JVM). Much of Java, including its syntax, is based on **C** and **C++**. Java is **statically typed**, which means that variables must be declared before they can be used.

## Hello World <a name="helloworld"></a>
filename: **HelloWorld.java**
```java
public class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello world!");
  }
}
```

### Compilation
```sh
javac HelloWorld.java
```
creates a file named **HelloWorld.class**
### Run

```sh
java HelloWorld
```
output:
```
Hello world!
```

## Primitive Data Types <a name="primitive"></a>
* **byte** - an 8-bit signed two's complement integer with range -128 to 127 inclusive (default = 0).
```java
byte b = 37;
```
* **short** - a 16-bit signed two's complement integer with range -32,768 to 32,767 inclusive (default = 0).
```java
short s = 2027;
```
* **int** - a 32-bit signed two's complement integer with range -2^31 to (2^31)-1 inclusive (default = 0).
```java
int number = 53617;
```
* **long** - a 64-bit signed two's complement integer with a range of -2^63 to (2^64)-1 inclusive (Java 8 supports it as unsigned as well) (default = 0L).
```java
int long = 104701;
```
* **float** - a single-precision 32-bit IEEE 754 floating point (note: in Java decimals default to the **double** type, hence the use of *f* to denote a float) (default = 0.0f).
```java
float f = 9.2f;
```
* **double** - a double-precision 64-bit IEEE 754 floating point (the default data type for decimals) (default = 0.0d).
```java
double pi = 3.14159;
```
* **boolean** - has only two possible values, **true** or **false** and represents 1-bit of data (default = false).
```java
boolean isAlive = true;
```
* **char** - a single 16-bit Unicode character with a minimum value of '\u0000' or 0 and a maximum value of 'u\ffff' or 65,535 inclusive (default = '\u0000').
```java
char c = 'a';
```

## Other Data Types <a name="otherdatatypes"></a>
Some data types, such as strings, are represented as objects in Java.
```java
String sentence = "This is just a test of the Java string system.";
```

## Reference Data Types <a name="referencedatatypes"></a>
Technically this refers to variables created using a defined constructor of their class, but here I'm just going over some more basic data types in Java that are not primitive data types.

**Arrays** - a container object that holds a fixed number of values of a *single* type. Array indices are zero-based. Below is an integer array of size ten.
```java
int[] numbers = new int[10];
numbers[0] = 3;
numbers[1] = 29;
``` 
Here is a String array size 64.
```java
String[] words = new String[64];
words[0] = "Java";
```

## Lists

You can define your own lists in Java, here we're just covering use of the ArrayList and LinkedList available from the Java API.
### ArrayList
The ArrayList needs to be imported.
```java
import java.util.ArrayList;
```

```java
ArrayList<String> al = new ArrayList();
al.add("Ginny");
al.add("Ron");
al.add("Fred");
al.add(2, "George");
String name = al.get(0);
int s = al.size();
```
Full details available from [the Java API](http://docs.oracle.com/javase/7/docs/api/java/util/ArrayList.html).

## LinkedList
The LinkedList data structure needs to be imported.
```java
import java.util.LinkedList;
```

```java
LinkedList<Integer> ll = new LinkedList<Integer>();
ll.add(1);
ll.add(3);
ll.add(4);
ll.add(1, 2);
ll.remove(0);
ll.clear();
```
Full details available from [the Java API](http://docs.oracle.com/javase/7/docs/api/java/util/LinkedList.html).

## HashMap
Stores key/value pairs.
```java
import java.util.HashMap;
```
```java
HashMap<String, Integer> hmap = new HashMap<String, Integer>();
hmap.put("Ron", 16);
hmap.put("Harry", 16);
hmap.put("Hermione", 16);
hmap.put("Ginny", 15);
hmap.get("Harry");
```
Full details available from [the Java API](http://docs.oracle.com/javase/7/docs/api/java/util/HashMap.html)

## Operators
Below is a list of operator precendence. For more details [check here](http://docs.oracle.com/javase/tutorial/java/nutsandbolts/operators.html)

| Operators            | Precedence                             |
|----------------------|----------------------------------------|
| postfix              | expr++ expr--                          |
| unary                | ++expr --expr +expr -expr ~ !          |
| multiplicative       | * / %                                  |
| additive             | + -                                    |
| shift                | << >> >>>                              |
| relational           | < > <= >= instanceof                   |
| equality             | == !=                                  |
| bitwise AND          | &                                      |
| bitwise exclusive OR | ^                                      |
| bitwise inclusive OR | |                                      |
| logical AND          | &&                                     |
| logical OR           | ||                                     |
| ternary              | ? :                                    |
| assignment           | = += -= *= /= %= &= ^= |= <<= >>= >>>= |

## Control Flow

**if-then**

```java
boolean sunny = true;
if(sunny) {
  System.out.println("It is sunny outside, yay!");
}
```

**if-then-else**

```java
boolean tired = false;
if(tired) {
  System.out.println("Go get some sleep.");
} else {
  System.out.println("Wanna grab a drink?");
}
```

**switch**

```java
String dayString = "";
int day = 3;
switch (day) {
  case 1: dayString = "Monday";
          break;
  case 2: dayString = "Tuesday";
          break;
  case 3: dayString = "Wednesday";
          break;
  case 4: dayString = "Thursday";
          break;
  case 5: dayString = "Friday";
          break;
  case 6: dayString = "Saturday";
          break;
  case 7: dayString = "Sunday";
          break;
  default: dayString = "Invalid Day";
          break;
}
System.out.println(dayString);
```

**while loop**

```java
boolean done = false;
int count = 0;
while(!done) {
  System.out.println(count);
  count++;
  if(count >= 10) done=true;
}
```

**do-while**

```java
int count = 0;
do {
  count++;
} while(count < 5);
```

**for loop**

```java
int[] numbers = new int[10];
for(int i = 0; i < numbers.length; i++) {
  System.out.println(numbers[i]);
}
```

**for-each loop**
```java
int[] nums = new int[22];
for(int n: nums) {
  System.out.println(n);
}
```

## Object-Oriented Explained
Java is a **class-based** object-oriented programming language. Each file represents a single class, and the filename and class name must match (i.e. filename=HelloWorld.java, classname=HelloWorld). Java **classes** are basically templates that describe the behavior and states that objects of its type support. Individual **objects** are instances of classes with specific states and behaviors. For instance, a robot might be stopped or in motion (state) and be able to move forward to go backwards (behavior). Each class can contain **methods** which correspond to behaviors (i.e. move forward). Classes also define **fields**, which can be used to store the state of objects of its type once instantiated.

Here is an example **Person.java**
```java
/**
 * Person.java
 */
public class Person {
    // Fields
    private int age;
    private String firstName;
    private String lastName;
    
    /**
     * A constructor for the Person class that accepts three parameters:
     * age, first name, last name.
     */
    public Person(int age, String firstName, String lastName) {
	  this.age = age;
	  this.firstName = firstName;
	  this.lastName = lastName;
    }

    /**
     * A getter method that returns the person's age.
     */
    public int getAge() {
	  return this.age;
    }

    /**
     * A setter method that sets our person's age.
     */
    public void setAge(int age) {
     this.age = age;
    }

    /**
     * A method that prints off the person's full name.
     */
    public void printFullName() {
	  System.out.println(this.firstName + " " + this.lastName);
    }

    /**
     * A method that prints off the person's age.
     */
    public void printAge() {
	  if(age == 1) {
	      System.out.println(this.firstName + " is 1 year old");
	  } else {
	      System.out.println(this.firstName + " is " + this.age + " years old");
	  }
    }

    /**
     * A method that increases the person's age by one.
     */
    public void increaseAge() {
	  this.age += 1;
    }

    /**
     * The main method executes only when this file is explicitly run,
     * i.e. 'java Person' from the terminal.
     */
    public static void main(String[] args) {
	  /* These variables will be used as arguments for our person.*/
	  int bobsAge = 52;
	  String bobsFirstName = "Bob";
	  String bobsLastName = "Smith";
	  
	  /* Here we create our new person object.*/
	  Person bob = new Person(bobsAge, bobsFirstName, bobsLastName);
	  
	  /* Now we print off Bob's full name */
	  bob.printFullName();
	  
	  /* Let's see how old Bob is. */
	  bob.printAge();
	  
	  /* Now we increase Bob's age. */
	  bob.increaseAge();
	  
	  /* And we'll see how old Bob is again. */
	  bob.printAge();
    }
}
```

## References

* [Java 7 API](http://docs.oracle.com/javase/7/docs/api/overview-summary.html)

## Recommended Sites

* [Coding Bat](http://codingbat.com/)
