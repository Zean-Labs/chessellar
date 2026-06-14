  
# Chessellar

**Chessellar** is an on-chain chess application built on the **Stellar blockchain** using **Soroban smart contracts**.

It brings classic PvP chess into Web3 by allowing players to create matches, compete against each other, and use smart contract escrow for secure XLM-based wagers.

> On-chain chess powered by Stellar, Soroban, and secure wager settlement.

---

## Overview

Chessellar is a decentralized chess game designed to explore how traditional strategy games can be enhanced with blockchain infrastructure.

The project combines a modern web-based chess interface with Soroban smart contracts to support transparent match creation, wager handling, and game resolution.

The goal is to create a smooth on-chain gaming experience where players can compete, verify outcomes, and settle rewards without relying on a fully centralized system.

---

## Why Chessellar?

Most online chess platforms are centralized. They control the game records, player interactions, and any value-based competition.

Chessellar introduces a Web3 model where:

- Match data can be tracked transparently
- Player wagers can be secured through smart contracts
- Game outcomes can be resolved in a trust-minimized way
- Players can interact using Stellar wallets
- Low-cost blockchain infrastructure makes on-chain gaming more practical

---

## Key Features

- **PvP Chess Matches**  
  Players can compete against each other in chess games.

- **Local Move Validation**  
  Chess moves are validated in the frontend using `chess.js`.

- **Soroban Smart Contracts**  
  Core on-chain logic is built with Rust and Soroban.

- **XLM Wager Escrow**  
  Wagers can be locked in smart contracts and released after game resolution.

- **Transparent Game State**  
  Match-related data can be tracked on the Stellar ledger.

- **Modern Web Interface**  
  The frontend uses a premium dark theme with a clean and responsive UI.

- **Stellar Wallet Integration**  
  Freighter wallet integration is planned for connecting players to the Stellar ecosystem.

---

## Project Structure

```txt
chessellar/
├── frontend/      # React + Vite + TypeScript frontend
├── contract/      # Rust + Soroban smart contracts
└── README.md      # Project documentation
````

---

## Tech Stack

### Frontend

* React
* Vite
* TypeScript
* Tailwind CSS
* chess.js

### Smart Contract

* Rust
* Soroban SDK
* Stellar CLI

### Blockchain

* Stellar
* Soroban
* XLM

### Wallet

* Freighter Wallet planned

---

## How It Works

1. A player creates a chess match.
2. Another player joins the match.
3. Both players can lock XLM wagers through the Soroban smart contract.
4. The game is played through the frontend interface.
5. Moves are validated locally using chess logic.
6. Match state and wager logic are handled through the contract layer.
7. The winner receives the escrowed reward after game resolution.

---

## Getting Started

### Prerequisites

Make sure you have the following installed:

* Node.js
* npm or yarn
* Rust
* Stellar CLI
* Git

---

## Running the Frontend

Move into the frontend directory:

```bash
cd frontend
```

Install dependencies:

```bash
npm install
```

Start the development server:

```bash
npm run dev
```

The frontend should be available at:

```txt
http://localhost:5173
```

---

## Working With the Smart Contract

Move into the contract directory:

```bash
cd contract
```

Build the Soroban contract:

```bash
stellar contract build
```

Run tests if available:

```bash
cargo test
```

---

## Deployment

### Frontend

The frontend is deployed here:

```txt
https://chessellar.vercel.app/
```

### Smart Contract

Soroban testnet deployment details will be added after deployment.

```txt
Contract ID: To be updated
Network: Stellar Testnet
```

---

## Current Status

Chessellar is currently in active development.

Completed:

* Frontend MVP
* Chess UI
* Local chess move validation
* Monorepo project structure
* Soroban contract foundation
* Project documentation

In progress:

* Freighter wallet integration
* Soroban testnet deployment
* Full wager escrow flow
* Match resolution improvements
* User testing

---

## Roadmap

* [ ] Complete Stellar wallet integration
* [ ] Deploy Soroban contract to testnet
* [ ] Enable full XLM wager escrow
* [ ] Improve game resolution logic
* [ ] Add player profiles
* [ ] Add match history
* [ ] Add leaderboard system
* [ ] Improve mobile responsiveness
* [ ] Run early Web3 gamer testing
* [ ] Prepare mainnet-ready contract version

---

## Contribution

Contributions are welcome.

You can contribute by:

* Improving the frontend UI
* Fixing bugs
* Writing tests
* Improving the Soroban contract
* Adding wallet integration
* Improving documentation
* Suggesting new gaming features

Before contributing, please open an issue or check existing issues to understand what needs to be worked on.

---

## Development Guidelines

To keep the project clean and useful:

* Write clear commit messages
* Keep pull requests focused
* Add tests where necessary
* Avoid unrelated changes
* Explain major changes in your PR description
* Make sure the project runs before submitting a PR

Low-quality, duplicated, unrelated, or spam pull requests will not be accepted.

---

## Maintainers

Built and maintained by **Zean Labs**.

Zean Labs is a blockchain infrastructure and product engineering lab building protocol tools, Web3 infrastructure, and practical on-chain applications.

---
## License

This project is licensed under the MIT License.

See the [LICENSE](./LICENSE) file for details.
