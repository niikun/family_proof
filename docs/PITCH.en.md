# Pitch Script (ETHGlobal Tokyo 2026)

🌐 [日本語版](PITCH.md) — the live demo itself is delivered in Japanese; this English version is a reference document for judges/materials.

Judging slot: 7 minutes (4 min demo + 3 min Q&A). Criteria: technicality / originality / practicality / UX / WOW factor.

Premise: only rehearsed, working demonstrations are scripted here. Family Constitution (the tiered Action Authorization system) was already deployed to production World Chain Sepolia and demonstrated on testnet before the event. A fallback is included in case the MCP-server integration attempted during the event doesn't make it in time (see "If MCP doesn't make it in time").

---

## Overall structure (target timing)

**Revised 2026-09-22: Scene 3 (AI voice-clone defense) has been cut from the demo, and its time reallocated to Family Constitution** (the developer's call: "the demo was too long and hard to follow — Family Constitution is the more interesting part"). The World ID voice-clone-defense design itself isn't gone — it's just not performed live; it's covered in the Q&A prep section instead.

| Part | Time | Content |
|---|---|---|
| 0. Hook (problem statement + reframe) | 0:00–0:25 | The contradiction in phone-scam countermeasures → pivot to "this is actually a bigger story" |
| 1. Scene 1: family-impersonation check | 0:25–0:45 | Verification succeeds without ever revealing the secret |
| 2. Scene 2: RLN reuse detection | 0:45–1:25 | A stolen secret used twice gets recovered and revoked |
| 3. Family Constitution (centerpiece) | 1:25–3:30 | Claude actually plays the AI Agent role → proposes → the attacker/Claude itself fails to approve → two neighbors approve and it executes |
| 4. Close (vision + tagline) | 3:30–4:00 | Trust Circle / a society of people living alone |

Anticipated Q&A is in the "Q&A prep" section at the end.

---

## 0. Hook (0:00–0:25)

> "The standard defense against phone impersonation scams in Japan is a family password. But the password approach has a built-in contradiction: the very act of saying a secret to verify your identity exposes that secret to being overheard on the call. Once it leaks, you can never use it again."
>
> "FamilyProof proves 'I am family' with a ZK proof, without ever revealing the secret. But as I built this out, I realized it wasn't solving 'family authentication' — it was solving something bigger: trust infrastructure for the age of AI, not bound by blood or cohabitation. Today I'll show you both halves of that."

(Show a single slide/the README's opening diagram here — don't linger, prioritize the live technical demo.)

---

## 1. Scene 1: family-impersonation check (0:25–0:45, ~20 sec)

Setup: keep `cargo run --bin notifier` running continuously in Terminal A. Terminal B is ready for commands. The `cast` keystore password has been confirmed beforehand.

```bash
cargo run --bin submit_demo -- 777
```

→ enter password → point at `status: 1 (success)`:

> "On a real testnet, the proof that 'I am family' just went through — without ever sending the secret."

---

## 2. Scene 2: RLN reuse detection (0:45–1:25, ~40 sec)

```bash
cargo run --bin submit_demo -- 888
```

(Only the challenge changes — a second proof with the same secret, same epoch.)

→ confirm the tx succeeded → within 30 seconds, Terminal A shows `PotentialLeak` detected and `recovered secret = 203`:

> "If a stolen secret gets reused across different calls with different challenges, the secret itself can be mathematically recovered from the two proofs. This is RLN (Rate-Limiting Nullifier) — combining a ZK-SNARK with a Shamir-style linear secret share. The more it's used, the more likely it is to be recovered and revoked — an asymmetric structure that works against the attacker."

(If time allows, as a closer)

```bash
cargo run --bin rotate
cast send 0xa9f1A920... "updateRoot(uint256)" <new root> --account deployer
cast call 0xa9f1A920... "familyRoot()"
```

> "Detect → recover → notify → reissue → update the root — all connected end-to-end on a real chain."

---

## 3. Family Constitution (centerpiece, 1:25–3:30, ~2 min 5 sec)

Premise: `FamilyConstitution.sol` is already deployed to production World Chain Sepolia (`0xf7f344E9399638b69DF158877F1e77a39A5F3D73`), and the Rust propose/approve CLIs are verified on-chain through Tier 0/1/2 (all completed before the event). The event-day goal is to wrap `propose_action`/`approve_action` as an MCP server so that **Claude itself sends the on-chain transaction via a real tool call, instead of a human running it**. **If MCP doesn't make it in time, the demo falls back to the already-proven "Claude proposes → a human copy-pastes the `cast send` command" flow below** (see "If MCP doesn't make it in time").

Screen layout: three terminal panes side by side. **Pane A = Claude Code (playing the AI Agent role — this is really Claude, not a script)**, Pane B = Neighbor A, Pane C = Neighbor B (B and C hold secrets/salts for two different members of the same tree, reusing the existing demo keys).

> "Here's the real point: what FamilyProof actually proves isn't 'are you family?' — it's 'did a trusted relationship approve this action?' From here, the AI Agent role isn't acted out — it's played by the real Claude."

Demo scenario (per SPEC.md §11.6, with the AI role replaced by an actual live LLM conversation; the lines and flow are the same either way — only how the command actually gets executed differs):

1. **Pane A (Claude)**: tell Claude, "I think we need to send ¥3,000,000 for mom's care costs — please propose it." Claude judges the tier from the risk (tier=2):
   - **If MCP is working**: Claude calls the `propose_action` tool directly and sends the transaction on the spot → `ActionProposed`
   - **Fallback (no MCP)**: Claude returns the exact command to run (`propose_action`/`approve_action` derive `actionId` automatically from the description string, so nobody needs to precompute or type in a numeric ID by hand):
     ```bash
     cargo run --bin propose_action "mom's care costs, 3,000,000 yen" 2
     ```
     → run the given command as-is → `ActionProposed`
   > "This isn't scripted — Claude is judging this and executing it live, right now."
2. An attacker (who has no secret) tries `approve_action` → cannot produce a valid proof → fails
3. **Pane A (Claude)**: ask, "Claude, try approving it yourself too." Claude replies that it holds no secret in the Trust Circle's Merkle Tree, so it cannot produce a valid ZK proof and cannot approve (if MCP is working, show Claude actually calling the `approve_action` tool and it reverting; otherwise, show the equivalent `cargo run --bin approve_action "mom's care costs, 3,000,000 yen" <leaf_idx>` reverting)
   > "It isn't a contract-level restriction stopping the AI from approving its own proposal — it's the soundness of the ZK system itself."
4. **Pane B (Neighbor A)**: approves with a real ZK proof:
   ```bash
   cargo run --bin approve_action "mom's care costs, 3,000,000 yen" 0
   ```
   → `ActionApproved(1/2)`
5. **Pane C (Neighbor B)**: approves with a real ZK proof (a different member, so a different `leaf_idx` and a different nullifier):
   ```bash
   cargo run --bin approve_action "mom's care costs, 3,000,000 yen" 1
   ```
   → `ActionApproved(2/2)` → `ActionAuthorized`
6. Contrast: ask **Pane A (Claude)** to "set a reminder for the appointment" → Claude judges tier=0 → executes immediately
   > "Day-to-day decisions can be handled by the AI alone, but the higher the risk, the more human consensus is required. It's not 'family means anything goes' — it's turning agreement within a trusted relationship into an enforced protocol."

**Mini fallback if Claude can't be reached (network, etc.)**: if the Pane A exchange fails, say "the live AI conversation isn't connecting today" and move straight to running step 1's command with a pre-picked `actionId`. Steps 2 onward don't depend on the network, so the rest of the demo continues unaffected.

### If MCP doesn't make it in time (fallback)

Even if the MCP-server integration doesn't make it in time, Family Constitution itself — the contract, the CLIs, the production Sepolia deployment — was already finished and proven before the event, so this scene still runs exactly as scripted above: Claude reasons about the request and proposes/judges tier in conversation, and a human runs the resulting command. Same lines, same flow — only the execution mechanism changes. There's no risk of the demo itself falling apart.

Only if something deeper goes wrong (Sepolia connectivity, a flaky testnet, etc.) fall back further to showing the local `forge test -vv` results (all tests green) while explaining verbally:

> "The Family Constitution contract and its tests are already complete — all scenarios pass, from tier-0 instant execution to double-approval prevention and rejecting an invalid attacker. Connectivity issues mean we can't show it live on testnet today, but the code is public and you're welcome to look."

Be upfront about the actual situation — the Continuity Track also rewards honest progress reporting.

---

## 4. Close (3:30–4:00)

> "In a society with more elderly people and single-person households living alone, defining 'family' only by blood leaves many people without anyone to turn to for important decisions. What FamilyProof aims to build is infrastructure that lets an AI and a handful of trusted people — neighbors, friends, whoever you choose — function as 'family,' while protecting everyone's privacy."
>
> "Family isn't an attribute — it's a relationship. And relationships can be updated. That's what the ZK, RLN, and Family Constitution I showed you today make possible."
>
> **"Prove trust, not identity."**

---

## Q&A prep

**Q: Isn't World ID actually not integrated?**
A: That's correct, and I want to be upfront about it. I validated the production design (RP-signing + v4 verify API) against docs.world.org and documented it in SPEC.md, but given a 2.5-day window, I prioritized hardening the core ZK + RLN + on-chain implementation over taking on the risk of a from-scratch integration. The AI-voice-clone defense design — separating proof of knowledge from proof of liveness — is written up in the README/SPEC.md, but it only exists as a self-contained JS mock, not something I demoed live in today's 4 minutes. I understand this may disqualify the project from a World ID/Worldcoin partner prize.

**Q: Why not just use TOTP (a rotating one-time password)?**
A: A TOTP scheme requires the verifying device to also hold the secret (or equivalent information), so you can't escape the problem of "the secret has to live somewhere." With the ZK approach, the verifier only ever needs the (public) root — verification completes without the verifier learning the secret at all. Even the provider of an anti-scam app never has to hold the family's secret.

**Q: RLN's epoch/limit is reused for Action Authorization too — isn't that a problem?**
A: It's a documented known limitation. The current RLN instance shares a "once per hour" constraint, so if the same member approves multiple Actions within one hour, they can unintentionally expose their own secret. The demo is sequenced so this doesn't happen, but a production deployment would need a separate RLN instance dedicated to Action Authorization.

**Q: What if the AI Agent goes rogue or gets compromised?**
A: The AI Agent can only execute Tier 0 (low-risk) actions alone — Tier 1 and above always require a human's valid ZK proof. Since the AI Agent itself holds no leaf in the Merkle Tree, it is structurally incapable of producing a valid proof. That said, the AI Agent having no cryptographic identity or revocation mechanism of its own is a known limitation — a way to cut off a compromised Agent from the Trust Circle is future work.

**Q: Which parts of this project were built before the event, and which during it?**
A: The ZK circuit, RLN, FamilyRegistry, notification infrastructure, anonymous statistics, and Family Constitution — the trust-approval-protocol extension I showed today — were all built and verified on production World Chain Sepolia before the event started (through Sept 24). What was newly built during the event (Sept 25–27) is the MCP-server integration that lets Claude itself operate the AI Agent role via real tool calls. This is documented explicitly in the README and the Continuity submission writeup.

**Q: Did you write all the code yourself? How did you use AI?**
A: I wrote all the Solidity and Rust code myself. Claude acted only as a coach — design review, confirming builds/tests — and never wrote any code. It found and pointed out implementation bugs, but I made every fix myself.

**Q: Claude was playing the AI Agent role in the demo — is an LLM actually integrated into the contract?**
A: No, there's no integration. The design has been unchanged since Family Constitution was first built: only a single EOA designated via `onlyAgent` can call `proposeAction`. Today, I simply replaced "the proposing party" — normally fixed Rust logic — with an actual live conversation with Claude. The substance doesn't change: whatever the AI proposes, Tier 1 and above can never execute without a human's ZK proof, enforced at the contract level. If anything, showing "the AI can't approve its own proposal" with a real LLM makes the point land harder.

**Q: Why not give the AI its own Trust Circle secret so it could participate in approvals?**
A: That's deliberate. If the AI held a secret, its own proof would count as one of the required approvals, effectively lowering the threshold by one. On top of that, an LLM's judgment can be hijacked via prompt injection — so "the AI's secret" would just become "a secret extractable through a prompt," reintroducing exactly the scam pattern this project started from (being talked into revealing a secret over the phone), just moved onto the AI. That's why the AI is never given one at all.

**Q: How far could this be extended technically? What's the next research direction?**
A: The most interesting direction isn't giving the AI authority directly, but having it prove, per Action, that it holds a *delegated* capability from the Trust Circle, via ZK. Delegation constraints (an amount cap, a tier cap) get committed into a separate Merkle Tree; the AI then proves both that it holds a delegation from that tree and that the current Action satisfies its constraints — via a range proof, without revealing the constraint values themselves. Revoking a delegation would reuse the same root-update mechanism as everything else. This needs a genuinely new circuit and key set on the proposer's side, so it's out of scope for this event, but the design is written up in `docs/SPEC.md §11.8`.
