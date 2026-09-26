# FamilyProof

🌐 [English version](README.en.md)

**Prove trust, not identity.**

> **家族は、血縁だけではない。**
> 信頼して、権限を託せる関係も「家族」になり得る。

FamilyProof は、血縁・婚姻・同居を前提としない **Trust Circle** を、ZK とスマートコントラクトで実装するプロトコルです。

ひとりで暮らしていても、ひとりで生きる必要はない。

遠くの家族、友人、近隣の人、支援者。
そして将来は **AI Agent** も、Trust Circle の一員を支える代理人になり得ます。

FamilyProof は、

> **「この人は誰か？」ではなく、
> 「この人は本人が選んだTrust Circleのメンバーであり、このActionを承認する権限を持つか？」**

を、秘密や個人情報を明かさずに検証します。

> 正確に言うと、暗号的に証明しているのは「信頼」そのものではなく「本人が選んだ Trust Circle の membership」です。

---

## From Family to Trust Circle

従来の本人確認：

```text
「この人は家族ですか？」
```

FamilyProof：

```text
「本人が選んだTrust Circleが、このActionを承認したか？」
```

Trust Circle は固定された「家族属性」ではありません。

```text
本人
 ├─ 家族
 ├─ 友人
 ├─ 近隣の人
 ├─ 支援者
 └─ AI Agent
```

メンバーは時間とともに入れ替えることができます。

---

## AI is an Agent, not a Human

AI Agent は、Trust Circle の人間と同じ権限を持ちません。

低リスクなActionはAIが単独で実行できますが、リスクが高くなるほどHuman approvalを要求します。

| Action         | Tier | Required approval |
| -------------- | ---: | ----------------: |
| 予定管理・リマインド     |    0 |                0人 |
| 少額の支払い         |    1 |          Human 1人 |
| 高額送金・重要な判断     |    2 |          Human 2人 |
| Trust Circle変更 |    3 |          Human 3人 |

**AIに権限を与えるのではなく、人間がAIに「どこまで任せるか」を暗号的に制御する。**

---

## Why ZK?

Trust Circle のメンバーは、自分の `secret` を公開する必要がありません。

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

証明者は、

> 「私はこのTrust Circleのメンバーである」

ことだけを証明します。

本名、secret、家族構成などの情報は、必要以上に公開しません。

### Cryptographic primitives

* **Merkle Tree** — membership を表現
* **Poseidon** — ZK回路向けハッシュ
* **Groth16 / BN254** — membership proof
* **RLN** — secret の使い回し検知と失効
* **Smart Contract** — Action policy と承認状態を検証

> **World ID**（Unique Human / Sybil resistance）は、**イベント期間中（9/25〜27）に IDKit を使った本物の統合を実装しました**。AI音声クローン対策として、電話で聞いた合言葉を World ID proof の `signal` に結び付けて検証します（詳細は「World ID: AI音声クローン対策」節）。

---

## RLN: Reusing a Stolen Secret Backfires

FamilyProof は、盗まれた `secret` の使い回しも検知します。

同じ `epoch` で、同じsecretを異なるchallengeに対して複数回使用すると、

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

となります。

つまり、

> **盗まれたsecretを使うほど、攻撃者側が不利になる。**

漏洩検知後は新しいsecretを発行し、Merkle Rootを更新できます。

---

## Action Authorization

AI Agent などがActionを提案します。

```text
AI Agent
    │
    │ proposeAction(actionId, tier)
    ▼
 Action
    │
    ├─ Tier 0 → 即時実行
    ├─ Tier 1 → Human 1人
    ├─ Tier 2 → Human 2人
    └─ Tier 3 → Human 3人
```

Trust Circle のHumanは、`actionId` をchallengeとして含むZK proofを提出します。

```text
secret + Merkle path + actionId
                ↓
           Groth16 Proof
                ↓
      membership + approval
```

必要な承認数に達すると `ActionAuthorized` が発行され、Actionを実行できます。

AI Agent 自身は Trust Circle の Merkle Tree に入っていないため、Human approval 用の有効なproofを生成できません。この一連の流れ（提案→承認2人→`ActionAuthorized`）は World Chain Sepolia の実チェーン上で実証済みです（詳細は「Current Status」参照）。

---

## World ID: AI音声クローン対策（イベント中に実装）

合言葉だけでは、AIで声をクローンした攻撃者が合言葉を聞き出した時点で突破されます。そこで「合言葉を知っている」ことに加えて、**Orb で確認された実在の人間が、登録済みの家族本人として、いまこの合言葉で証明した**ことを World ID で確認します。

```text
確認する側（親）
  ① その場で合言葉を決めて、電話で相手に伝える

証明する側（電話をかけてきた相手）
  ② 電話で聞いた合言葉を入力
  ③ /api/rp-signature でRP署名を取得（@worldcoin/idkit-server）
     → IDKit.request（signal = 合言葉）→ World App で承認 → proof

確認する側（親）
  ④ 自分が伝えた合言葉と proof を /api/verify-call へ送る

サーバー（worldid/server.js）
  ① World v4 verify API で proof を検証         → humanOk
  ② proof の signal_hash == hash(合言葉)         → phraseOk
  ③ proof の nullifier が登録済み家族のものか    → memberOk
```

