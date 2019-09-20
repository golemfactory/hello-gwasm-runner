# hello-gwasm-runner

`Hello gWasmRunner!` is a dead simple program that is meant to demonstrate
how to interface with the [gwasm-runner] and [gwasm-api].

The program showcases how to use the [gwasm-api] and [gwasm-runner] on a
very simple, easily parallelisable problem of computing a sum of an array
of 100 integers in the range of 1..100 (inclusive). To do this, the program
demonstrates how to perform the 3 key steps of the [gwasm-api]: split,
execute (or map), and merge (or reduce). Given our example of computing a
sum of an array of 100 integers, the program splits the array
into 10 gWasm tasks (chunks of 10 integers each; the `split` step),
sums each chunk separately (the `execute`/`map`), and finally, merges the
partial sums by summing all of them to generate the final result (the
`merge`/`reduce` step).

[gwasm-runner]: https://github.com/golemfactory/gwasm-runner
[gwasm-api]: https://github.com/golemfactory/gwasm-runner/tree/master/gwasm-api

## Building
In order to build the project, you'll need a recent version of Emscripten SDK
installed on your machine. The steps on how to do this can be found [here].

[here]: https://emscripten.org/docs/getting_started/downloads.html

Afterwards, assuming you've cloned this repo, in order to build it, simply run:

```
cargo build --release
```

*NB:* this will automatically cross-compile the project to `wasm32-unknown-emscripten`.

## Running
In order to run the generated gWasm binary, firstly, you'll need to get the latest
`gwasm-runner` binary. You can either build it from source (the instructions can be
found [here](https://github.com/golemfactory/gwasm-runner)), or download the
precompiled binary from [here](https://github.com/golemfactory/gwasm-runner/releases).

Afterwards, simply run:

```
gwasm-runner target/wasm32-unknown-emscripten/release/hello_world.wasm
```

