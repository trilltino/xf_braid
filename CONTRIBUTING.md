# Contributing to XF Braid

## Development Setup

1. Install prerequisites:
   - Rust 1.90+ with `wasm32-unknown-unknown` target
   - Node.js 20+ for Tailwind CSS
   - Trunk for WASM builds: `cargo install trunk`
   - cargo-leptos: `cargo install cargo-leptos`

2. Clone repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/xf_braid.git
   cd xf_braid
   ```

3. Copy environment template:
   ```bash
   cp .env.example .env
   ```

4. Start development servers:
   ```bash
   # Leptos site
   leptos-local.bat
   
   # XFMail chat (separate terminal)
   xfmail-local.bat
   ```

## Docker Development

Build and test locally:
```bash
docker-compose up --build
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` to catch issues
- Follow Rust API Guidelines

## Testing

```bash
# Run all tests
cargo test --workspace

# Specific crate
cd xfmail/backend
cargo test --features ssr
```

## Pull Request Process

1. Create a feature branch
2. Make your changes
3. Run tests and linters
4. Submit PR with clear description
5. Wait for CI checks to pass

## Questions

Open an issue for questions or discussion.
