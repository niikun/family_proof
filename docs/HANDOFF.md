# HANDOFF — 別PCへの引き継ぎ

最終更新: 2026-09-10 / ブランチ: `main` / remote: `git@github.com:niikun/family_proof.git` / 同期: `a8d5abc`（`origin/main` と一致・作業ツリー clean）

> ⚠️ **現在ビルド赤**（コミット済みだが未完了の途中状態）。`src/merkle.rs` の Poseidon 差し替え・固定深さ化（下記 [x] 3項目）は済んだが、`src/main.rs` の呼び出しと `merkle.rs` のテストが旧 SHA-256 API のまま → `cargo build` が 13 エラー。次にやるのは「merkle テスト書き換え」＋「main.rs の配線」。

## いまどこ

ロードマップ（[SPEC.md](SPEC.md) §7）で **Step 0・1・2 完了、Step 3 ほぼ完了（Rust から Groth16 setup/prove/verify + CLI 疎通まで動作）**。全体 ≈ 40%。

| Step | 状態 |
|---|---|
| 0 Rust Merkle 骨格（ZKなし） | ✅ `cargo test` |
| 1 circom サンプル写経・compile→prove→verify | ✅ WSL でも全パイプライン疎通 |
| 2 MVP 回路を自ユースケースへ | ✅ circom 側 done / Rust パディングを `EMPTY_HASH` 固定に（commit `9c52de4`） |
| 3 `ark-circom` で Rust から proof 生成・検証 | 🟢 8割（`a8d5abc`）。proof.rs の setup/prove/verify 緑。merkle.rs は Poseidon 化・固定深さ化まで済（`pso-poseidon`）。**残: merkle テスト書き換え＋main.rs 配線（今ビルド赤）** |
| 4 RLN（§6.2） | ⬜ |
| 5 デモ UI | ⬜ |
| 6 on-chain（+ World ID ゲート） | ⬜ |

**スコープ方針（2026-09-08 決定）**: RLN と World 連携（World ID オンボーディングゲート ＋ World Chain Sepolia デプロイ）を**両方チャレンジ**。厳しければ Step 3 完了前後で descope を再判断。

## 確定済みの設計判断（蒸し返さない）

1. **leaf = `Poseidon([0, secret, salt])`** — 先頭 `0` はドメインタグ。`salt` はメンバーごとの2つ目の32バイト乱数、端末内に `secret` と一緒に保存、外部送信しない（Semaphore 型コミットメント / membership-oracle 対策）。`salt` は RLN には使わない。
2. **secret の表現** = 32バイト乱数を BN254 scalar field の素数 `r`（`21888242871839275222246405745257275088548364400416034343698204186575808495617`）未満に `mod r` した10進文字列。※いまの `MEMBERS` はまだ短いダミー文字列。
3. **パディング** = 固定空値。circom/JS 側は `EMPTY_LEAF = 0n`。Rust 側も固定値にする（「最後の葉を複製」は N と N+1 が同じ root を作れるので不可）。
4. **domain separation** = 案A の A1。葉のみタグ付け（`Poseidon(3入力)`）、節は `Poseidon([L,R])`（2入力）のまま。引数個数が違えば circomlib Poseidon は別インスタンスなので葉と節は別レンジ → **`main.circom` の変更は不要**。節側の対称タグ（`Poseidon([1,L,R])`）は Step 4 の回路書き直しに畳み込む。

詳細は memory（Claude 側）にも記録済み。

## 切り替え時のルール

現在は全部コミット済み・`origin/main` と同期（`a8d5abc`）、作業ツリー clean（ただし上記のとおりビルドは赤）。
別PCでは `git pull` すればそのまま続きから入れる。

中断して別PCに移るときは毎回: `git status` で未コミットが無いか確認 → あれば
`git add` / `git commit` / `git push` してから離れる。zkey/vkey を作り直したら
それも忘れず commit（[WORKFLOW.md](../circuits/WORKFLOW.md) 参照）。

## 残タスク（TODO）

