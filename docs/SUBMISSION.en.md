# ETHGlobal Tokyo 2026 — Submission Draft

Copy the relevant sections into the ETHGlobal submission form. This file is a working draft, not part of the demoed product.

**Deadline: Sept 27, 9:00 JST (no late submissions).**

---

## Project title

FamilyProof — Prove Trust, Not Identity

## Short description (≤100 chars, if the form asks for one)

ZK proof of trust + World ID: stop AI voice-clone scams and keep AI agents under human approval.

## One-line tagline

A ZK protocol for proving membership in a chosen "Trust Circle" — not bound by blood, marriage, or cohabitation — and enforcing tiered human approval over what an AI Agent can do on your behalf.

## GitHub repo

https://github.com/niikun/family_proof

## Track

Continuity Track (see "Continuity Track note" below for the required pre-event/during-event split).

## Partner prizes to select (up to 3, counts as 1 slot per partner)

- **World: Best Use of IDKit**
- **World: Best Use of World ID for Agents**

FamilyProof now uses the real IDKit SDK, built during the event: an RP-signing backend (`@worldcoin/idkit-server`), proofs generated in the real World App, and server-side verification through the World v4 verify API. "Best Use of IDKit" is the natural fit. "World ID for Agents" is more of a stretch: World ID guards the human side (voice-clone defense), while AI-Agent approvals go through the ZK Trust Circle, not World ID. Selecting both costs nothing (same partner, same slot).

---

## Description

**FamilyProof proves you're a member of someone's trusted circle — without ever revealing who you are or what secret you know.**

It started as a countermeasure for Japan's "ore-ore" phone impersonation scams (¥113.81 billion in losses, 14,489 confirmed cases in 2025 — National Police Agency). The standard defense is a family password, but that has a fatal flaw: saying a secret out loud to prove your identity exposes it to being overheard, and once it leaks, it's gone for good.

FamilyProof replaces the spoken password with a ZK-SNARK (Groth16 over BN254, circom + Poseidon): a member proves "I hold a secret committed in this Merkle tree" without ever transmitting the secret. An RLN (Rate-Limiting Nullifier) construction means that if a stolen secret *is* reused across two different calls, the two proofs mathematically reveal the secret to the verifier — so reuse self-destructs the leak instead of hiding it.