| humanOk | phraseOk | memberOk | 判定 |
|---|---|---|---|
| ✅ | ✅ | ✅ | 受理：本人からの正当な確認 |
| ✅ | ❌ | ✅ | 拒否：合言葉が一致しない |
| ✅ | ✅ | ❌ | 拒否：合言葉は合っているが、登録済みの家族ではない（AI音声クローン・なりすましの可能性） |
| ❌ | — | — | 拒否：World ID の検証に失敗 |

合言葉は事前に共有する秘密ではなく、**親が通話のたびにその場で決めるチャレンジ**です。AIクローンも通話を聞いているので合言葉は入力できますが、登録済みの家族本人の World ID では証明できません。また合言葉が proof に焼き込まれるため、過去の proof を使い回すこともできません。

設計上の制約: 合言葉は World ID の `signal` にだけ結び付け、RLN の `challenge` / `epoch` には流用しません（流用すると、正規メンバーが同じ epoch 内に別の合言葉で2回通話しただけで RLN が発動し、secret が露出するため）。通話中にブロック確定を待てないので、検証は off-chain で行います。

---

## Architecture

```text
          World ID（IDKit・イベント中に実装）
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
| Human uniqueness | World ID（IDKit / `@worldcoin/idkit-server`） |
| World ID backend | Node.js / Express / `viem`           |
| Backend / CLI    | Rust / `alloy` / `tokio`             |
| Demo UI          | HTML / Vanilla JS                    |

---

## Current Status

### ✅ Implemented & verified on World Chain Sepolia（イベント開始前、〜9/24）

Core ZK membership + leak detection:

* Merkle membership proof / Poseidon commitment
* Groth16 proof generation / verification（Rust witness/provingパイプライン）
* RLN nullifier / 同一epoch二重使用検知 / secret復元・失効 / Merkle Root rotation
* `Groth16Verifier.sol` / `FamilyRegistry.sol`
* 通知・匿名統計インフラ（`notifier.rs`, `stats.html`）

Family Constitution（tier別Action Authorization）:

* `FamilyConstitution.sol`: `proposeAction` / ZK-backed `approveAction` / tier別自動実行 / `ActionAuthorized` / 二重承認防止 / challenge不一致防御 / AI Agent限定の提案権限。ユニットテスト全緑
* `propose_action.rs` / `approve_action.rs`（Action提案・承認CLI）
* **World Chain Sepolia に実際にデプロイ・実証済み**: `FamilyConstitution` = `0xf7f344E9399638b69DF158877F1e77a39A5F3D73`
* tier0（即時実行）/ tier1（承認1人）/ tier2（承認2人、異なるメンバーで異なるnullifierになることまで確認）を実チェーン上で一気通貫実証済み
  （tx: propose `0xc76a25bfad576afa5605871b650e145f3d5ddd0ea9a2d66c68f44d4a3407ab45` / approve 1人目 `0xa421b7f1b18c082c5f4f1df5da8f123df8f580fd32d0c71a72d7fa77075c4b1e` / approve 2人目 `0xdfee0dd25f7476912714fd5b33395d1854fbf7d8e4291841d6b6c589a03d30e0`）
* `propose_action` の MCP サーバー化: AI Agent役を Claude 自身が実際のツール呼び出しでオンチェーン送信するところまで担う（`mcp__family-proof__propose_action`としてClaude Codeから直接呼び出し可能。`approve_action`のMCPツール化は未着手、CLIでの手動実行のみ）

### 🚧 built during the event（9/25–27）

* **World ID 実統合（IDKit）による AI音声クローン対策**: `worldid/`（Express による RP署名・検証サーバー + `voice_challenge.html`）。イベント前の疑似 `nullifier_hash` のモックを廃止し、World App 実機での proof 生成 → World v4 verify API → 合言葉（`signal_hash`）と登録済み家族（`nullifier`）の照合まで動作確認済み
* 最終リハーサル・デモ動画の収録・Continuity提出文の仕上げ

### 🔭 Future work

* World ID の「登録済み家族」判定を、on-chain の Trust Circle（`FamilyRegistry` の Merkle root）と連動させる
* AI による自律的なリスク／tier判定
* Trust Circle 自身によるtier→閾値の動的ガバナンス
* AI Agent への暗号的ID付与／ZKによる委任権限の証明（[docs/SPEC.md §11.8](docs/SPEC.md)）
* AI Agent 署名鍵のMPCによる分散管理（[docs/SPEC.md §11.7](docs/SPEC.md)）

---

## Repository

```text
circuits/       circom circuits / proving artifacts
src/            Rust implementation
src/bin/        CLI tools
contracts/      Solidity / Foundry
worldid/        World ID（IDKit）RP署名・検証サーバー + voice_challenge.html
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

