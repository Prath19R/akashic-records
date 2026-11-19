# 🏛️ Akashic Records

> Personal Systems Engineering & Backend Portfolio of **Prathmesh Ranadive**  
> Built with **100% Rust** using [Leptos](https://leptos.dev) (CSR mode), WebAssembly, [Trunk](https://trunkrs.dev), and Tailwind CSS.

Hosted for free on **GitHub Pages**: [prath19r.github.io/akashic-records](https://prath19r.github.io/akashic-records/)

---

## ⚡ Tech Stack

- **Core Framework:** [Leptos 0.7+](https://leptos.dev) (CSR mode)
- **Language & Runtime:** Rust (`wasm32-unknown-unknown`) compiled to WebAssembly
- **WASM Bundler:** [Trunk](https://trunkrs.dev)
- **Styling:** Tailwind CSS with Dark/Zinc Minimalist aesthetic (`#09090b` palette)
- **CI/CD:** Automated GitHub Actions workflow (`.github/workflows/deploy.yml`)
- **Embedded AI Assistant:** [Athena](https://github.com/Prath19R/athena) embedded via responsive iframe widget and standalone URL

---

## 🚀 Local Development

### Prerequisites
- Rust & Cargo (`rustup default stable`)
- WASM target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk` (or prebuilt binary)
- Node.js & npm (for Tailwind CSS)

### Run
```bash
# Install Tailwind CSS dependency
npm install

# Start Trunk development server
trunk serve --port 8080
```
Open [http://localhost:8080](http://localhost:8080) in your browser.

---

## 📦 Deployment

Every push to `main` triggers `.github/workflows/deploy.yml` which:
1. Sets up the Rust toolchain + `wasm32-unknown-unknown`
2. Caches Rust build artifacts with `Swatinem/rust-cache`
3. Runs `npm ci` for Tailwind CSS
4. Compiles release WASM bundle via `trunk build --release`
5. Deploys to GitHub Pages via `actions/deploy-pages`
