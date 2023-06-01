# React

React is a client-side framework and Javascript library for building user interfaces. React uses a special markup language called JSX (JavaScript XML).

## JSX 

JSX is a declarative language that is a superset of JavaScript, meaning that all JS is JSX but not all JSX is JavaScript. JSX provides for an easier way to create REACT elements than using traditional JavaScript. 

JSX is intended to be used with preprocessors such as a compiler or transpiler. The compiler, such as Babel, will compile JSX into JavaScript objects to then be parsed by a JavaScript engine.

## A Simple Hello World

The simplest hello world script you can make with React requires the following in a file named index.html

```
<html>
  <head>
    <script src="https://unpkg.com/react@18/umd/react.development.js"></script>
    <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"></script>
    <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
  </head>
<body>
  <div>Hello World!</div>
  <div id="root"></div>
  <script type="text/babel">
    const root = () => ReactDOM.createRoot(document.getElementById('root'));
    root.render(<h1>Hello, world!</h1>);
  </script>
</body>
</html>
```

## Setting up a REACT project using NPM and create-react-app

First off you'll want to install node package manager if you haven't already. Follow these [instructions to install NPM.](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

Next, we'll navigate to the directory we want our project to live and create it.

```
cd /path/to/working/dir
npx create-react-app helloapp
```

This may prompt you to install the create-react-app package if you haven't already; select 'y'. After that command completes cd into the app and start it.

```
cd helloapp
npm start
```

Starting the app will start a local server hosting the app at http://localhost:3000/

In the project directory 'helloapp' you'll see the following structure:
- helloapp
    - node_modules
    - public
    - src
        - App.css
        - App.js
        - App.test.js
        - index.css
        - injdex.js
        - logo.svg
        - reportWebVitals.js
        - setupTests.js
    - .gitignore
    - package.json
    - README.md
    - yarn.lock

For our helloapp we don't need the following files, they can be removed:
- src/App.css
- src/App.test.js
- src/index.css
- src/logo.svg
- src/reportWebVitals.js
- src/setupTests.js

Next we'll edit src/App.js:
```
function App(props) {
    const theDate = new Date();
    return (
        <div>
            <h1>Hello, everybody!</h1>
            <h2>The current time is {theDate.toLocaleTimeString()}</h2>
        </div>
    );
}
export default App;
```
and index.js:
```
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

const root = ReactDOM.createRoot(document.getElementById('root'));
function ref() {
    root.render(<App/>);
}
setInterval(ref, 1000);
```

Then restart the server with ```npm start``` and navigate to http://localhost:3000/

