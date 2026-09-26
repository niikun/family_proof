# FamilyProof

🌐 [日本語版](README.ja.md)

**Prove trust, not identity.**

🎬 **Demo video**: https://youtu.be/bzp4HG2sUcQ (3:32, Japanese narration with English subtitles)

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

> **World ID** (Unique Human / Sybil resistance) **was integrated for real with IDKit during the event (Sept 25–27)**. As a defense against AI voice cloning, the passphrase heard on the phone is bound to the World ID proof's `signal` and verified server-side (see "World ID: AI Voice-Clone Defense").

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

## World ID: AI Voice-Clone Defense (built during the event)

A passphrase alone falls the moment an attacker with an AI-cloned voice gets someone to say it. So on top of "knows the passphrase", FamilyProof uses World ID to check that **an Orb-verified real human, who is a registered family member, just proved with this passphrase**.

```text
Verifier (the parent)
  ① Picks a passphrase on the spot and says it over the phone

Prover (the caller)
  ② Enters the passphrase heard on the phone
  ③ Gets an RP signature from /api/rp-signature (@worldcoin/idkit-server)
     → IDKit.request (signal = passphrase) → approve in World App → proof

Verifier (the parent)
  ④ Sends the passphrase they said, together with the proof, to /api/verify-call

Server (worldid/server.js)
  ① Verify the proof with the World v4 verify API    → humanOk
  ② proof's signal_hash == hash(passphrase)          → phraseOk
  ③ proof's nullifier belongs to a registered member → memberOk
```

