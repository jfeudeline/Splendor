# 0007. Front-end stack: React + WASM, then PWA, then Tauri

- Status: Proposed (takes effect in phase 8; to be confirmed then)
- Date: 2026-10-15

## Context

The game must eventually be playable on web and mobile, solo against AIs (ideally
offline) and online. The author already knows TypeScript/React for the web, has no
mobile experience, and wants to deepen Rust skills.

Options considered for the UI:

| Option | Pros | Cons |
|---|---|---|
| Rust UI (Leptos / Dioxus) | One language, shared types | Younger ecosystem, mobile support still maturing |
| **React + TS, engine in WASM** | Mature ecosystem, known by the author, great for animations and platform pages | Two languages; types must be synced |
| Bevy | Rich rendering | Overkill for a card game, poor fit for platform UI (lobby, menus) |

Options for mobile:

| Option | Pros | Cons |
|---|---|---|
| PWA | Zero extra code, offline | No app stores, limited push on iOS |
| Capacitor | Very mature on mobile, many official plugins | No Rust |
| **Tauri 2** | Rust shell, desktop for free, consistent with the project | Smaller mobile plugin ecosystem |
| React Native | Native UI | UI rewrite; WASM poorly supported by Hermes |

## Decision

1. Web front-end in **React + TypeScript** (Vite), calling the Rust engine and AIs
   compiled to **WASM**; AIs run in a Web Worker. Rust types are exported to TS
   (e.g. with `ts-rs` or `tsify`).
2. Responsive, touch-first design, shipped as a **PWA** for offline solo play.
3. **Tauri 2** to package desktop and mobile apps; **Capacitor** as fallback if a mobile
   need (push notifications, store features) is blocked with Tauri.

## Consequences

- The engine must stay WASM-compatible (ADR 0003).
- The web app is the same code in browser, PWA, Tauri and Capacitor: switching packager
  is cheap.
- Most of the Rust learning happens in the engine, AI, WASM bindings and server, which
  are independent of the packaging choice.
- This decision only takes effect from phase 8; it can be revisited by then.
