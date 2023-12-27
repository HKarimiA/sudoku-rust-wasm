<div align="center">

  <h1><code>sudoku-rust-wasm</code></h1>

  <strong>A simple Sudoku game with Rust and WebAssembly, using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>


## About

This project is a simple yet challenging Sudoku game. It generates random Sudoku puzzles while ensuring they are always solvable, providing an engaging and varied experience for players.

## Features

- Random Puzzle Generation: Randomly generates Sudoku puzzles to ensure a unique and enjoyable experience every time.
- Difficulty Levels: Aims to generate puzzles with varying difficulty levels to provide a challenge without sacrificing solvability.
- WebAssembly Integration: Utilizes WebAssembly (Wasm) technology generated with <strong>wasm-pack</strong>, enabling direct usage from JavaScript files for enhanced performance and interaction.

## 🚴 Usage

This project uses WebAssembly to deliver an efficient and seamless Sudoku gaming experience directly within the browser. Simply access the hosted version temporarily available at https://hkarimi.rf.gd to enjoy solving Sudoku puzzles online.

### 🐑 Clone the repository

```
git clone https://github.com/HKarimiA/sudoku-rust-wasm.git
cd sudoku-rust-wasm
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🎁 Install the dependencies

```
cd www
npm install
```

## 🔋 Serving Locally

```
npm run start
```

