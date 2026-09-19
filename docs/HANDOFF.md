# HANDOFF — 別PCへの引き継ぎ

最終更新: 2026-09-19（Step 4 完了） / ブランチ: `main` / remote: `git@github.com:niikun/family_proof.git` / 同期: **未コミットあり（`Cargo.toml` `Cargo.lock` `circuits/input.json` `circuits/main.circom` `circuits/scripts/build_input.js` `docs/SPEC.md` `src/main.rs` `src/merkle.rs` `src/proof.rs`）**。`git add -A && git commit && git push` で `origin/main` と一致させる

> ✅ **Step 4（RLN）完了**。`main.circom` に `epoch`/`challenge`/`a1`/`x`/`y`/`nullifier` を制約として実装、Rust 側は `secret`/`salt`/`epoch`/`challenge` を回路に渡す配線＋2点からの secret 復元（`proof::recover_secret`）まで実装・テスト済み。`cargo test` 8本緑。
> **5連休（2026-09-19〜）でラストスパート、まだ Day1。次は Step 6（on-chain）— Day3終了ゲートに向けて先に進める。**

## いまどこ

ロードマップ（[SPEC.md](SPEC.md) §7）で **Step 0〜4 完了**。全体 ≈ 73%。

| Step | 状態 |
|---|---|
| 0 Rust Merkle 骨格（ZKなし） | ✅ `cargo test` |
| 1 circom サンプル写経・compile→prove→verify | ✅ WSL でも全パイプライン疎通 |
| 2 MVP 回路を自ユースケースへ | ✅ circom 側 done / Rust パディングを `EMPTY_HASH` 固定に（commit `9c52de4`） |
| 3 `ark-circom` で Rust から proof 生成・検証 | ✅ **完了**。CLI の Merkle root が回路の public root と一致することまで実証済み |
| 4 RLN（§6.2） | ✅ **完了**（2026-09-19）。回路実装・Rust配線・2点復元テストまで完走。詳細は下記「Step 4」節 |
| 5 デモ UI | ⬜ |
| 6 on-chain（+ World ID ゲート） | ⬜ **次はここ** |

**スコープ方針（2026-09-19 更新）**: 5連休が実質ラストチャンスのため、Must = Step 4 RLN → Step 6 on-chain（Verifier.sol + Registry + testnet デプロイ）→ Step 5 最小デモ（CLI可）。Should = World ID（IDKit）。Cut候補 = ENS 名解決・levels=20拡張。Day3終了時点（on-chain まで動く状態）をゲートにし、遅れていたら World ID を切ってコア強化に回す。（2026-09-08 時点の「両方チャレンジ」から更新）

## 確定済みの設計判断（蒸し返さない）

1. **leaf = `Poseidon([0, secret, salt])`** — 先頭 `0` はドメインタグ。`salt` はメンバーごとの2つ目の32バイト乱数、端末内に `secret` と一緒に保存、外部送信しない（Semaphore 型コミットメント / membership-oracle 対策）。`salt` は RLN には使わない。
2. **secret の表現** = 32バイト乱数を BN254 scalar field の素数 `r`（`21888242871839275222246405745257275088548364400416034343698204186575808495617`）未満に `mod r` した10進文字列。※いまの `MEMBERS` はまだ短いダミー文字列。
3. **パディング** = 固定空値。circom/JS 側は `EMPTY_LEAF = 0n`。Rust 側も固定値にする（「最後の葉を複製」は N と N+1 が同じ root を作れるので不可）。
4. **domain separation** = 案A の A1。葉のみタグ付け（`Poseidon(3入力)`）、節は `Poseidon([L,R])`（2入力）のまま。引数個数が違えば circomlib Poseidon は別インスタンスなので葉と節は別レンジ → **`main.circom` の変更は不要**。節側の対称タグ（`Poseidon([1,L,R])`）は Step 4 の回路書き直しに畳み込む。

詳細は memory（Claude 側）にも記録済み。

## 切り替え時のルール

Step 4 完了分（回路・JS・Rust一式）が未コミット。本ファイルと一緒に commit / push すれば `origin/main` と一致。
別PCでは `git pull` すればそのまま続きから入れる。

