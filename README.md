# Aether

Sovereign peer-to-peer communication. No servers. No accounts. Just cryptographic identity and direct connections.

Aether is a desktop application for decentralized messaging and voice built on Ed25519 identity and peer-to-peer networking. Your identity lives in your system keychain -- not on someone else's server.

## Architecture

- **Frontend**: Svelte 5 with a terminal-inspired UI
- **Backend**: Rust via Tauri v2
- **Identity**: Ed25519 keypairs stored in the platform keychain (iCloud Keychain on macOS)
- **Networking**: libp2p for peer discovery and NAT traversal (planned)

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- Platform dependencies for [Tauri v2](https://v2.tauri.app/start/prerequisites/)

## Getting Started

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Structure

```
src/                     # Svelte 5 frontend
  lib/
    components/          # UI components
      layout/            # App shell, sidebar, channel list
      setup/             # Identity setup flow
      profile/           # Avatar, profile popover
    stores/              # Reactive state (identity)
src-tauri/               # Rust backend
  src/
    identity/            # Ed25519 key generation & keychain storage
    commands/            # Tauri IPC command handlers
```

## How It Works

On first launch, Aether generates an Ed25519 keypair that becomes your permanent identity. The private key is stored in your operating system's secure keychain (with iCloud sync on macOS). Your public key serves as your address -- share it with others to connect directly.

No email. No phone number. No password. Your keys are your identity.

## License

MIT
