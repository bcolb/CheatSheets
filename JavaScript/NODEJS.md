# Node.js

Node.js is a server-side JaavScript runtime environment. It runs on Google's V8 Chrome JavaScript engine and allows you to execute JavaScript code outside of the browser.

## Node.js Architecture

While single-threaded, by leveraging an event-driven asynchronous architecture and callbacks Node.js is a very fast language for concurrent, IO-bound operations such as web servers. This architecture allows Node.js to handle a high volume of concurrent connections. 

## Installing and Using Node.js

Install via a [package manager](https://nodejs.org/en/download/package-manager) or download the [binary](https://nodejs.org/en/download).

Then, in the terminal type the following to open up an interactive console:
```
node
```

Use the command ```.exit``` to exit the prompt, and using ```node --version``` from the terminal to see the version of Node.js that you're running.

## Import and Require

Every JavaScript file is a module in Node.js

A Module corresponds to a script file and a package can contain one or more nodes.

Import and require are used to bring a JavaScript module into an application.

Two common module specifications: CommonJS and ES

CommonJS
- import with ```require()```
    - Can be called anywhere in the code
    - Can be called within conditionals and functions
    - Dynamic (can create run-time errors)
    - Synchronous
- export with ```module.exports```
- naming convention: {package}.js

ES
- import with ```import()```
    - Can only be called at the beginning of the file
    - Cannot be called within conditionals or functions
    - Static
    - Asynchronous (runs faster because of this)
- export with ```export```
- naming convfention: {package}.mjs

## Simple Web Server

Save the following in simpleserver.js:

```
const http = require('http');

let server = http.createServer(function(request, response) {
    let body = "Hello world!";

    response.writeHead( 200, {
        'Content-Length':body.length,
        'Content-Type':'text/plain'
    });
    response.end(body);
});
server.listen(8080);
```

Then run ```node simple-server.js``` from the command line and navigate to http://localhost:8080/

## Node.js Modules

Package.json - describes details of the module. Without a package.json Node assumes index.js is the main class.


Importing a module
```
const http = require('http');
```

Import from a file with a relative path
```
let the_date = require('./the_date');
```

Then the script would be in /the_date/index.js file.
```
let todays_date = new Date();
console.log("in module");
exports.todays_date = todays_date;
```

Then use it in a script (test_todays_date.js):
```
console.log("in script")
let the_date = require('./the_date');
console.log("back in script")
// see what's in the module
console.log(the_date);
// log just today's date to console
console.log(the_date.todays_date);
```

## Node Package Manager (NPM)

NPM is the default package manager for Node.js

There are a few important core modules: http, path, fs, os, util, url, and querystring

Installing a package from the command line (local):
```
npm install <package_name>
```

Global package install:
```
npm install -g <package_name>
```

All NPM packages have a package.json file in the project's root directory. NPM uses package.json to determine dependencies.

package.json example
```
{
    "name" : "cheatSheets",
    "version" : "0.1.0",
}
```
