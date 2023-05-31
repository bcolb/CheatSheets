# JavaScript Cheat Sheet

This is just a quick-reference guide for JavaScript.

## Summary

JavaScript is a high-level language that conforms to thhe ECMAScript standard and was originally used only in web browsers. However, starting with Node.js JavaScript has made its way into server side processing.

## Hello World

There are a few ways to write Hello World in the browser with JavaScript.

```
console.log("Hello, World!");
alert("Hello, World!");
document.write("Hello, World!");
```

You can execute the above hello world code in a browser console. To get to the console:

In Chrome:
1. Click on the 'three dots' icon (aka the kebab menu)
2. Hover over 'more tools'
3. Click on 'developer tools'
4. Select the 'Console' tab
5. Type your command at the '>' prompt

## Data Types

JavaScript types are dynamic.

### Primitive data types

JS features the following primitive types:
- string
- number
- bigint
- boolean
- undefined
- symbol
- null

Additionally, JavaScript features the 'Object' DataType.

### Data Type Examples

```
// Strings
let name = "Brice"
let sentence = "This is just a sentence."

// Numbers
let a_good_number = 42;
let another_good_number = 7;
let a_pi = 3.14;

// Bigint
let x = BigInt("8372032582392352345239385232346097670922468")

// Booleans
let awake = True;
let am_sleeping = False;

// undefined
let not_a_function = undefined

// symbol
const sym = Symbol("foo");

// Null
let to_be_named = null
```

### Object Examples

```
// Arrays
const colors = ["yellow", "red", "blue"]
```

```
// Objects
const dog = {name:"Lassie", breed:"Rough Collie", sex:"female"}
```

## Var, Let, and Const

Variables in javascript are defined with the ```var``` keyword. 

```
var x = 'x';
var name = 'cheatsheet';
```
Let is used to declare a variable that is block scoped, meaning they have scope in the block they are declared and in any sub blocks.

```
let n = 12;
```

the ```const``` keyword can be used to create a block-scoped constant. This is similar to the scoping of let but a const variable can't be changed through reassignment.

```
const number = 7;
```

## Control Flow

Javascript control flow is used with if/else statements.

```
const fundamental_truth = true;

var result = null;

if (fundamental_truth) {
    result = "the_truth";
} else {
    result = "a lie";
}
```

Switch statement

```
var x = 2;
var y = null;

switch (x) {
    case 1:
    y = "one";
    break;
    case 2:
    y = "two";
break;
default:
console.log("no case found");
}
```

For loops

```
var result = ""

for (var count = 0; count < 144; count++) {
    result += "Around the world, around the word\n";    
}
console.log(result);
```

While loops

```
var is_it_over = false;
while (!is_it_over) {
    console.log("it's not over yet");
    is_it_over = true;
}
console.log("now it's over");
```

## Functions, Prototypes,  and Classes

```
function say_hi (name) {
    return "Hello, " + name + "!";
}
console.log(say_hi("Jack"));
```

ES6 allows for arrow functions.

```
const say_hi = (name)=> console.log("Hello " + name + "!");
```

Objects can be made from functions.

```
function Dog(breed, color, age) {
    this.breed = breed;
    this.color = color;
    this.age = age;
    this.bark = function () {
        if (this.breed == 'Pomeranian') {
            return "Yip!";
        } else {
            return "Bark!";
        }
    }
}
var mydog = new Dog("lab", "yellow", 4);
console.log(mydog.bark());
```

Prototypes

```
Dog.prototype.name = "Dog";
Dog.prototype.getName = function() {
    return this.name;
}
console.log(mydog.getName());
```

Note the abovef example returns 'Dog' for my dog as the prototype added name to all dog objects.


Class
- introduced with ES6
- can have a constructor

```
class Dog {
    constructor(name, breed) {
        this.name = name;
        this.breed = breed;
        console.log("Dog created");
        console.log("Name: " + this.name);
        console.log("Breed: " + this.breed);
    }
};

let theDog = new Dog("Lassie", "Rough Collie");
```

Class Inheritance
```
class BlackLab extends Dog {
    constructor(name) {
        super(name, "Black Lab");
    }
};

let oldDog = new BlackLab("Charlie");
```

Self-executing functions - run immediatlely after invocation.

```
(function () {
    const fundamental_truth = true;
    var program_name = "truthteller";
    console.log(program_name);
})();
```

## Promise

Starting with ES6 a JavaScript promise represents the eventual completion of an asynchronous operation and its return value.

Promises have state:
- pending - when it is invoked
- fulfilled - when it completes
- rejected - when it fails

```
let thePromise = ((resolve, reject) => {
    setTimeout(() => {
        let currTime = new Date().getTime();
        if(currTime % 2 === 0) {
            resolve("Success!")
        } else {
            reject("Failed!!")
        }
    }, 4000)
})

let newPromise = new Promise(thePromise);
```


## Working with HTML

APIs for working with nodes in an HTML doc
- ```document.getElementById(id);```
- ```document.getElementsByTagName(name);```
- ```document.createElement(Name);```
- ```parentNode.appendChild(node);```

This functionality is greatly extended withy [jQuery](https://jquery.com/).

```
// get all links on a page
var linkSet = document.getElementsByTagName("a");
var links = [];
for (var i = 0; i < linkSet.length; i++) {
    links.push(linkSet[i].href);
}
console.log(links);
```

Adding nodes to a page.

```
<html>
  <head>
    <script>
      function addHelloWorld() {
        var newP = document.createElement("p");
        var newHello = document.createTextNode("Hello world!");
        newP.appendChild(newHello);
        document.body.appendChild(newP);
      }
    </script>
  </head>
  <body onload="addHelloWorld()">
  </body>
</html>
```

Modifying content.
- ```element.innerHTML```
    - retrieves or sets content of an HTML element
- ```element.style```
    - retrieve or set style
    - overrides CSS style

Modifying attributes
- ```element.setAttribute(attrName, attrValue);```
- ```element.removeAttribute(attrName);```
- ```element.getAttribute(attrName);```

Events
- Open a new browser window
    - ```window.open(url, name, [features, replace]);```
        - returns a reference to the new window object
- Start a function after the page is loaded
    - ```window.onload```
- Write a message to the console
    - ```window.dump("text");```
- Scroll the browser to a spot on a page
    - ```window.scrollTo(x, y);```

Using OnLoad

```
// Run a function onload
onload = (function () {
    alert("Ran on page load");
})();
```

```
// Here's the commonly used jQuery version
$( document ).read(function() {
    alert("jQuery ran on document ready");
});
```