中断して別PCに移るときは毎回: `git status` で未コミットが無いか確認 → あれば
`git add` / `git commit` / `git push` してから離れる。zkey/vkey を作り直したら
それも忘れず commit（[WORKFLOW.md](../circuits/WORKFLOW.md) 参照）。

## 残タスク（TODO）

### Step 3（完了）

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
- [x] **済**: `src/merkle.rs` のテスト3本を新 API で書き換え。`MerkleTree` に `n_leaves` フィールド追加、`proof()` は `assert!(index < self.n_leaves)` でパディング枠 index を弾く（`Fr::ZERO` センサネル走査は不採用）。
  - `test_from_leaves` / `test_root`: 同一葉16枚 `hash_leaf(Fr::from(1), Fr::from(2))` → `from_leaves(_, 4)` → 手計算 a→b→c→d→e と `layers.last()[0]` / `root()` の一致
  - `test_proof_verify`: 葉7枚 `hash_leaf(Fr::from(i), Fr::from(1))` → 全 index で `verify_proof` roundtrip ＋ 非メンバー / depth 不一致で `false`
  - ⚠️ これらは**自己整合テスト**。回路 root との突き合わせは下記「次1」で追加する。
- [x] **済（未コミット）**: `src/main.rs` の3行を新 API へ。`let se:[u8;32]=rng.random(); Fr::from_le_bytes_mod_order(&se)`（`use ark_ff::PrimeField`、確定判断#2）で secret/salt → `hash_leaf(secret, salt)` 値渡し / `from_leaves(leaves.clone(), 4)` / 非メンバーは `hash_leaf(Fr::from(999u64), Fr::from(999u64))`。`cargo test` 5本緑、`cargo run` で `root=17396…816` / `verify=true`。
  - ⚠️ `cargo run` が出す `root` は `build_circuit()` が読む**固定 `circuits/input.json` 由来**であって、直前に組んだ乱数メンバーの木の root ではない（値が既知ベクタと同じなのは input.json がその5メンバー固定だから）。両者は未接続 → 「次2」で解消。
  - 小物 warning: `src/proof.rs:5` `SeedableRng` / `:6` `std::str::FromStr` が非テストビルドで未使用。`#[cfg(test)] mod test` 内へ `use` を移すと消える。
- [x] **済（未コミット）**: Rust ⇔ circom 一致テストを [merkle.rs](../src/merkle.rs) の `#[cfg(test)]` に2本追加。`examples/test_poseidon.rs` / `circuits/input.json` と同じ5メンバー `("101","9001")…("105","9005")`、`from_leaves(_, 4)` で 16 枚まで `EMPTY_HASH` 埋め。
  - **`test_verify_circom_root`（A: root 一致）**: `from_leaves(members, 4).root() == Fr::from_str("17396252260025783793058854431926620863655419045074533465745990270806947938816")`。`test_poseidon.rs` は独自ループ実装なので `from_leaves` 経由での突き合わせは別価値。
  - **`test_verify_circom_proof`（B: proof 順序）**: `input.json`（`TARGET_INDEX=2`）の `pathIndices`/`siblings` から手組みした `proof: Vec<(Hash,bool)>` で `verify_proof` が通ることに加え、**`assert_eq!(tree.proof(2), proof)`** で `MerkleTree::proof()` の実出力そのものが circom 規約と一致することも直接検証（最初のドラフトはここが抜けていて指摘・修正済み）。
  - `cargo test` 7本緑（proof 2 + merkle 5）、warning 0（`mem` 未使用 import と `pathIndices` snake_case も解消）。
