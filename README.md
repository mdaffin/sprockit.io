# sprockit.io

## Requirements
The following are needed to build/develop sprockit.io.

- [Node.js](https://nodejs.org/en/) and [Yarn](https://yarnpkg.com/en/docs/getting-started)
- [Rust](https://rustup.rs/)

Additionally if you use VS Code follow the guide on [Setting up VSCode](Setting-up-VSCode).

## Quick Start

### The API server

The server is written in rust and located in `$project/server` you can compile and run it with.

```bash
cd server
cargo install
cargo run
```

This will start a http server on http://localhost:4000.

### The Web UI

With the API server running you can start the UI in `$project/web-ui`.
```bash
cd web-ui
yarn install
yarn run dev
```

This will start another web server on http://localhost:3000 with the UI.
