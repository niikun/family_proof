# FamilyProof

**Prove trust, not identity.**

血縁・婚姻・同居を前提としない「Trust Circle」を、暗号技術によって実装するプロトコル。**家族は Trust Circle の一つの実装例**である。

FamilyProof は、本人が選んだ Trust Circle の一員であることを、秘密や身元を明かさずに証明し、その membership を使って AI 時代の重要な意思決定を複数人の合意によって守る。

> ひとりで暮らすことと、ひとりで生きることは違う。

AI は代理人として日常の低リスクな判断を支援し、リスクが上がるほど Trust Circle の人間による承認を要求する。ZK-SNARK（Groth16 / circom）＋ RLN（Rate-Limiting Nullifier）＋ オンチェーンの Family Constitution（tier別承認）によって、「誰なのか」ではなく「信頼関係の中で承認権を持つこと」を検証可能にする。ETHGlobal Tokyo 2026 提出（Continuity Track）。

> ⚠️ ステータス（2026-09-21時点）: ZK Merkle 証明・RLN・on-chain Registry（Step 0〜6）は World Chain Sepolia に実デプロイ済み・実チェーンで動作確認済み。Family Constitution（Step 7、本READMEの中核テーマ）はコントラクト・テストまで完成、testnet実演は本番期間（9/25〜27）に実施予定。詳細は「動いているもの／まだ動いていないもの」を参照。

---

## なぜ始まり、なぜ広がったか

もともと FamilyProof は「オレオレ詐欺（なりすまし電話詐欺）対策として、家族の合言葉（secret）を一度も口にせずに『自分は家族である』ことを証明する」プロジェクトとして始まった。

しかし実装を進める中で気づいたのは、この仕組みが解いているのは「本人確認」ではなく、もっと一般的な問題だということ。

> ひとりで暮らすことと、ひとりで生きることは違う。

独居高齢者・単身世帯が増える社会では、「血縁・婚姻・同居」を前提にした家族像だけでは、多くの人が「重要な判断のときに頼れる相手がいない」状態に置かれる。一方で、頼れる関係——AI、隣人、友人、遠方の親族——は本来もっと柔軟でいい。ただしそれを「家族」として機能させるには、

- 全員に本名や個人情報を晒さずに「この人は信頼できる関係の一員だ」と証明できること
- 一人の判断・一人の乗っ取りだけでは重要な行動が実行されないこと
- その関係（メンバー構成）が、時間とともに更新できること

が要る。FamilyProof がすでに実装している「秘密を明かさない ZK membership proof」「盗まれた秘密が使うほど自壊する RLN」「`updateRoot()` によるメンバー構成の更新」は、実はこの一般化されたインフラのコア部品として、ほぼそのまま説明し直せる。

```
                  AI（代理人／低リスクのみ単独実行可）
                   │
                   │ 日常の判断・記録
                   │
         ┌─────────┴─────────┐
         │                   │
       本人 ─────────── 信頼できる人 A
         │                   │
         │                   │
         └──── 信頼できる人 B ─┘
```

ここで重要なのは、**全員が互いの本名や個人情報を知る必要がない**こと。証明されるのは「この人は本人が選んだ信頼関係（Trust Circle）の一員である」ことだけで、それが誰なのかは必要以上に公開されない。

### 「家族」は Trust Circle の一実装

ここまでずっと「家族」という言葉を使ってきたが、正確には FamilyProof が実装しているのはもっと一般的な概念——**Trust Circle**——であり、「家族」はその一つの表れに過ぎない。

```
Family
  ↓
血縁ではない・婚姻ではない・同居でもない
  ↓
Trust Circle（ZK membership ＋ 複数人承認）
```

この抽象化によって、同じ仕組みは血縁家族以外にも適用できる:

- 高齢者 ＋ 近隣住民
- 障害者 ＋ 介助者
- 海外在住者 ＋ 日本に残る家族
- 小規模企業 ＋ 共同経営者
- AI エージェント ＋ 人間

「家族を再定義する」のが思想で、「Trust Circle」がそれを実装する技術概念——FamilyProof というプロジェクト名は最初のユースケース（家族）に由来するが、プロトコルとしての射程はそれより広い。

### Before → After

| | 証明する対象 |
|---|---|
| **Before**（当初の設計） | 「この人は家族ですか？」（Identity Proof） |
| **After**（本プロジェクトの現在地） | 「本人が選んだ Trust Circle が、この Action を承認したこと」（Action Authorization） |

