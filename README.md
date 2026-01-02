# XF Braid

A monorepo containing two distinct Rust applications:

1.  **XFMail**: A peer-to-peer real-time chat application powered by the **BraidFS** protocol (located in `/xfmail`).
2.  **XF Braid Website**: A server-side rendered marketing/landing site built with **Leptos** (located in `/frontend` and `/backend`).

---

## 1. XFMail (The BraidFS Application)
**Location:** `/xfmail`

XFMail is a demonstration of decentralized, offline-first collaboration. It allows users to chat and collaborate in real-time, with data synchronized seamlessly between clients and the server using a custom implementation of the Braid-HTTP protocol.

### How it Works: BraidFS Integration
At the heart of XFMail is **BraidFS**, a custom engine that enables peer-to-peer file synchronization.

#### A. Protocol Implementation
*   **Custom Client**: XFMail uses a custom client (`braid_fetch`) that implements the [Braid-HTTP](https://github.com/braid-org/braid-spec) protocol from scratch.
*   **Real-Time Subscriptions**: Instead of polling, the client sends `Subscribe: true` headers. This opens a persistent connection where the server pushes updates (patches) the moment they happen.
*   **Braid Headers**: It strictly adheres to the protocol using `Version` (state vector), `Parents` (causality), and `Patches` (deltas) to manage consistency.

#### B. The Hybrid Storage Engine
To act as a "Filesystem", XFMail uses a storage abstraction (`BraidStorage`) that works identically on Desktop and Web:
*   **Desktop (Native)**: In the native app, it writes directly to your hard drive (e.g., `~/http`). You can see the chat messages as files on your disk.
*   **Web (WASM)**: In the browser, it uses a **Virtual File System** on top of `LocalStorage`. This means the web app behaves exactly like the desktop app, maintaining its own "files" inside the browser sandbox.

#### C. Smart Synchronization (Diffing)
*   **Efficiency**: XFMail doesn't send whole files. It uses the **Myers diff algorithm** (via `dissimilar`) to calculate the exact difference (Insert/Delete) between the local and remote state.
*   **Conflict-Free**: A persistent `VersionStore` tracks the "Version Vector". When you send a message, it includes the `Parents` (the version you last saw). If two people speak at once, the server uses this info to merge the timeline without losing data.
*   **Loop Avoidance**: An internal `PendingWrites` registry prevents local file watchers from triggering infinite sync loops when applying remote updates.

---

## 2. XF Braid Website (Leptos)
**Locations:** `/frontend` (UI), `/backend` (Server)

This is the public-facing website for the project. It uses a modern SSR (Server-Side Rendering) architecture.

*   **Frontend (`/frontend`)**: Built with **Leptos**, a Rust web framework. It compiles to WebAssembly for interactivity (hydration) but is rendered on the server first for SEO and speed. It uses **Tailwind CSS** for styling.
*   **Backend (`/backend`)**: An **Axum** web server that handles the SSR process, serves the static files, and manages user authentication (JWT) and SQLite database connections (`data.db`).

---

## Project Structure Map

*   `xf_braid/`
    *   `backend/`: **(Website)** The Axum server for the Leptos site.
    *   `frontend/`: **(Website)** The Leptos UI code.
    *   `xfmail/`: **(Chat App)** The workspace for the chat application.
        *   `backend/`: The chat server (Braid-HTTP capabilities).
        *   `frontend-egui/`: The chat client (WASM and Native compatible).
        *   `shared/`: Common code shared between chat client and server.
    *   `references/`: Reference implementations and specs.

## Deployment

The project uses Docker to package both applications into a single deployable image.
*   **Dockerfile.leptos**: Builds the Leptos site AND compiles the XFMail WASM client, serving the chat app under the `/xfmail` path.

```bash
# Deploy to Fly.io
fly deploy
```

## Local Development

### Prerequisites
- Rust (nightly)
- Node.js & NPM (for Tailwind)
- `cargo install trunk` (for WASM)
- `cargo install cargo-leptos` (for the website)

### Running XFMail (Chat)
```bash
# Backend
cd xfmail/backend
cargo run --features ssr

# Frontend (Native)
cd xfmail/frontend-egui
cargo run --features native

# Frontend (Web)
cd xfmail/frontend-egui
trunk watch --features web
```

### Running the Website
```bash
leptos-local.bat
# or
cargo leptos watch
```
