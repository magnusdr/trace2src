# `trace2src`
> A simple cli util for tracing a thrown error to its source code using [source maps](https://sourcemaps.info/spec.html).

## Usage
```
$ trace2src --help
Usage: trace2src [OPTIONS] <LINE_NO> <COL_NO>

Arguments:
  <LINE_NO>  
  <COL_NO>   

Options:
  -s, --source-map <SOURCE_MAP>  [default: index.js.map]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Example use case
Let's say you printed the logs from your server side rendered React application, and you see a stack trace like this:
```
[2024-04-03T07:58:11.117Z] error: Error: Just an example error!
    at MyComponent (file:///myapp/build/index.js:1488:9)
    at Uc (/myapp/node_modules/react-dom/cjs/react-dom-server.node.production.min.js:70:44)
    at Xc (/myapp/node_modules/react-dom/cjs/react-dom-server.node.production.min.js:72:253)
    at Z (/myapp/node_modules/react-dom/cjs/react-dom-server.node.production.min.js:78:89)
    at Yc (/myapp/node_modules/react-dom/cjs/react-dom-server.node.production.min.js:81:98)
    ...
```

The `index.js:1488:9` is what you want to trace back to the source code. Then you need the source maps for this index.js (usually just named `index.js.map`). 

You could use `trace2src` to do look at the source code like this:

```
$ trace2src --source-map index.js.map 1488 9
Source file: ../app/components/StatusLabel.tsx:67:8

  64 }
  65 
  66 export function MyComponent({ status }: { status: string }) {
  67   throw new Error("Hello, world!");
  68 
  69   return (
  70     <div>
  71       <div className={className} />
  72       <div>{label}</div>

```