- [x] **次2 完了（未コミット）**: CLI の Merkle root（[main.rs](../src/main.rs) の乱数メンバーの木）を回路入力に接続。`load_input_json`（固定 `circuits/input.json`）の代わりに `CircomBuilder::push_input` で `leaf`/`pathIndices`/`siblings` を直接投入する `proof::build_circuit_with_inputs(leaf, &path_indices, &siblings)` を追加。
  - 変換の要点: `push_input<T: Into<num_bigint::BigInt>>`。`Fr` は直接 `Into<BigInt>` ではないが `ark_ff::PrimeField::into_bigint()`（`Fr` → `ark_ff::BigInt<N>`）経由で `num_bigint::BigInt: From<ark_ff::BigInt<N>>` が効くので `fr.into_bigint()` をそのまま渡せる（文字列変換不要）。`bool` の `pathIndices[i]` は `as u64` でOK。
  - ハマりどころ: `Cargo.toml` に `num-bigint` を**独自バージョンで**足すと ark-circom 内部の num-bigint（0.4系）と型が unify されず `Into<BigInt>` を満たせない → **`num-bigint = "0.4"` に固定**（`push_input` の `BigInt` と同じ系列にする）。
  - `main.rs`: `tree.proof(i)` の `Vec<(Hash,bool)>` を `siblings`/`path_indices` に分解して `build_circuit_with_inputs` に渡し、`assert_eq!(pubs[0], root)` で接続を実証。7メンバー全員で成立確認済み。
  - **setup をループ外に**: `CircomBuilder::setup()`（ark-circom 側、witness なしの `CircomCircuit` を返す — `proof::setup`＝Groth16鍵生成とは別物）を使う `proof::build_setup_circuit()` を追加し、`proof::setup()` はループ前に1回だけ呼ぶ形に整理（7回 trusted setup → 1回）。
  - 旧 `build_circuit()`（固定 input.json 版）と `test_build_circuit`/`test_prove_verify` はそのまま残し、既知ベクタへの回帰確認用に維持。
- [x] `src/merkle.rs` テストの warning 掃除（`mem` 未使用 import、`pathIndices` → `path_indices` snake_case）。

### Step 4（RLN、完了 — 2026-09-19）

SPEC §6.2 の式（`a1 = Poseidon(secret,epoch)` / `x = Poseidon(challenge)` / `y = secret + a1・x` / `nullifier = Poseidon(a1)`）をそのまま実装。

- [x] **`circuits/main.circom` に RLN 制約を追加**。`signal input epoch, challenge`（public）、`signal input secret, salt`（private、`leaf` 直接入力は廃止）、`signal output y, nullifier` を追加。leaf は回路内で `Poseidon([0,secret,salt])` を計算する形に変更（確定判断#1 通り、外から leaf を渡す旧方式を廃止）。
  - `component main {public [epoch, challenge]} = MerkleTreeInclusionProof(4);` — public signal 指定の構文は `{public [...]}`（コロン無し）、template 引数と別枠。
  - `<==` は「witness計算＋制約」がセットの演算子（`<--` は計算のみ・制約なし）。tamper 実験（`y` を witness 計算後に+1して `cs.is_satisfied()` を見る、一時 `examples/` スクリプトで確認・削除済み）で `a1`/`x`/`y`/`nullifier` が実際に制約として効いていることを実証済み。
  - コンパイル結果: `public inputs: 2 / private inputs: 10 / public outputs: 3（root, y, nullifier）/ non-linear constraints: 1924`（RLN追加前は1248）。`pot12`（4096）の余裕内。
  - **`get_public_inputs()` の並び順** = output宣言順 → public input宣言順 = `[root, y, nullifier, epoch, challenge]`（`witness.json` で実測確認済み）。