**From family to Trust Circle.** Building this out, it became clear the underlying primitive is more general than "family": a chosen group of people you trust enough to delegate decisions to — neighbors, friends, caregivers, an AI agent — regardless of blood or cohabitation. `FamilyConstitution.sol` extends the same circuit (reusing its `challenge` public input, no new circuit needed) into **tiered Action Authorization**: an AI Agent proposes an action; low-risk actions execute immediately; higher-risk actions (a large money transfer, changing who's in the circle) require 1–3 human members to each submit a ZK membership proof binding to that specific action ID. The AI Agent holds no leaf in the Merkle tree, so it is *structurally* incapable of producing a valid approval proof for its own proposal — not by contract-level access control, but by the soundness of the ZK system itself.

**What's real, on-chain, right now (World Chain Sepolia):**
- The full ZK pipeline: circom circuit → Rust (`ark-circom`/`arkworks`) witness/proof generation → Solidity Groth16 verifier, with real proofs verified on-chain.
- RLN, running end-to-end on World Chain Sepolia: two proofs with the same secret in one epoch → automatic secret recovery via linear interpolation → `PotentialLeak` event → email notification → new secret issued → Merkle root rotated on-chain.
- `FamilyConstitution.sol` deployed to World Chain Sepolia (`0xf7f344E9399638b69DF158877F1e77a39A5F3D73`), with a full propose → attacker-fails → 2 human approvals (from two different members, producing two distinct nullifiers) → `ActionAuthorized` flow demonstrated end-to-end on real transactions.
- An anonymous, privacy-preserving statistics dashboard aggregating daily detection counts — no addresses, no family roots, no per-event data — published publicly.

**Built during the event — real World ID integration (AI voice-clone defense):** a shared passphrase alone fails against an attacker with an AI-cloned voice. In `worldid/`, the parent picks a passphrase on the spot and says it over the phone; the caller proves with World ID (IDKit, `signal` = the passphrase heard on the phone), and the server checks three things: the proof is valid according to the World v4 verify API (a real, Orb-verified human), the proof's `signal_hash` matches the verifier's passphrase, and the proof's `nullifier` belongs to a registered family member. An AI on its own can't produce a World ID proof at all; a human scammer using a voice clone can, but isn't a registered family member — the right passphrase from the wrong person is rejected, and because the passphrase is a fresh per-call challenge baked into the proof, old proofs can't be replayed. Verified end-to-end on a real phone: accept, wrong passphrase, and non-member (simulated by removing the caller's nullifier from the allowlist, and disclosed as such in the video). The pre-event JS placeholder page (fake `nullifier_hash`) has been deleted from the repository — every World ID check in the demo goes through the real World App and World's v4 verify API. Limitation: the registered-member list is an off-chain allowlist of nullifiers, not yet linked to the on-chain `FamilyRegistry`.

**Also real, before the event:** `propose_action` wrapped as an MCP server, so an AI assistant can actually call it as a tool live during the demo — playing the "AI Agent" role for real — instead of a human copy-pasting the suggested `cast send` command. (`approve_action` isn't wrapped as an MCP tool yet; approvals, including the AI Agent's own failed self-approval attempt, are still demonstrated via the CLI.)

**AI-assistance disclosure (file by file):**
- **Code — written by the developer, solo:** the circom circuit, Rust (`src/`), Solidity (`contracts/`), and the World ID verification logic (`worldid/server.js`, plus the IDKit call and verification flow in `voice_challenge.html`). Claude acted as a coach — design discussion, code review, pointing out bugs, running builds/tests — and did not write this code; the developer made every fix.
- **Code exceptions:** during the event, at the developer's request, Claude edited part of the World ID demo UI in `worldid/public/voice_challenge.html` (explanatory text, panel order, step numbering, proof-result summary display) switched `.mcp.json` to a relative path, and deleted the practice page (`worldid/public/index.html`) and its practice-only endpoint (`/api/verify-proof`) as pre-submission cleanup.
- **Documentation — mostly written/edited by Claude:** `README.md` (English), `README.ja.md` (Japanese), `docs/` (SPEC, PITCH, SUBMISSION, HANDOFF), and `demo/narration_script.md`, drafted from conversations with the developer, who reviewed them and made the decisions.
- **Demo video — editing assisted by Claude:** all screen recordings were captured by the developer in the real environment. Claude generated the title cards, progress badges, and captions (Python/Pillow) and did the cutting/assembly with ffmpeg. The narration is the developer's own voice (no AI voice).

---

## Continuity Track note

- **Pre-existing (before Sept 25):** the ZK circuit, RLN, `FamilyRegistry.sol`/`Groth16Verifier.sol`, notification/statistics infrastructure, `FamilyConstitution.sol` with its propose/approve CLIs and real Sepolia deployment, and the MCP-server wrapping of `propose_action` letting an AI assistant actually execute the proposal step via a tool call (`approve_action` remains CLI-only).
- **During the event (Sept 25–27):** the real World ID integration in `worldid/` (Express RP-signing/verification server, IDKit in `voice_challenge.html`, `/api/verify-call` returning `humanOk` / `phraseOk` / `memberOk`), replacing (and deleting) the pre-event JS placeholder; plus final demo rehearsal, video recording, and this submission writeup.

## How it's made (tech stack)

circom 2.2.3 + Poseidon (circuit) · Rust `ark-circom`/`arkworks` (witness/proof generation, BN254/Groth16) · Solidity/Foundry (`Groth16Verifier.sol`, `FamilyRegistry.sol`, `FamilyConstitution.sol`) · World Chain Sepolia (deployment target) · `alloy` + `reqwest` + `tokio` (Rust event-monitoring/notification backend, Resend API) · plain HTML/vanilla JS (demo UI, no build tooling) · `rmcp` (Rust MCP SDK, wraps `propose_action` as a tool; `approve_action` is not yet MCP-wrapped) · World ID: `@worldcoin/idkit-core` (browser, via esm.sh) + Node.js/Express + `@worldcoin/idkit-server` (RP signing) + `viem` (signal hashing) + World v4 verify API.

## Demo video

https://youtu.be/bzp4HG2sUcQ (3:32, 1080p; narration in Japanese by the developer, with English subtitles. Source: `demo/family_proof_final_en.mp4`)

## Live contracts (World Chain Sepolia)

| Contract | Address |
| --- | --- |
| Groth16Verifier | `0x132a7dbd30784d2283b83D96BD45B731AF331c8a` |
| FamilyRegistry | `0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d` |
| FamilyConstitution | `0xf7f344E9399638b69DF158877F1e77a39A5F3D73` |

---

## TODO before final submission

- [x] Narration recorded by the developer (own voice, no AI voice) and mixed; English subtitles added → `demo/family_proof_final_en.mp4`
- [x] Upload the final video and paste the link into "Demo video" above
- [ ] Commit and push everything (World ID code + docs + video) so the GitHub repo matches this writeup
- [x] World ID scene added to the demo video (`demo/family_proof_final_en.mp4`): accept, and the impostor case (the same World ID removed from the family allowlist, disclosed on screen)
- [ ] Paste final text into the ETHGlobal submission form (field names/limits may differ slightly from this draft's section breaks — adjust as needed)
