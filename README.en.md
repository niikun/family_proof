# FamilyProof

🌐 [日本語版](README.md)

**Prove trust, not identity.**

> **Family is not only about blood.**
> A relationship you trust enough to delegate authority to can be "family" too.

FamilyProof is a protocol that implements a **Trust Circle** — not bound by blood, marriage, or cohabitation — using ZK proofs and smart contracts.

Living alone doesn't have to mean facing important decisions alone.

Family who live far away, friends, neighbors, caregivers.
And in the future, an **AI Agent** too can act as a delegate supporting a Trust Circle member.

FamilyProof verifies, without revealing any secret or personal information:

> Not **"who is this person?"**, but
> **"is this person a member of the Trust Circle this individual chose, with the authority to approve this Action?"**

> To be precise, what is cryptographically proven is not "trust" itself, but "membership in a Trust Circle the individual chose beforehand." *"Prove trust, not identity"* is the philosophical tagline; the technically accurate claim is closer to *"Prove membership, protect identity."*

---

## From Family to Trust Circle

Traditional identity verification:

```text
"Is this person family?"
```

FamilyProof:

```text
"Did the Trust Circle this person chose
 approve this Action?"
```

A Trust Circle is not a fixed "family attribute."

```text
You
 ├─ Family
 ├─ Friends
 ├─ Neighbors
 ├─ Caregivers
 └─ AI Agent
```

Membership can change over time.

---

## AI is an Agent, not a Human

An AI Agent does not hold the same authority as a human Trust Circle member.

Low-risk Actions can be executed by the AI alone, but the higher the risk, the more human approvals are required.

| Action                              | Tier | Required approval |
| ------------------------------------ | ---: | -----------------: |
| Scheduling / reminders               |    0 |               0 humans |
| Small payments                       |    1 |          1 human |
| Large transfers / important decisions |    2 |          2 humans |
| Trust Circle membership changes      |    3 |          3 humans |

**Instead of granting the AI authority, humans cryptographically control how much can be delegated to it.**

---

## Why ZK?

Trust Circle members never have to reveal their `secret`.

```text
secret
   ↓
Poseidon
   ↓
Merkle Tree
   ↓
Merkle Root
   ↓
Groth16 Proof
```

The prover proves only that:

> "I am a member of this Trust Circle."

Real names, secrets, family composition and other information are never disclosed beyond what's necessary.

### Cryptographic primitives

* **Merkle Tree** — represents membership
* **Poseidon** — ZK-circuit-friendly hash
* **Groth16 / BN254** — membership proof
* **RLN** — detects and revokes reused secrets
* **Smart Contract** — verifies Action policy and approval state

> **World ID** (Unique Human / Sybil resistance) has been designed and validated as a production integration, but the demo substitutes a self-contained JS mock (not implemented — see "Known Limitations").

---

## RLN: Reusing a Stolen Secret Backfires

FamilyProof also detects reuse of a stolen `secret`.

If the same secret is used against different challenges within the same `epoch`:

```text
(x1, y1)
(x2, y2)
   ↓
2-point interpolation
   ↓
secret recovery
   ↓
revoke
```

In other words:

> **The more a stolen secret is used, the worse it gets for the attacker.**

Once a leak is detected, a new secret can be issued and the Merkle Root updated.

---

## Action Authorization

An AI Agent (or similar) proposes an Action.

```text
AI Agent
    │
    │ proposeAction(actionId, tier)
    ▼
 Action
    │
    ├─ Tier 0 → executes immediately
    ├─ Tier 1 → 1 human
    ├─ Tier 2 → 2 humans
    └─ Tier 3 → 3 humans
```

Human Trust Circle members submit a ZK proof that includes `actionId` as the challenge.

```text
secret + Merkle path + actionId
                ↓
           Groth16 Proof
                ↓
      membership + approval
```

Once enough approvals are collected, `ActionAuthorized` is emitted and the Action can execute.

Because the AI Agent itself is not in the Trust Circle's Merkle Tree, it cannot generate a valid proof for human approval. This entire flow (propose → 2 approvals → `ActionAuthorized`) has been demonstrated end-to-end on-chain (see "Current Status").

---

## Architecture

```text
      World ID (designed, demo is a mock)
              Unique Human
                    │
        ┌───────────┴───────────┐
        │                       │
   Trusted Human            AI Agent
        │                       │
        └───────────┬───────────┘
                    │
               Trust Circle
                    │
              Merkle Root
                    │
             ZK Membership
                    │
          Family Constitution
                    │
          Action Authorization
                    │
                   RLN
                    │
              World Chain
```

---

## Technical Stack

| Layer            | Technology                          |
| ---------------- | ------------------------------------ |
| Circuit          | circom 2.2.3                         |
| Hash             | Poseidon                             |
| ZK Proof         | Groth16 / BN254                      |
| Rust             | `ark-circom` / `arkworks`            |
| Smart Contract   | Solidity / Foundry                   |
| Blockchain       | World Chain Sepolia                  |
| Human uniqueness | World ID (designed, demo is a mock)  |
| Backend / CLI    | Rust / `alloy` / `tokio`             |
| Demo UI          | HTML / Vanilla JS                    |

---

## Current Status

### ✅ Implemented & verified on World Chain Sepolia (built before the event, through Sept 24)

Core ZK membership + leak detection:

* Merkle membership proof / Poseidon commitment
* Groth16 proof generation / verification (Rust witness/proving pipeline)
* RLN nullifier / same-epoch double-use detection / secret recovery & revocation / Merkle Root rotation
* `Groth16Verifier.sol` / `FamilyRegistry.sol`
* Notification & anonymous-statistics infrastructure (`notifier.rs`, `stats.html`)

