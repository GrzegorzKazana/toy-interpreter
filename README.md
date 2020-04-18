# toy-interpreter

Simple programming language interpreter written in Rust, brought to your browser using Sapper and WebAssembly 🚀

### The language

The purpose of designing this language was learning about compilers and Rust. Consider it a toy.

Features:

-   variables

```
a = 5
b = a + 1
```

-   math expressions

```
2 + 2
9 - 2 * 3
(9 - 2) * 3
```

-   functions

```
fun sum(a, b) = a + b
fun sub(a, b) = sum(a, -b)
```

-   conditional expressions

```
2 ? 3 : 4       // 3, positive values considered truthy
0 ? 3 : 4       // 4

fun min(a, b) = a - b ? b : a
```

-   support for recurrence

```
fun factorial(n) = n ? n * factorial(n - 1) : 1
fun fibonacci(n) = n ? (n - 1 ? fibonacci(n - 1) + fibonacci(n - 2) : 1) : 0
```

Feel free to give it a go at https://grzegorzkazana.github.io/toy-interpreter/.

### Tools

-   [Rust](https://www.rust-lang.org/) compiled to [WebAssembly](https://webassembly.org/) using [wasm-pack](https://github.com/rustwasm/wasm-pack).
-   [Sapper](https://sapper.svelte.dev/) - server-side [Svelte](https://svelte.dev/) framework used for exporting static html (SSG)
-   [Typescript](https://www.typescriptlang.org/)

### How to run

Prerequisites: `node`, `cargo` and `wasm-pack`.

Running the console version of interpreter:

```bash
cd interpreter
cargo run
```

Building the wasm binary:

```
make build-interpreter
```

Building the website:

```
make build-frontend
```

Running tests:

```
make test-all
```

To run frontend dev server (make sure you have built wasm binary prior to this step!):

```
cd frontend
npm run dev
```
