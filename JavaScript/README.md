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
```

```
// Numbers
let a_good_number = 42;
let another_good_number = 7;
let a_pi = 3.14;
```

```
// Bigint
let x = BigInt("8372032582392352345239385232346097670922468")
```

```
// Booleans
let awake = True;
let am_sleeping = False;
```

```
// undefined
let not_a_function = undefined
```

```
// symbol
const sym = Symbol("foo");
```

```
// Null
let to_be_named = Null
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