- [x] **`circuits/scripts/build_input.js` を新スキーマへ更新**。`leaf` を書き出すのをやめ、`secret`/`salt`（対象メンバー）＋ `epoch`/`challenge`（ダミー値）を出力。木構築ロジック自体は変更なし。`circuits/input.json` 再生成、`build_circuit()`（旧APIのまま）とそのテスト（`test_build_circuit`/`test_prove_verify`）はこの新 input.json で無修正のまま緑に復帰（root の期待値・pubs[0] の位置は変わらず）。
- [x] **Rust 配線**: `proof::build_circuit_with_inputs` のシグネチャを `(leaf, path_indices, siblings)` → `(secret, salt, epoch, challenge, path_indices, siblings)` に変更、`push_input` で6種を投入。`main.rs` は `(secret, salt)` をメンバーごとに保持（`Fr: Copy` なので `hash_leaf` に渡した後も使い回せる）、`epoch` は実時刻（`SystemTime` → `floor(unixtime/3600)`）、`challenge` はデモ用固定値。7メンバー全員で `cargo run` 確認済み。
- [x] **secret 復元ロジック**: `merkle::hash_single(x: Fr) -> Fr`（`Poseidon(1)`、回路の `x = Poseidon(challenge)` を Rust 側でも再現するため）と `proof::recover_secret(x1,y1,x2,y2) -> Fr`（`a1=(y2-y1)/(x2-x1)`、`secret=y1-a1・x1`。`Fr: Field` の `.inverse()` で体の割り算）を追加。
  - `test_rln_secret_recovery`（`proof.rs`）: 同一 secret・同一 epoch・別 challenge で2回 `build_circuit_with_inputs` → `nullifier` 一致確認 → `recover_secret` で元の secret と一致することを確認。Groth16 の prove は不要（`get_public_inputs()` の witness 値だけで完結）。
  - `cargo test` 8本緑（+1、proof 3 + merkle 5）。
- [x] **SPEC.md に epoch 鮮度チェックの注記を追加**（§6.2）。回路は「今が何時か」を知らないので `epoch` は prover 自己申告の public input に過ぎない。検証側（Step 6 の Registry）で `epoch == floor(block.timestamp/3600)` 相当のチェックが必須。無いと (a) 古い証明のリプレイ (b) `epoch` を変え続けることで rate limit 自体を回避、が成立してしまう。**Step 6 の実装要件としてここに明記**。
- [x] **SPEC.md §8 に量子耐性の既知の限界を追加**。Poseidon は Grover で二次的減衰のみ（実用上ほぼ影響なし）、Groth16/BN254 は Shor で理論上破られる（証明の正しさの根拠がここに依存）。量子耐性のある証明系への移行はスコープ外と明記。

### 既知の小物

- [x] `src/main.rs` の `i` 未使用 warning → `for _ in` で解消。
- [x] `src/proof.rs:1` `Bn254` 未使用 warning → proof/verify で使用中のため解消。
- [x] `src/proof.rs` 末尾の `build_witness()` コメントアウト残骸を削除。
- [x] `src/proof.rs` の `SeedableRng` を `#[cfg(test)] mod test` 内へ移動、`std::str::FromStr` の冒頭 import は削除（テスト内では `ark_bn254::Fr` 経由で解決）。
- [ ] `src/proof.rs:30` `let leaf_str = leaf.to_string();`（未使用、`into_bigint()` に切り替えた際の残骸）を削除。
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
cargo test       # ビルド緑・8本パス（proof 3 + merkle 5）
cargo run --example test_poseidon   # Poseidon ゲートは単体で緑（回路 root 17396…816 を再現）
cargo run        # メンバー生成→Merkle→setup(1回)→7人分 prove→verify（secret/salt/epoch/challenge込み）。各人 assert_eq!(pubs[0], root) で CLI root と回路 root の一致を実証
```

### git 管理の方針（2026-09-08 整理済み）

- **追跡する**: `main.circom` / `scripts/` / `WORKFLOW.md` / `package.json` / `package-lock.json` / `circuits/input.json` / 共有鍵 `main_final.zkey` `verification_key.json` `pot12_final.ptau`
- **gitignore（各PCで再生成）**: `node_modules/` / `main_js/` / `main.r1cs` / `main.sym` / 中間 ptau / `main_0000.zkey` / `witness.*` / `proof.json` / `public.json`
- 回路を変えたら zkey/vkey は作り直して**両方コミット**（[WORKFLOW.md](../circuits/WORKFLOW.md) の「0→2」）。1つの鍵を両PCで共有するのが原則。

### ツール版

circom 2.2.3（`~/.cargo/bin`）/ snarkjs 0.7.6 / Node は nvm 管理（このWSLは v22.23.2）/ bn128 / `pot12`（2^12=4096、回路は ~2080 制約）。