Action Authorization のデモ（`FamilyConstitution`への提案・承認）:

```bash
cargo run --bin propose_action "<description>" <tier>   # tier: 0-3
cargo run --bin approve_action "<description>" <leaf_idx>
```

### World ID デモ（`worldid/`）

```bash
cd worldid
npm install
# .env に RP_ID / RP_SIGNING_KEY / ACTION / PASSPHRASE / FAMILY_NULLIFIERS（カンマ区切り）を設定
npm start
# http://localhost:3000/voice_challenge.html を開く
```

画面の左が確認する側（親）、右が証明する側（電話の相手）です。①左で合言葉を決める → ②右に同じ合言葉を入力 → ③「World ID で証明する」を押し、表示されたリンクをスマホの World App で開いて承認 → ④左の「検証する」で判定が出ます。リンクはリクエストごとに1回限りなので、証明のたびにボタンから新しいリンクを出してください。

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

匿名統計ダッシュボード（誰がいつ検証・漏洩検知されたかは含まない、日次カウントのみ公開）: `stats.html`（ローカルで直接開くか、[公開版](https://niikun.net/family_proof/)）

---

## Why FamilyProof?

FamilyProof started as a solution to impersonation scams:

> **「秘密の合言葉を言わずに、家族であることを証明できないか？」**

日本のオレオレ詐欺は被害額が高止まりしている社会問題——2025年確定値で認知件数14,489件・被害額1,138.1億円、既遂1件あたり平均523.6万円（警察庁）。過去5年間に特殊詐欺と思われる電話等を受けた人は33.3%、うち実際に被害に遭った人は1.5%（法務省法務総合研究所、2025年3月）。従来対策（固定の合言葉）には根本的な矛盾があり、「本人確認のために秘密を言わせる」行為そのものが盗聴・録音による漏洩リスクを生む。

そこから問いを広げました。

> **「家族とは、本当に血縁だけなのか？」**

FamilyProof が目指すのは、AI時代の新しいTrust Circleです。

**Identity → Membership → Authorization**

誰かの身元を公開するのではなく、
**「この関係の中で、何を任せることができるのか」**を暗号的に検証します。

---

## Known Limitations

* RLN の `epoch=1時間 / limit=1` を Action Authorization にも同じインスタンスで流用しているため、同一メンバーが1時間以内に複数のActionを承認すると意図せず自分のsecretを露出しうる（実運用では用途ごとの別インスタンス化が必要）
* AI Agent 自身は暗号的なIDを持たない。現状は「`proposeAction` を呼べる特定のEOA」というアクセス制御レベルの権限に留まる
* Family Constitution の tier→閾値マッピングはコントラクトの定数で、Trust Circle自身によるガバナンスは未実装
* World ID の「登録済み家族」判定は、サーバーの環境変数 `FAMILY_NULLIFIERS`（事前に取得した nullifier の許可リスト）による off-chain チェック。on-chain の `FamilyRegistry` とはまだ連動していない
* World ID デモは、証明する側と確認する側を1つのページ・1つのサーバーで実演している。確認用の合言葉もブラウザからサーバーへ送っており、実運用では親の端末側で検証する構成が必要
* Groth16/BN254は理論上Shorのアルゴリズムで破られうる（量子耐性のある証明系への移行はスコープ外）
* Family Constitutionの承認処理（`FamilyConstitution.approveAction`）は現在、`FamilyRegistry`のRLN漏洩検知状態（`PotentialLeak`）を経由せず、Groth16 proofとAction単位のnullifier重複チェックのみで完結している。実運用ではAction Authorization用のRLNインスタンスと、Registry側の失効・漏洩検知を明示的に統合する必要がある

---

## Continuity Track / AI利用方針

本プロジェクトは ETHGlobal Tokyo 2026 の **Continuity Track** に提出する。ZK回路・RLN・on-chain Registry・Family Constitution・通知/統計インフラ・MCPサーバー化（AI Agent役をClaudeが実際にツール呼び出しで操作する部分）はいずれもイベント開始前（〜9/24）の既存部分。イベント期間中（9/25〜27）は、**World ID の IDKit 実統合（`worldid/`）** と、最終リハーサル・デモ動画の収録・Continuity提出文の仕上げに充てている。

AI（Claude）はコーチ・設計レビュー・ビルド/テスト実行確認のみを担当し、**Solidity/Rust のコードは一切書いていない**。実装は開発者本人（ソロ開発）が書いている。唯一の例外として、イベント中に `worldid/public/voice_challenge.html` のデモUIの一部（説明文・パネルの並び順・手順番号・証明結果の要約表示）を、開発者の依頼で Claude が編集した。World ID の判定ロジック（`worldid/server.js` と、ページ内の IDKit 呼び出し・検証処理）は開発者本人が実装している。

---

## Author

niikun ([GitHub](https://github.com/niikun/family_proof))