Family Constitution (tiered Action Authorization):

* `FamilyConstitution.sol`: `proposeAction` / ZK-backed `approveAction` / tier-based auto-execution / `ActionAuthorized` / double-approval prevention / challenge-mismatch protection / agent-only proposal rights. All unit tests passing
* `propose_action.rs` / `approve_action.rs` (Action proposal/approval CLIs)
* **Deployed to production World Chain Sepolia**: `FamilyConstitution` = `0xf7f344E9399638b69DF158877F1e77a39A5F3D73`
* Tier 0 (instant execution) / Tier 1 (1 approval) / Tier 2 (2 approvals, from two different members producing two different nullifiers) demonstrated end-to-end on-chain
  (tx: propose `0xc76a25bfad576afa5605871b650e145f3d5ddd0ea9a2d66c68f44d4a3407ab45` / approval 1 `0xa421b7f1b18c082c5f4f1df5da8f123df8f580fd32d0c71a72d7fa77075c4b1e` / approval 2 `0xdfee0dd25f7476912714fd5b33395d1854fbf7d8e4291841d6b6c589a03d30e0`)

### 🚧 Built during the event (Sept 25–27, planned)

* Wrap `propose_action` / `approve_action` as an MCP server so Claude itself, playing the AI Agent role, can send on-chain transactions via real tool calls (currently a human copy-pastes the `cast send` command Claude suggests)

### 🔭 Future work

* Real World ID SDK integration (IDKit + RP-signing backend)
* AI-driven autonomous risk/tier judgment
* Dynamic governance letting the Trust Circle itself set tier→threshold mappings
* Cryptographic identity for the AI Agent / ZK-provable delegated capability ([docs/SPEC.md §11.8](docs/SPEC.md))
* MPC-based custody of the AI Agent's signing key ([docs/SPEC.md §11.7](docs/SPEC.md))

---

## Repository

```text
circuits/       circom circuits / proving artifacts
src/            Rust implementation
src/bin/        CLI tools
contracts/      Solidity / Foundry
docs/
  SPEC.md       detailed specification
  HANDOFF.md    development log
  PITCH.md      presentation / demo script
stats.html      anonymous statistics dashboard
```

---

## Setup

### Circuit

```bash
cd circuits
npm ci
circom main.circom --r1cs --wasm --sym -l node_modules
node scripts/build_input.js
```

### Rust

```bash
cd ..
cargo test
cargo run
```

Action Authorization demo (propose/approve against `FamilyConstitution`):

```bash
cargo run --bin propose_action "<description>" <tier>   # tier: 0-3
cargo run --bin approve_action "<description>" <leaf_idx>
```

### Solidity

```bash
cd contracts
forge build
forge test -vv
```

---

## Live Contracts

### World Chain Sepolia

| Contract | Address |
| --- | --- |
| Groth16Verifier | `0x132a7dbd30784d2283b83D96BD45B731AF331c8a` |
| FamilyRegistry | `0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d` |
| FamilyConstitution | `0xf7f344E9399638b69DF158877F1e77a39A5F3D73` |

Anonymous statistics dashboard (daily counts only — never who verified or was flagged, or when): `stats.html` (open locally, or the [hosted version](https://niikun.net/family_proof/))

---

## Why FamilyProof?

FamilyProof started as a solution to impersonation scams:

> **"Can I prove I'm family without ever saying a secret password?"**

Impersonation phone scams ("ore-ore sagi") are a persistent problem in Japan — 14,489 confirmed cases and ¥113.81 billion in losses in 2025 (National Police Agency, final figures), averaging ¥5.236 million per completed case. Over the past five years, 33.3% of people received a call believed to be a scam attempt, and 1.5% of them actually suffered a loss (Japan's Research and Training Institute, Ministry of Justice, March 2025). Traditional countermeasures (a fixed shared password) have a fundamental contradiction: the very act of speaking a secret to verify identity exposes it to eavesdropping or recording — and once leaked, it can never be used again.

From there, the question broadened:

> **"Is family really only about blood, after all?"**

What FamilyProof aims to build is a new kind of Trust Circle for the age of AI.

**Identity → Membership → Authorization**

Instead of exposing anyone's identity, it cryptographically verifies
**what can be delegated within this relationship.**

---

## Known Limitations

* RLN's `epoch = 1 hour / limit = 1` is reused, as the same instance, for Action Authorization. If the same member approves multiple Actions within one hour, they may unintentionally expose their own secret (production use would require a separate RLN instance per use case)
* The AI Agent has no cryptographic identity of its own. Today it is only an access-control-level permission — "a specific EOA allowed to call `proposeAction`"
* Family Constitution's tier→threshold mapping is a contract constant; self-governance by the Trust Circle is not implemented
* The demo's World ID integration (AI voice-clone defense) is a proof-of-concept mock, not the real SDK. The production design has been validated (RP-signing + v4 verify API) but not implemented
* Groth16/BN254 is theoretically breakable by Shor's algorithm (migrating to a post-quantum-secure proof system is out of scope)

---

## Continuity Track / AI Usage Policy

This project is submitted to the **Continuity Track** of ETHGlobal Tokyo 2026. The ZK circuit, RLN, on-chain Registry, Family Constitution, and notification/statistics infrastructure are the pre-existing part, built before the event started (through Sept 24). The MCP server integration (letting Claude itself operate the AI Agent role via real tool calls) is the new part planned to be built during the event (Sept 25–27) — the two are clearly distinguished.

Claude (AI) acted only as a coach — reviewing designs, running builds/tests, and confirming behavior. **All Solidity/Rust code was written by the developer.** The entire implementation is the work of a solo developer.

---

## Author

niikun ([GitHub](https://github.com/niikun/family_proof))
