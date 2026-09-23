# ETHGlobal Tokyo 2026 — Submission Draft

Copy the relevant sections into the ETHGlobal submission form. This file is a working draft, not part of the demoed product.

---

## Project title

FamilyProof — Prove Trust, Not Identity

## One-line tagline

A ZK protocol for proving membership in a chosen "Trust Circle" — not bound by blood, marriage, or cohabitation — and enforcing tiered human approval over what an AI Agent can do on your behalf.

## GitHub repo

https://github.com/niikun/family_proof

## Track

Continuity Track (see "Continuity Track note" below for the required pre-event/during-event split).

## Partner prizes to select (up to 3, counts as 1 slot per partner)

- **World: Best Use of IDKit**
- **World: Best Use of World ID for Agents**

Both are a long shot: FamilyProof's World ID integration is a self-contained JS mock, not the real IDKit SDK (see "Known limitations" — this is disclosed up front in the README and demo, not something judges will discover on their own). Selecting these costs nothing (same partner, same slot) and the rest of the stack — real ZK circuit, real on-chain deployment, real AI-agent-authorization flow — is fully genuine. Worth applying; not worth overclaiming.

---

## Description

**FamilyProof proves you're a member of someone's trusted circle — without ever revealing who you are or what secret you know.**

It started as a countermeasure for Japan's "ore-ore" phone impersonation scams (¥113.81 billion in losses, 14,489 confirmed cases in 2025 — National Police Agency). The standard defense is a family password, but that has a fatal flaw: saying a secret out loud to prove your identity exposes it to being overheard, and once it leaks, it's gone for good.

FamilyProof replaces the spoken password with a ZK-SNARK (Groth16 over BN254, circom + Poseidon): a member proves "I hold a secret committed in this Merkle tree" without ever transmitting the secret. An RLN (Rate-Limiting Nullifier) construction means that if a stolen secret *is* reused across two different calls, the two proofs mathematically reveal the secret to the verifier — so reuse self-destructs the leak instead of hiding it.

**From family to Trust Circle.** Building this out, it became clear the underlying primitive is more general than "family": a chosen group of people you trust enough to delegate decisions to — neighbors, friends, caregivers, an AI agent — regardless of blood or cohabitation. `FamilyConstitution.sol` extends the same circuit (reusing its `challenge` public input, no new circuit needed) into **tiered Action Authorization**: an AI Agent proposes an action; low-risk actions execute immediately; higher-risk actions (a large money transfer, changing who's in the circle) require 1–3 human members to each submit a ZK membership proof binding to that specific action ID. The AI Agent holds no leaf in the Merkle tree, so it is *structurally* incapable of producing a valid approval proof for its own proposal — not by contract-level access control, but by the soundness of the ZK system itself.

**What's real, on-chain, right now (World Chain Sepolia):**
- The full ZK pipeline: circom circuit → Rust (`ark-circom`/`arkworks`) witness/proof generation → Solidity Groth16 verifier, with real proofs verified on-chain.
- RLN in production: two proofs with the same secret in one epoch → automatic secret recovery via linear interpolation → `PotentialLeak` event → email notification → new secret issued → Merkle root rotated on-chain.
- `FamilyConstitution.sol` deployed to World Chain Sepolia (`0xf7f344E9399638b69DF158877F1e77a39A5F3D73`), with a full propose → attacker-fails → 2 human approvals (from two different members, producing two distinct nullifiers) → `ActionAuthorized` flow demonstrated end-to-end on real transactions.
- An anonymous, privacy-preserving statistics dashboard aggregating daily detection counts — no addresses, no family roots, no per-event data — published publicly.

**What's a deliberate, disclosed mock:** the "AI voice-clone defense" demo scene uses a JS mock instead of the real World ID SDK, because the real integration needs a signed-request backend (`RP_SIGNING_KEY`) whose signing algorithm isn't well-documented outside the official SDK — judged too risky to get right blind in the time available. This is disclosed in the README and in the pitch itself, not hidden.

**Also real, before the event:** `propose_action` wrapped as an MCP server, so an AI assistant can actually call it as a tool live during the demo — playing the "AI Agent" role for real — instead of a human copy-pasting the suggested `cast send` command. (`approve_action` isn't wrapped as an MCP tool yet; approvals, including the AI Agent's own failed self-approval attempt, are still demonstrated via the CLI.)

**AI-assistance disclosure:** Claude acted only as a coach throughout — reviewing designs, running builds/tests, catching bugs (never fixing them). All Solidity and Rust code was written by the developer, solo, over the course of the project.

---

## Continuity Track note

- **Pre-existing (before Sept 25):** the ZK circuit, RLN, `FamilyRegistry.sol`/`Groth16Verifier.sol`, notification/statistics infrastructure, `FamilyConstitution.sol` with its propose/approve CLIs and real Sepolia deployment, and the MCP-server wrapping of `propose_action` letting an AI assistant actually execute the proposal step via a tool call (`approve_action` remains CLI-only).
- **During the event (Sept 25–27):** final demo rehearsal, video recording, and this submission writeup.

## How it's made (tech stack)

circom 2.2.3 + Poseidon (circuit) · Rust `ark-circom`/`arkworks` (witness/proof generation, BN254/Groth16) · Solidity/Foundry (`Groth16Verifier.sol`, `FamilyRegistry.sol`, `FamilyConstitution.sol`) · World Chain Sepolia (deployment target) · `alloy` + `reqwest` + `tokio` (Rust event-monitoring/notification backend, Resend API) · plain HTML/vanilla JS (demo UI, no build tooling) · `rmcp` (Rust MCP SDK, wraps `propose_action` as a tool; `approve_action` is not yet MCP-wrapped).

## Demo video

[link — add once recorded]

## Live contracts (World Chain Sepolia)

| Contract | Address |
| --- | --- |
| Groth16Verifier | `0x132a7dbd30784d2283b83D96BD45B731AF331c8a` |
| FamilyRegistry | `0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d` |
| FamilyConstitution | `0xf7f344E9399638b69DF158877F1e77a39A5F3D73` |

---

## TODO before final submission

- [ ] Record and link the demo video
- [ ] Paste final text into the ETHGlobal submission form (field names/limits may differ slightly from this draft's section breaks — adjust as needed)