> **正確に言うと**: FamilyProof が暗号的に証明しているのは「信頼」そのものではなく、「本人があらかじめ選んだ Trust Circle の membership を持っていること」である。"Prove trust, not identity." は思想としてのタグラインであり、技術的な主張としてはむしろ "Prove membership, protect identity."（誰かの信頼性そのものではなく、身元を明かさずに membership を証明する）が正確。タグラインは思想として掲げつつ、本文では何を数学的に証明しているかを正確に説明する。

AI は Trust Circle の対等なメンバーではなく、**代理人**として位置づける。低リスクな判断（予定管理など）は単独で実行できるが、リスクが上がるほど人間の承認人数が増える——「家族だから何でも自由」ではなく、**信頼関係の合意をプロトコルとして強制する**。

| Action の例 | tier | 必要承認人数 |
|---|---|---|
| 日常対応（予定管理・リマインド） | 0 | 0（AI が単独実行） |
| 少額の支払い | 1 | 人間 1人 |
| 医療・介護など重要な判断／高額送金 | 2 | 人間 2人 |
| Trust Circle 構成の変更 | 3 | 人間 3人 |

そして Trust Circle は固定ではない。メンバーは時間とともに入れ替わってよく（隣人 → 友人 → 介護者、など）、その更新は Merkle Tree の root を更新する（`updateRoot()`）という、すでに実装済みの技術操作としてそのまま表現できる。**家族は「属性」ではなく「関係」であり、関係は更新できる。**

AI が「最初の家族」、数人の信頼できる人間が「社会的な家族」になる——その関係を ZK・スマートコントラクト・（将来的には）AI エージェントによってプライバシーを守りながら成立させるのが FamilyProof の目指す姿。

---

## そもそもの出発点：オレオレ詐欺対策として

- 日本のオレオレ詐欺は被害額が高止まりしている社会問題。令和7年（2025年）確定値で認知件数 14,489件・被害額 1,138.1億円（警察庁）
- 既遂1件あたり平均被害額は特殊詐欺全体で523.6万円、オレオレ詐欺サブタイプでは785万円（同資料）
- 過去5年間に特殊詐欺と思われる電話等を受けた人は33.3%、うち実際に被害に遭った人は1.5%（法務省法務総合研究所、令和7年3月）
- 従来対策（固定の合言葉・暗証番号）には根本的な矛盾がある。「本人確認のために秘密を言わせる」行為そのものが、電話越しの盗聴・録音によって秘密を漏洩リスクに晒し、一度漏れたら二度と使えない

FamilyProof は secret を一切送信・保存せず、ZK-SNARK で「知っていること」だけを証明する。さらに RLN により、盗まれた secret を電話ごとに異なる challenge で2回使うと、その2つの証明から `secret` を数学的に復元でき、漏洩した membership を失効させられる（2回使用 → `PotentialLeak` 検知 → secret 復元 → 新 secret 発行 → Merkle Tree 再構築 → root 更新、詳細は後述）。

---

## 動いているもの／まだ動いていないもの（正直な線引き）

Continuity Track はイベント前からの既存部分とイベント中に新規に作った部分を明記する必要があるため、ここは特に正確に書く。

### ✅ 実装済み・実チェーンで動作確認済み（World Chain Sepolia、Step 0〜6、イベント前）

- circom による Merkle 包含証明回路（Poseidon ハッシュ、`Poseidon([0, secret, salt])` の2重コミットメント）
- RLN（Rate-Limiting Nullifier）: 同一 epoch 内に同一 secret で異なる challenge の証明を2つ出すと、2点のシェアから `secret` を体の演算で復元し失効できる
- Rust（`ark-circom` / `arkworks`）による witness 生成・proof 生成・検証・secret復元・Solidity calldata変換
- `Groth16Verifier.sol` / `FamilyRegistry.sol`（root 登録・membership 検証・epoch鮮度チェック・nullifier記録・`PotentialLeak` イベント発行）
- `updateRoot()` によるメンバー再登録（漏洩検知→復元→新secret発行→木の再構築→root更新、を実チェーンで一気通貫実証済み）
- 通知インフラ（`notifier.rs`：イベント監視→secret復元→メール通知→匿名統計集計→S3公開、をポーリングループで自動実行）
- 匿名統計ダッシュボード（`stats.html`、家族root・アドレス等の個人特定情報は一切含まない日次カウントのみ）
- 攻撃者の期待損失シミュレーション（警察庁確定値ベース、出典明記）

