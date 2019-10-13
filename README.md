# sprockit.io

## Requirements

Install the following to build/develop sprockit.io.

- [Node.js](https://nodejs.org/en/) and
  [Yarn](https://yarnpkg.com/en/docs/getting-started)
- [Rust](https://rustup.rs/)

If you use VS Code follow the guide on [Setting up
VSCode](https://github.com/mdaffin/sprockit.io/wiki/Setting-up-VSCode).

## Quick Start

### The Game Server

Game servers are in `./games/$game`. To compile and run one, for example the
maze solver run the following

```bash
cd game/maze
cargo install
cargo run
```

This will start a http server on http://localhost:4000.

### The Web UI

With the API server running you can start the UI in `./web-ui`.

```bash
cd web-ui
yarn install
yarn run dev
```

This will start another web server on http://localhost:3000 with the UI.