### Step 3（いまここ・次の本丸）

- [x] `ark-circom` / `color-eyre` を `Cargo.toml` に追加。
- [x] `src/proof.rs` `build_circuit()` — `circuits/main.r1cs` と `circuits/main_js/main.wasm` を読み、`circuits/input.json` を入力に witness 計算 → `get_public_inputs()`。`test_build_circuit` で root の期待値一致を確認。
  - ※ テストの `#[cfg(test)]`（`tests` ではない）と、`cargo test` の cwd がクレートルート＝相対パス `circuits/...` が実在すること前提。
- [x] **Groth16 proof 生成・検証**（`0d063dd`）。`setup(circuit, rng) -> (ProvingKey, PreparedVerifyingKey)` / `prove(pk, circuit, rng) -> (Proof, Vec<Fr>)` / `verify(pvk, public_inputs, proof) -> bool` に分割。
  - 実装は arkworks 0.6 系: `Groth16::<Bn254>::circuit_specific_setup` → `process_vk` → `Groth16::prove` → `verify_with_processed_vk`。
  - `test_prove_verify` 緑（正 proof 通過 / `pubs[0]`（root）を +1 すると検証失敗）。
  - デモは arkworks 生成鍵で確定。既存 `circuits/main_final.zkey` の `read_zkey` 取り込みは不採用（必要なら Step 6 で再検討）。
- [x] Rust CLI（`src/main.rs`）で「メンバー生成 → Merkle Tree → root」→ `build_circuit` → `setup` → `prove` → `verify` を一連実行し root と検証結果を出力。
  - ※ 現状 CLI の Merkle 部分は `src/merkle.rs`（SHA-256）で、回路が使う root（`circuits/input.json` 由来の Poseidon root）とは別物。Poseidon 化（下記）で統合する。
- [x] **Poseidon クレート選定＋ゲート**。`pso-poseidon` 0.4（arkworks 0.6 対応・light-poseidon の fork・`Poseidon::<Fr>::new_circom(n)`）を採用。`light-poseidon` 本体は arkworks 0.5 固定でプロジェクト全体の巻き戻しが要るため不採用。`examples/test_poseidon.rs` で `build_input.js` の `MEMBERS` から 16枚・4レベルの木を組み、root が `proof.rs::test_build_circuit` の期待値（`17396252…816`）＋ `input.json` の `leaf` 値に一致することを確認済み（`cargo run --example test_poseidon`）。circom 既知ベクタ `poseidon([1])` / `poseidon([1,2])` とも一致。
  - 対応: leaf = `new_circom(3).hash(&[Fr::ZERO, secret, salt])`（先頭 0 はドメインタグ、circom の `Poseidon([0,secret,salt])` と同一 state）。節 = `new_circom(2).hash(&[L, R])`。`hash(&mut self)` は毎回 `state.clear()` するのでインスタンス使い回し可。
- [x] `src/merkle.rs` の `hash_leaf`/`hash_pair` を SHA-256 → `pso-poseidon` に差し替え（`a8d5abc`）。`type Hash = Fr`（案A）、`EMPTY_HASH = Fr::ZERO`（`use ark_ff::AdditiveGroup`）、`hash_leaf(secret: Fr, salt: Fr) -> Hash` = `new_circom(3).hash(&[Fr::ZERO, secret, salt])`、`hash_pair` = `new_circom(2)`。
- [x] `from_leaves` を固定深さ版 `from_leaves(leaves: Vec<Hash>, levels: usize)` に（`a8d5abc`）。`(1 << levels)` 枚まで `EMPTY_HASH` 埋め、畳み込みちょうど `levels` 回、`layers[0]` は葉。`proof()` / `verify_proof` は `Fr` 化で自動対応。
  - padding-forgery 再点検済み: 固定深さ4＋固定 `Fr::ZERO` 埋めなら木の形は葉数によらず 16→8→4→2→1 で一定 → N/N+1 別 root。`Fr::ZERO` が `Poseidon([0,s,salt])` 出力と衝突する確率は無視可。→ 条件クリア。[[merkle-padding-forgery-deferred]]