### ✅ 実装済み・テスト完了（Step 7、イベント中に新規実装）

- `FamilyConstitution.sol`: Action の提案（`proposeAction`、AI Agent 役の EOA のみ実行可）、ZK証明つき承認（`approveAction`）、tier別閾値での自動実行、`ActionAuthorized` イベント
- ユニットテスト6本全緑（tier0即実行／tier2の1人承認では未実行／2人承認で実行／同一nullifierの二重承認防止／challenge不一致でrevert／agent以外からの提案でrevert）
- **AI Agent はコントラクトレベルで「提案はできるが、tier1以上は人間の有効な ZK 証明なしには実行できない」ことが強制される**。AI Agent 自身は Trust Circle の Merkle Tree に leaf を持たない（＝secret を持たない＝有効な proof を作れない）ため、これは権限管理の実装ではなく ZK の健全性そのものの帰結

### 🚧 未実装・イベント本番（9/25〜27）で取り組む予定

- Rust側の Action 提案〜承認 CLI（`propose_action.rs` 相当）
- `FamilyConstitution.sol` の World Chain Sepolia への実デプロイと testnet 上でのデモ一気通貫実演

### 🔭 ビジョン（設計はしたが、今回のスコープでは実装しない）

- AI Agent が LLM で自律的に「この判断は誰の承認が要るか」を判断する仕組み（今回は Rust の固定ロジックで「AI が提案する」ことをシミュレート）
- Trust Circle が自分たちで tier→閾値のマッピングを動的に変更できるガバナンス機構（現状はコントラクトの定数）
- AI Agent 自身が暗号的な ID を持ち、侵害された Agent を Trust Circle から切り離せる仕組み——さらに一段深い方向性として、AI に Trust Circle の secret を持たせる（＝承認の1票にする）のではなく、**Trust Circle から委任された権限そのものをActionごとにZKで証明させる**仕組み（金額上限などの制約を、その値自体は明かさずにレンジ証明で満たすなど）。新しい回路・鍵一式が必要な本格的な拡張のため今回は実装しないが、次の研究方向として位置づける（詳細は [docs/SPEC.md §11.8](docs/SPEC.md)）
- AI Agent 自身の署名鍵を単一の場所に置かず、**MPC（閾値署名）で複数主体に分散して守る**運用（本番のAIエージェント×ウォレット運用の定石）。Trust Circle 側の非同期なZK承認フローには影響しない、鍵管理レイヤーの強化として位置づける（詳細は [docs/SPEC.md §11.7](docs/SPEC.md)）
- World ID の実SDK統合（IDKit + RP署名バックエンド）。デモの「AI音声クローン対策」シーンは、時間制約により本物の SDK を使わない自前 JS モックで概念実証している（本番実装の設計自体は確認済み。詳細は「既知の限界」参照）

---

## 動作原理

```
[オンボーディング(一度だけ)]
  各メンバー: 端末内で secret を乱数生成（外部送信しない）
  leaf = Poseidon([0, secret, salt])
  全メンバーの leaf から Merkle Tree 構築 → root
  → root を FamilyRegistry / FamilyConstitution に登録

[通常の membership 確認（なりすまし確認）]
  検証側: challenge を発行、現在の epoch を定める
  証明側: secret + Merkle path + challenge + epoch から Groth16 proof を生成
          public output として RLN のシェア (x, y) と nullifier が出る
  検証側: root一致・epoch鮮度・pairing検証（on-chainまたはoff-chain）
          同一 epoch で同一 nullifier・別 x の proof が2つ出たら
          2点補間で secret を復元し失効（漏洩検知）

[Action Authorization（Trust Circle 拡張、Step 7）]
  AI Agent: proposeAction(actionId, tier) — tier=0なら即実行
  Trust Circle メンバー: 通常のZK証明の challenge に actionId を渡して approveAction
                          （secretを持たない攻撃者・AI自身は有効な証明を作れない）
  → 承認人数が tier の閾値に達したら ActionAuthorized、実行可能に
```

circom 回路は Step 4 以降変更なし。`challenge` という public input を「リプレイ防止」「(将来の)World ID live-challenge」「Action ID」の3用途で使い回す設計になっている。

