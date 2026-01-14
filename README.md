# VeritasNodes: Decentralized Infrastructure Audit Protocol 🇮🇳

**VeritasNodes** is a high-performance DePIN (Decentralized Physical Infrastructure Network) protocol built on **Solana**. It bridges the "Trust Gap" in urban infrastructure by turning regular citizen-owned devices into verifiable, immutable auditors of Air Quality, Temperature, and Road Integrity.



## 🎯 The Vision
Modern cities rely on sparse, centralized sensors that are expensive and prone to data manipulation. VeritasNodes decentralizes this "Truth" by:
- **Scaling Resolution:** Moving from 40 city-wide sensors to 40,000 street-level nodes.
- **Ensuring Integrity:** Using the Solana blockchain to lock data hashes, making them immune to redaction.
- **Incentivizing Citizens:** Providing a sustainable "side-hustle" for regular people to help protect their neighborhoods.

## 🚀 Current Milestone: Level 3 (The Economic Foundation)
This repository contains the core Rust program (Smart Contract) and the TypeScript validation suite for the protocol's foundation.

### **Implemented Features**
- **Level 1: Identity Management** – Implemented **Program Derived Addresses (PDAs)** to map physical hardware (IoT sensors/phones) 1:1 with on-chain "Digital Twins."
- **Level 2: Data Integrity** – Lightweight "Heartbeat" logic that stores 32-byte SHA-256 data hashes. This proves data existence and origin without high gas costs.
- **Level 3: Reward Economy** – An automated claim system that allows nodes to earn for "Proof-of-Sensing."
  - **Atomic State Resets:** Prevents double-claiming of rewards through strict state transitions.
  - **Resource Management:** Optimized account closing logic to refund SOL rent to users upon deactivation.

## 🛠 Tech Stack
- **Smart Contract:** Rust & Anchor Framework
- **Blockchain:** Solana (Localhost/Devnet)
- **Testing:** TypeScript (Mocha/Chai)
- **Identity:** PDA-based Hardware Authentication

## 📦 Getting Started

### **Installation**
1. **Clone the repository:**
   ```bash
   git clone [https://github.com/YOUR_USERNAME/veritasnodes-protocol.git](https://github.com/YOUR_USERNAME/veritasnodes-protocol.git)
   cd veritasnodes-protocol
2. **Build the program:**
   ```bash
   anchor build
3. **Test The Program**
5. ```bash
   anchor test

## 🗺 Roadmap (The Veritas Evolution)

### **[x] Phase 1: The Foundation (Current)**
- **Level 1: Identity** – PDA-based device registration for unique hardware-to-wallet mapping.
- **Level 2: Integrity** – SHA-256 data hashing on-chain to ensure immutable "Proof-of-Sensing."
- **Level 3: Economy** – Automated reward loop with atomic state resets to prevent double-claiming.

### **[ ] Phase 2: Multimodal Expansion (Level 4)**
- **Heat Island Mapping:** Integrating `i16` temperature data streams to identify urban thermal hotspots.
- **Structural Auditing:** Leveraging device accelerometers to passively detect and map road potholes and cracks using vibration analysis.

### **[ ] Phase 3: Economic Gravity & Security (Level 5-6)**
- **Stake-Weighted Security:** Implementing a security deposit system (Staking) for node activation.
- **Anti-Fraud Slashing:** Developing automated "Slashing" logic to penalize nodes submitting malicious or spoofed data.

### **[ ] Phase 4: Mass Adoption / Web2.5 (Level 7-9)**
- **Invisible Blockchain:** Integration of **Social Logins** (Google/Phone OTP) via Embedded Wallets.
- **Frictionless UX:** Implementing **Gasless Transactions** and **UPI-to-Stake** on-ramps specifically designed for the Indian user base.