- [ ] **次1**: `src/merkle.rs` のテスト3本を書き換え（今は旧 API でコンパイル不可）。
  - A: `MEMBERS`（`(101,9001)`…）→ `hash_leaf(Fr::from(s), Fr::from(salt))` → `from_leaves(leaves, 4)` → `root() == Fr::from_str("17396252…816")`（`use core::str::FromStr`）
  - B: `circuits/input.json`（`TARGET_INDEX=2`）と `tree.proof(2)` を突き合わせ。`pathIndices [0,1,0,0]` → 各 `.1` が `[false,true,false,false]`、`siblings` 4個 → 各 `.0` が一致、`layers[0][2]` が `input.json` の `leaf` と一致。← proof() 順序の決定的検証
  - C: `Fr` 葉で `verify_proof` roundtrip ＋ 非メンバーで `false`
- [ ] **次2**: `src/main.rs` — `[u8;32]` 乱数 → `Fr::from_le_bytes_mod_order`（確定判断#2）で secret/salt、`hash_leaf(secret, salt)` → `from_leaves(leaves, 4)`。旧 `hash_leaf(b"abc")` / `from_leaves(leaves)` の呼び出しを全部直す。CLI の root を回路入力に接続。

### 既知の小物

- [x] `src/main.rs` の `i` 未使用 warning → `for _ in` で解消。
- [x] `src/proof.rs:1` `Bn254` 未使用 warning → proof/verify で使用中のため解消。
- [x] `src/proof.rs` 末尾の `build_witness()` コメントアウト残骸を削除。
- [ ] Step 6 に World ID 統合のサブタスクを明記（SPEC §7 に無い）: オンボーディング UI に IDKit、Registry のメンバー登録で World ID nullifier をオンチェーン検証してから leaf 追加。

## 別PCでの再開手順

```bash
git pull
cd circuits
npm ci                       # circomlib / circomlibjs（node_modules は gitignore 済み）
circom main.circom --r1cs --wasm --sym -l node_modules   # main_js/ と main.r1cs は gitignore、要再生成
node scripts/build_input.js                              # circuits/input.json 再生成 + root 表示
node main_js/generate_witness.js main_js/main.wasm input.json witness.wtns
snarkjs groth16 prove main_final.zkey witness.wtns proof.json public.json
snarkjs groth16 verify verification_key.json public.json proof.json   # → OK! で環境OK
```

Rust 側:

```bash
cd ..            # クレートルート（cargo test の cwd が相対パス circuits/... の前提）
cargo test       # ※ a8d5abc 時点ではビルド赤（main.rs / merkle テストが旧 API）。
                 #   TODO「次1・次2」を片付けると緑（proof 2 + merkle 3 想定）
cargo run --example test_poseidon   # Poseidon ゲートは単体で緑（回路 root を再現）
cargo run        # 上記 TODO 完了後: メンバー生成→Merkle→setup→prove→verify、root と verify=true
```

### git 管理の方針（2026-09-08 整理済み）

- **追跡する**: `main.circom` / `scripts/` / `WORKFLOW.md` / `package.json` / `package-lock.json` / `circuits/input.json` / 共有鍵 `main_final.zkey` `verification_key.json` `pot12_final.ptau`
- **gitignore（各PCで再生成）**: `node_modules/` / `main_js/` / `main.r1cs` / `main.sym` / 中間 ptau / `main_0000.zkey` / `witness.*` / `proof.json` / `public.json`
- 回路を変えたら zkey/vkey は作り直して**両方コミット**（[WORKFLOW.md](../circuits/WORKFLOW.md) の「0→2」）。1つの鍵を両PCで共有するのが原則。

### ツール版

circom 2.2.3（`~/.cargo/bin`）/ snarkjs 0.7.6 / Node は nvm 管理（このWSLは v22.23.2）/ bn128 / `pot12`（2^12=4096、回路は ~2080 制約）。