---

## アーキテクチャ / 技術スタック

| レイヤー | 技術 |
|---|---|
| 回路（circuit） | circom 2.2.3、Poseidon ハッシュ |
| proof 生成・検証 | Rust + `ark-circom` / `arkworks`（BN254、Groth16） |
| on-chain | Solidity（Foundry）。`Groth16Verifier.sol`（snarkjs生成）/ `FamilyRegistry.sol` / `FamilyConstitution.sol` |
| ネットワーク | World Chain Sepolia（testnet） |
| 通知・統計 | Rust（`alloy` + `reqwest` + `tokio`）、Resend API、S3公開 |
| デモUI | 単体HTML + vanilla JS（ビルドツールなし） |

## Live デプロイ（World Chain Sepolia）

| コントラクト | アドレス |
|---|---|
| Groth16Verifier | `0x132a7dbd30784d2283b83D96BD45B731AF331c8a` |
| FamilyRegistry | `0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d` |

実チェーン上での動作確認例（tx）:
- 正規メンバーの証明成功: `0xdf823cf0124ae7b9ba43595efb09ee923d479ea9a9747249aad056999e767c66`
- root rotation（メンバー再登録）: `0x2762bce1...`

匿名統計ダッシュボード（誰がいつ検証・漏洩検知されたかは含まない、日次カウントのみ公開）: `stats.html`（ローカルで直接開くか、S3公開版）

## リポジトリ構成

```
circuits/       circom 回路一式（main.circom, zkey/vkey, ビルドスクリプト）
src/            Rust: Merkle Tree / proof 生成・検証 / secret復元
src/bin/        CLI群: submit_demo, rotate, notifier, damage_sim 等
contracts/      Foundry プロジェクト（Groth16Verifier / FamilyRegistry / FamilyConstitution）
docs/           SPEC.md（設計仕様）/ HANDOFF.md（開発ログ）/ PITCH.md（デモ台本）
stats.html      匿名統計ダッシュボード
```

## セットアップ・実行

```bash
# 回路（初回のみ再生成が必要なファイルあり）
cd circuits
npm ci
circom main.circom --r1cs --wasm --sym -l node_modules
node scripts/build_input.js

# Rust
cd ..
cargo test        # Merkle / proof / RLN のユニットテスト
cargo run          # メンバー生成→証明→検証の一連デモ

# Solidity（Foundry）
export PATH="$PATH:$HOME/.foundry/bin"
cd contracts
forge build
forge test -vv     # FamilyRegistry / FamilyConstitution のユニットテスト
```

## Continuity Track / AI利用方針

- 本プロジェクトは ETHGlobal Tokyo 2026 の **Continuity Track** に提出する。Step 0〜6（ZK回路・RLN・on-chain Registry・通知インフラ・匿名統計）はイベント開始前（〜9/24）の既存部分、Step 7（Family Constitution / Trust Circle 拡張）はイベント期間中（9/25〜27）に新規実装した部分として明確に区別している
- AI（Claude）はコーチ・設計レビュー・ビルド/テスト実行確認のみを担当し、**Solidity/Rust のコードは一切書いていない**。実装はすべて開発者本人（ソロ開発）が書いている

## 既知の限界

- RLN の `epoch=1時間 / limit=1` は Action Authorization にも同じインスタンスを流用しているため、同一メンバーが1時間以内に複数の Action を承認すると意図せず自分の secret を露出しうる（実運用では用途ごとに別インスタンス化が必要）
- AI Agent 自身は暗号的な ID を持たない。現状は「`proposeAction` を呼べる特定の EOA」というアクセス制御レベルの権限に留まる
- Family Constitution の tier→閾値マッピングはコントラクトの定数で、Trust Circle 自身によるガバナンスは未実装
- デモの World ID 連携（AI音声クローン対策）は実SDKではなく概念実証のモック。本番実装の設計は確認済み（RP署名 + v4 verify API）だが未実装
- Groth16/BN254 は理論上 Shor のアルゴリズムで破られうる（量子耐性のある証明系への移行はスコープ外）

詳細な設計判断とその理由は [docs/SPEC.md](docs/SPEC.md)、開発ログは [docs/HANDOFF.md](docs/HANDOFF.md) を参照。

## Author

niikun ([GitHub](https://github.com/niikun/family_proof))