| humanOk | phraseOk | memberOk | Verdict |
|---|---|---|---|
| ✅ | ✅ | ✅ | Accept: a genuine check from the member |
| ✅ | ❌ | ✅ | Reject: passphrase mismatch |
| ✅ | ✅ | ❌ | Reject: right passphrase, but not a registered member (e.g. a human scammer using an AI voice clone) |
| ❌ | — | — | Reject: World ID verification failed (an AI on its own can't prove it is an Orb-verified human at all) |

The passphrase is not a pre-shared secret — it is **a challenge the parent picks on the spot for each call**. The attack is stopped at two levels. **An AI on its own** (e.g. an automated voice call) is not an Orb-verified human, so it can't produce a World ID proof at all. **A human scammer using an AI voice clone** can prove with their own World ID and, having heard the call, can type the passphrase — but they are not a registered family member, so they are rejected. The second example in the demo (the impostor test) shows this second case. And because the passphrase is baked into the proof, an old proof can't be replayed.

Design constraint: the passphrase is bound only to World ID's `signal` and is never reused as RLN's `challenge` / `epoch` (otherwise a legitimate member making two calls with different passphrases in one epoch would trigger RLN and expose their secret). Verification is off-chain, since a phone call can't wait for block confirmation.

---

## Architecture

```text
      World ID (IDKit, built during event)
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
| Human uniqueness | World ID (IDKit / `@worldcoin/idkit-server`) |
| World ID backend | Node.js / Express / `viem`           |
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
* **Deployed and verified on World Chain Sepolia**: `FamilyConstitution` = `0xf7f344E9399638b69DF158877F1e77a39A5F3D73`
* Tier 0 (instant execution) / Tier 1 (1 approval) / Tier 2 (2 approvals, from two different members producing two different nullifiers) demonstrated end-to-end on-chain
  (tx: propose `0xc76a25bfad576afa5605871b650e145f3d5ddd0ea9a2d66c68f44d4a3407ab45` / approval 1 `0xa421b7f1b18c082c5f4f1df5da8f123df8f580fd32d0c71a72d7fa77075c4b1e` / approval 2 `0xdfee0dd25f7476912714fd5b33395d1854fbf7d8e4291841d6b6c589a03d30e0`)
* `propose_action` wrapped as an MCP server, so Claude itself, playing the AI Agent role, can send on-chain transactions via a real tool call (callable directly from Claude Code as `mcp__family-proof__propose_action`; `approve_action` isn't wrapped as an MCP tool yet — it's still run manually via the CLI)

### 🚧 Built during the event (Sept 25–27)

* **Real World ID (IDKit) integration for AI voice-clone defense**: `worldid/` (Express RP-signing/verification server + `voice_challenge.html`). The pre-event placeholder page (fake `nullifier_hash`) has been deleted from the repo; verified end-to-end from proof generation in the real World App → World v4 verify API → passphrase (`signal_hash`) and registered-member (`nullifier`) checks
* Final rehearsal, demo video recording, and finishing the Continuity submission writeup

### 🔭 Future work

* Link World ID's "registered member" check to the on-chain Trust Circle (`FamilyRegistry`'s Merkle root)
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
worldid/        World ID (IDKit) RP-signing/verification server + voice_challenge.html
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

### MCP server (let Claude play the AI Agent)

Exposes `propose_action` as an MCP tool, callable from Claude Code as `mcp__family-proof__propose_action` (`approve_action` is not MCP-wrapped; run it via the CLI).

```bash
cargo build --release --bin mcp_server
```

Register it in `.mcp.json` at the repository root (start Claude Code from the repository root):

```json
{
  "mcpServers": {
    "family-proof": {
      "type": "stdio",
      "command": "./target/release/mcp_server"
    }
  }
}
```

Prerequisites:

* `mcp_server` runs `cargo run --release --bin propose_action` internally, which calls Foundry's `cast send`. Both `cargo` and `cast` must be on your PATH
* It runs `propose_action` in the repository location it was built from (the path is embedded at build time). Rebuild if you move the repository
* Your `cast` keystore needs an `agent` account and a password file at `~/.foundry/keystores/agent.pw`
* `proposeAction` is restricted by `onlyAgent` to a single EOA. You can only propose to the deployed `FamilyConstitution` if you hold that EOA's key. To try it yourself, deploy `FamilyConstitution` with your own `agent` address and replace the contract address in `propose_action.rs`

Start Claude Code and check that `family-proof` shows as connected in `/mcp`. Then ask something like "Propose a ¥3M transfer for hospital fees", and Claude will pick a tier and call `propose_action`.

### World ID demo (`worldid/`)

```bash
cd worldid
npm install
# set RP_ID / RP_SIGNING_KEY / ACTION / PASSPHRASE / FAMILY_NULLIFIERS (comma-separated) in .env
npm start
# open http://localhost:3000/voice_challenge.html
```

The left panel is the verifier (the parent), the right panel is the prover (the caller). ① Pick a passphrase on the left → ② enter the same passphrase on the right → ③ press "World ID で証明する" and open the displayed link in World App on your phone to approve → ④ press "検証する" on the left to see the verdict. Each link is single-use, so press the button for a fresh link every time you prove.

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
* World ID's "registered member" check is an off-chain allowlist of pre-captured nullifiers in the server's `FAMILY_NULLIFIERS` env var. It is not yet linked to the on-chain `FamilyRegistry`
* The World ID demo runs both the prover and verifier roles on one page and one server, and the verifier's reference passphrase is sent from the browser to the server. A production deployment would verify on the parent's own device
* Groth16/BN254 is theoretically breakable by Shor's algorithm (migrating to a post-quantum-secure proof system is out of scope)
* Family Constitution's approval flow (`FamilyConstitution.approveAction`) currently verifies the Groth16 proof and checks for a duplicate nullifier scoped to that single Action — it does not route through `FamilyRegistry`'s RLN leak-detection state (`PotentialLeak`). A production deployment would need its own RLN instance for Action Authorization, explicitly integrated with the Registry's revocation/leak handling

---

## Continuity Track / AI Usage Policy

This project is submitted to the **Continuity Track** of ETHGlobal Tokyo 2026. The ZK circuit, RLN, on-chain Registry, Family Constitution, notification/statistics infrastructure, and the MCP server integration (letting Claude itself operate the AI Agent role via real tool calls) are all pre-existing work, built before the event started (through Sept 24). During the event itself (Sept 25–27), the work is **the real World ID integration with IDKit (`worldid/`)**, plus final rehearsal, recording the demo video, and finishing the Continuity submission writeup.

How AI (Claude) was used, file by file:

* **Code (written by the developer)**: the circom circuit, Rust (`src/`), Solidity (`contracts/`), and the World ID verification logic (`worldid/server.js`, and the IDKit call and verification flow in `voice_challenge.html`) were all written by the developer, solo. Claude acted as a coach — design discussion, code review, pointing out bugs, running builds/tests — but did not write this code. Bug fixes were made by the developer
* **Code exceptions**: during the event, at the developer's request, Claude edited part of the demo UI in `worldid/public/voice_challenge.html` (explanatory text, panel order, step numbering, and the proof-result summary display), changed the server path in `.mcp.json` to a relative path, and, as pre-submission cleanup, deleted the practice page (`worldid/public/index.html`) and the practice-only endpoint it used (`/api/verify-proof`)
* **Documentation (mostly written/edited by Claude)**: `README.md` / `README.ja.md` / `docs/` (`SPEC.md`, `PITCH.md`, `PITCH.en.md`, `SUBMISSION.en.md`, `HANDOFF.md`) / `demo/narration_script.md` were mostly written by Claude from conversations with the developer; the developer reviewed them and made the decisions
* **Demo video (editing assisted by Claude)**: all screen recordings were captured by the developer in the real environment. Claude generated the title cards, progress badges, and captions (Python/Pillow) and did the cutting and assembly with ffmpeg. The narration is the developer's own voice; no AI voice is used

---

## Author

niikun ([GitHub](https://github.com/niikun/family_proof))
