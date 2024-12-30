# Shady-Life

A WebGPU implementation of Particle Life simulation, running in the browser via WebAssembly. Built with Rust and wgpu.

## Description

Shady-Life is a particle life simulation that demonstrates emergent behavior through simple interaction rules between different types of particles. The simulation runs entirely on the GPU using compute shaders, with the results rendered in real-time in your web browser.

## Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- wasm-pack (install with `cargo install wasm-pack`)
- A WebGPU-compatible browser (Chrome Canary or recent versions of Chrome/Edge)

## Project Structure

```
shady-life/
├── client/             # WebAssembly client code
│   ├── src/           # Rust source files
│   ├── index.html     # Web page template
│   └── Cargo.toml     # Client dependencies
├── server/            # Actix web server
│   ├── src/          # Server source files
│   └── Cargo.toml    # Server dependencies
└── Cargo.toml         # Workspace configuration
```

## Building and Running

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/shady-life.git
   cd shady-life
   ```

2. Run the development server:
   ```bash
   cargo run -p particle-life-server
   ```

3. Open your browser and navigate to:
   ```
   http://localhost:8080
   ```

## Development

The project uses a workspace with two main components:

- `client`: The WebAssembly client that runs the particle simulation
- `server`: An Actix web server that serves the client files

When you run the server, it automatically:
1. Builds the WebAssembly client
2. Copies the necessary files to the distribution directory
3. Serves the files through the Actix web server

## Browser Compatibility

This project requires a browser with WebGPU support. Currently, this includes:
- Chrome Canary (with appropriate flags)
- Recent versions of Chrome/Edge
- Other browsers may require enabling WebGPU flags

## License

[Your chosen license]

## Acknowledgments

This project is inspired by:
- Particle Life simulations
- The Rust and WebGPU communities