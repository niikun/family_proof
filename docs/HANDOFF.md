# HANDOFF — 別PCへの引き継ぎ

最終更新: 2026-09-09 / ブランチ: `main` / remote: `git@github.com:niikun/family_proof.git`

## いまどこ

ロードマップ（[SPEC.md](SPEC.md) §7）で **Step 0・1・2 完了、Step 3 着手（witness 生成まで動作）**。全体 ≈ 30%。

| Step | 状態 |
|---|---|
| 0 Rust Merkle 骨格（ZKなし） | ✅ `cargo test` |
| 1 circom サンプル写経・compile→prove→verify | ✅ WSL でも全パイプライン疎通 |
| 2 MVP 回路を自ユースケースへ | ✅ circom 側 done / Rust パディングを `EMPTY_HASH` 固定に（commit `9c52de4`） |
| 3 `ark-circom` で Rust から proof 生成・検証 | 🟡 着手。`src/proof.rs` `build_witness()` が r1cs+wasm を読んで witness 計算 → public inputs 取得、`test_build_witness` 緑。proof 生成・検証はこれから |
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

## ⚠️ 切り替え前に必ずやること

未コミット（コミット `ff0535c` の先）。**コミット＆push してから**別PCで pull すること。

```
Cargo.toml / Cargo.lock  (M)  ← ark-circom, color-eyre を追加
src/main.rs              (M)  ← mod proof; を追加
src/proof.rs             (??) ← 新規。build_witness()
```

`origin/main` より 1 コミット先行しているので、コミット後 `git push` を忘れずに。

## 残タスク（TODO）

### Step 3（いまここ・次の本丸）

- [x] `ark-circom` / `color-eyre` を `Cargo.toml` に追加。
- [x] `src/proof.rs` `build_witness()` — `circuits/main.r1cs` と `circuits/main_js/main.wasm` を読み、`circuits/input.json` を入力に witness 計算 → `get_public_inputs()`。`test_build_witness` で root の期待値一致を確認。
  - ※ テストの `#[cfg(test)]`（`tests` ではない）と、`cargo test` の cwd がクレートルート＝相対パス `circuits/...` が実在すること前提。
- [ ] **次**: `build_witness()` を伸ばして Groth16 proof 生成・検証まで。関数を `setup() -> pk` / `prove(pk) -> (proof, public_inputs)` / `verify(vk, public_inputs, proof) -> bool` に分ける。
  - arkworks 側で鍵生成する場合: `Groth16::<Bn254>::generate_random_parameters_with_reduction(circom.clone(), &mut rng)` → `create_random_proof_with_reduction(circom, &pk, &mut rng)` → `process_vk` + `verify_with_processed_vk`。
  - API 名は ark-groth16 0.6 で変わっている可能性あり。docs.rs で `Groth16` trait を要確認。
  - 既存の trusted setup（`circuits/main_final.zkey`）を使うなら `ark_circom::read_zkey` に寄せる判断（デモは arkworks 生成で可）。
- [ ] Rust CLI から「オンボーディング（`(secret,salt)` 生成 → Merkle Tree → root）」→「proof 生成」→「検証」を一連で実行できる状態に。
- [ ] `src/merkle.rs` の `hash_leaf`/`hash_pair` は現状 **SHA-256**。回路と一致させるため Poseidon に差し替える（arkworks 系の Poseidon、パラメータを circomlib と合わせる必要あり — ここは要調査）。
- [ ] `from_leaves` を固定深さ版（`from_leaves(leaves, levels)`、`1<<levels` まで `EMPTY_HASH` 埋め）に。回路の `nLevels=4` と合わせる。→ この変更時に merkle padding の N/N+1 衝突を再点検（memory `merkle-padding-forgery-deferred`）。

### 既知の小物

- [ ] `src/main.rs` `for i in 0..FAMILY_MEMBERS` の `i` 未使用 warning（`_i` か `for _ in`）。
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

### git 管理の方針（2026-09-08 整理済み）

- **追跡する**: `main.circom` / `scripts/` / `WORKFLOW.md` / `package.json` / `package-lock.json` / `circuits/input.json` / 共有鍵 `main_final.zkey` `verification_key.json` `pot12_final.ptau`
- **gitignore（各PCで再生成）**: `node_modules/` / `main_js/` / `main.r1cs` / `main.sym` / 中間 ptau / `main_0000.zkey` / `witness.*` / `proof.json` / `public.json`
- 回路を変えたら zkey/vkey は作り直して**両方コミット**（[WORKFLOW.md](../circuits/WORKFLOW.md) の「0→2」）。1つの鍵を両PCで共有するのが原則。

### ツール版

circom 2.2.3（`~/.cargo/bin`）/ snarkjs 0.7.6 / Node は nvm 管理（このWSLは v22.23.2）/ bn128 / `pot12`（2^12=4096、回路は ~2080 制約）。
