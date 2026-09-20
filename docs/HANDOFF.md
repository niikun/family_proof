# HANDOFF — 別PCへの引き継ぎ

最終更新: 2026-09-19（Step 6 着手・Foundry環境構築） / ブランチ: `main` / remote: `git@github.com:niikun/family_proof.git` / 同期: **未コミットあり**（`Cargo.toml` `Cargo.lock` `circuits/*`（input.json/main.circom/build_input.js/新zkey一式/verifier.sol）`docs/SPEC.md` `src/*` `contracts/`（新規、Foundryプロジェクト）。`git add -A && git commit && git push` で `origin/main` と一致させる

> ✅ **Step 4（RLN）完了・Step 6 大きく進行**。`FamilyRegistry.sol` 実装＋ユニットテスト7本＋本物の証明データでの統合テスト（`forge test` 8本緑）に加え、**Rust側の鍵ミスマッチも解消**（`proof::setup()` が実際の `main_final.zkey` を読み込む形に、`prove()` は `CircomReduction` 指定に修正）。**これで Rust CLI が作る証明はそのまま on-chain の Verifier で検証できる状態になった**（`cargo test` 8本緑・`cargo run` 7人全員 `verify=true`）。**さらに `proof::to_solidity_calldata()`（G1/G2 → Solidity `uint256[2]`/`uint256[2][2]` 変換）を実装・`main.rs` に組み込み済み**。`anvil` にデプロイした本物の `Groth16Verifier.sol` に `cast call` で実測検証し、正しい変換であることを確認済み（下記「Step 6」節に詳細）。残るは testnet デプロイ・通知インフラ・匿名統計公開。
> **⚠️ 重要: ETHGlobal Tokyo 2026 の日程・提出ルールが確定済み（下記参照）。この5連休の位置づけが変わったので必読。**

## ⚠️ ETHGlobal Tokyo 2026 日程・提出ルール（2026-09-19 確認）

- **イベント本体は 9/25〜27**。提出締切 **9/27 9:00 JST**、遅延提出不可。審査は7分（デモ4分＋Q&A3分）、基準は technicality / originality / practicality / UX / "WOW factor"
- **Continuity Track での提出が必須**。"Classic From Scratch" はイベント開始後（9/25〜）に書いたコードのみが対象で、Step 0〜4（9/6〜9/19に実装済み）は対象外になってしまう → Continuity Track を選び、「イベント前からの既存部分」と「イベント中に新規に作った部分」を明確に書き分けて提出する
- **AIツール利用ポリシー**: AI支援は許可されるが、人間の実質的な貢献を示しつつ明記が必要。[[no-writing-code]]（Claude はコーチのみ、コードは全部ユーザーが書く）の運用がそのままこの要件を満たす — README/提出文に明記すること
- デモ動画（2〜4分、720p以上）は任意だが推奨。倍速・電話撮影・テキストのみ+音楽・AIナレーションは禁止

**改訂後の進め方（2026-09-19時点の推奨）**: 今日から9/24（イベント前日）までを「Continuity Track の“既存部分”」の仕上げに使い、**Step 6 のコア（Verifier.sol / Registry / テストネットデプロイ）をここで完成させる**。**イベント本番（9/25〜27）は「期間中に新規に作った部分」として World ID 連携・Step 5 デモUI・統合・デモ動画・Continuity提出文の執筆に充てる。** これで「動くもの」と「イベント中に作った説得力のあるストーリー」の両方を確保する。

## いまどこ

ロードマップ（[SPEC.md](SPEC.md) §7）で **Step 0〜4 完了、Step 6 進行中**。全体 ≈ 81%（ウェイト: 0=5/1=5/2=7/3=13/4=25/5=15/6=30、Step6は現在≈85%進捗）。

| Step | 状態 |
|---|---|
| 0 Rust Merkle 骨格（ZKなし） | ✅ `cargo test` |
| 1 circom サンプル写経・compile→prove→verify | ✅ WSL でも全パイプライン疎通 |
| 2 MVP 回路を自ユースケースへ | ✅ circom 側 done / Rust パディングを `EMPTY_HASH` 固定に（commit `9c52de4`） |
| 3 `ark-circom` で Rust から proof 生成・検証 | ✅ **完了**。CLI の Merkle root が回路の public root と一致することまで実証済み |
| 4 RLN（§6.2） | ✅ **完了**（2026-09-19）。回路実装・Rust配線・2点復元テストまで完走。詳細は下記「Step 4」節 |
| 5 デモ UI | ⬜ **イベント本番（9/25〜27）に着手する方針** |
| 6 on-chain（+ World ID ゲート） | 🟡 **進行中（≈85%）**。Verifier/Registry/ユニットテスト/本物データでの統合テスト/Rust鍵統一/calldata変換/**World Chain Sepoliaへのデプロイ**まで済み。**残: 通知インフラ・匿名統計公開**。詳細は下記「Step 6」節 |

**スコープ方針（2026-09-19 更新・再更新）**: Must = Step 4 RLN（済） → **Step 6 コア**（Verifier.sol + Registry + testnet デプロイ、9/19〜9/24中に完成させる） → イベント本番で Step 5 最小デモ + World ID 連携。Cut候補 = ENS 名解決・levels=20拡張。World ID は「Should」ではなく**イベント本番の目玉（下記 World ID 音声対策）**に格上げ — Step 6 コア完了後に着手する。

## 🆕 第2の差別化ポイント: World ID live-challenge による AI音声クローン対策（2026-09-19 採用、"Option A"）

RLN（盗んだ secret の使い回し検知）と**脅威モデルを分離**。RLN が防げない攻撃 — 攻撃者が secret を知らずとも AI で声をクローンしてなりすます攻撃 — をこちらで防ぐ。「家族の合言葉」という定番の詐欺対策を、暗号的に堅牢な形にする、というピッチ。

- **絶対に守る設計制約**: RLN の `challenge`/`epoch` を、この生の合言葉に**流用しない**。流用すると、正規メンバーが同一 epoch 内に別々の合言葉で2回正規の通話をしただけで RLN の自己暴露が発動し、secret が漏れてしまう。**完全に分離**すること — 合言葉は World ID の `signal` フィールド（`signal = hash(code)`）にのみ紐付ける。circom 側の変更は不要
- **検証は on-chain ではなく off-chain**。通話中にブロック確定を待つのは非現実的。フロー: 親が合言葉を読み上げる → 子の端末が (a) `signal=hash(code)` の World ID proof と (b) いつもの RLN proof（別物）を作成 → 親の端末が両方を off-chain 検証し、FamilyProof 側の結果は on-chain Registry の状態（現在の root・失効状況）と突き合わせる。Registry が唯一の正本、通話中の検証はそれに対する高速な off-chain チェック
- World ID には実機 Orb 不要の Simulator/staging モードがある → デモは親子2役のブラウザ mini-app で十分（電話回線は不要）
- スコープリスク: これまで Rust + circom + Solidity だけだったスタックに JS/Web フロントエンド（IDKit widget）が新たに加わる。**Step 6 コア完了後に着手**すること
- 競合調査（2026-09-19時点）: ZK × World ID をオレオレ詐欺対策に組み合わせた既存プロジェクトは見つからず、独自性は高そう
- **ピッチの軸（2026-09-19 追加）**: 「合言葉にZKを足した」だけだとありがちなハッカソン構成（既存の地味な対策＋暗号を足すパターン）に見えるリスクがある。差別化点は技術要素そのものではなく問題設定の切り口 — 「秘密を知っているだけ」では AI 音声クローンに突破される、だから knowledge の証明(RLN)だけでなく liveness の証明(World ID live-challenge)が要る、という脅威分離のロジックをピッチの中心に置く
- **AI音声クローンデモ（2026-09-19 採用）**: 上記ロジックを言葉でなく体験として見せるため、Step 5 のデモに「クローン音声が正しい合言葉を言っても live-challenge で弾かれる」シーンを追加する。音声クローンは同意を得たチームメンバー本人の声でイベント前に事前生成（ライブ生成は音声合成の失敗・レイテンシ等のデモ事故リスクが高いため）、デモ本番ではその音声の再生＋検知部分のみライブで行う。RLN単体のデモ（漏洩secretの使い回し検知）とは別シーンとして構成。詳細は [SPEC.md](SPEC.md) Step 5 / §9 に反映済み

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

### Step 6（on-chain、進行中 — 2026-09-19着手）

- [x] **Foundry 導入**。`curl -L https://foundry.paradigm.xyz | bash` → `foundryup` で `forge`/`cast`/`anvil` v1.8.3 インストール。**PATH は `~/.bashrc` に `export PATH="$PATH:$HOME/.foundry/bin"` を追記する必要あり**（まだ追記してなければ別PCでも同様に必要）。
- [x] **[contracts/](../contracts) に Foundry プロジェクト新規作成**（`forge init contracts --no-git`、サンプルの `Counter.*` は削除済み）。
- [x] **zkey/vkey/verifier.sol を RLN回路向けに作り直し**。旧鍵（9/8時点、RLN追加前）は無効なので `circuits/` で再実行:
  ```
  snarkjs groth16 setup main.r1cs pot12_final.ptau main_0000.zkey
  snarkjs zkey contribute main_0000.zkey main_final.zkey --name="1st" -v
  snarkjs zkey export verificationkey main_final.zkey verification_key.json
  snarkjs zkey export solidityverifier main_final.zkey verifier.sol
  ```
  - `nPublic: 5`（root, y, nullifier, epoch, challenge）で想定通り。
  - ⚠️ **制約数 4033 / pot12 上限 4096 でギリギリ**。今後回路を増やすなら `pot13` への引き上げが先に必要。
  - `verifier.sol` → [contracts/src/Groth16Verifier.sol](../contracts/src/Groth16Verifier.sol) にコピー。`function verifyProof(uint[2] _pA, uint[2][2] _pB, uint[2] _pC, uint[5] _pubSignals) public view returns (bool)`。
- [x] **`IGroth16Verifier.sol`**（interface）を追加。`FamilyRegistry` は具象型 `Groth16Verifier` ではなくこの interface 型で verifier を保持 — テストで Mock に差し替え可能にするため。
- [x] **`FamilyRegistry.sol`** 実装（[contracts/src/FamilyRegistry.sol](../contracts/src/FamilyRegistry.sol)）。
  - `familyRoot`（owner のみ `updateRoot()` で更新）
  - `verifyMembership(pA,pB,pC,pubSignals)`: ① `pubSignals[0]==familyRoot` ② `pubSignals[3]==block.timestamp/1 hours`（epoch鮮度、SPEC §6.2 追記済みの穴の対策） ③ `verifier.verifyProof(...)` の3点を `require` → `seenNullifiers[nullifier]` を見て: 未登録なら記録、同一challengeなら `revert`（リプレイ拒否）、別challengeなら `PotentialLeak` イベント発行（`revert` はしない — その場の確認自体は成立させる。secret復元は on-chain ではやらない設計。下記参照）
  - **secret 復元は on-chain ではやらない**: Poseidon を Solidity に実装するコストが高いため。on-chain は衝突検知とイベント発行まで、実際の復元計算（`x=Poseidon(challenge)`、`recover_secret`）は Rust 側（`proof.rs` に実装済み）に任せる off-chain 通知スクリプトの仕事にする
  - **失効（revocation）は「leaf除外」ではなく「secret公開+イベント」までが on-chain の限界**: `secret` が復元できても `salt` は分からないので leaf を特定できない（確定判断#1の設計上の帰結）。実運用は「family admin がイベントを見て該当メンバーの secret/salt を再発行し、木を組み直して `updateRoot()`」という運用でカバーする
- [x] **`contracts/test/FamilyRegistry.t.sol`** — Mock Verifier（常に `true`）を使ったユニットテスト7本、全緑（`forge test -vv`）: 初回証明成功 / 同一challengeリプレイでrevert / 別challengeで`PotentialLeak`発行 / root不一致でrevert / epoch不一致でrevert / **`updateRoot`をowner が呼べば成功＋`RootUpdated`発行 / owner以外が呼ぶとrevert**。
  - ハマりどころ: Foundry のテスト環境は `block.timestamp` がデフォルト `1`（＝`epoch=0`）。「古いepoch」として `0` を使うと現在の epoch とたまたま一致してテストが誤通過する。`block.timestamp/1 hours + 大きい定数` のような動的な値を使うこと。
  - `vm.prank(address)` で次の1呼び出しだけ `msg.sender` を差し替えて owner以外からの呼び出しを再現。
- [x] **統合テスト（本物の証明データ、Mock でなく実 `Groth16Verifier` 使用）**。`contracts/test/FamilyRegistryIntegration.t.sol`。`circuits/` で `snarkjs groth16 prove main_final.zkey witness.wtns proof.json public.json` → `snarkjs zkey export soliditycalldata public.json proof.json` で本物の `(pA,pB,pC,pubSignals)` を取得しテストにハードコード。`vm.warp(472223 * 1 hours)` で証明に埋め込まれた epoch と一致させ、`familyRoot` は `pubSignals[0]`（本物のroot）で初期化。**本物のペアリング検証を通過**（circom回路→snarkjs鍵→Solidity Verifier→Registryが実際に繋がっていることの最終証明）。`forge test` 全体で8本緑（unit 7 + integration 1）。
- [x] **Rust側の鍵ミスマッチ解消**。`proof::setup()` を `circuit_specific_setup` から `ark_circom::read_zkey()` で `circuits/main_final.zkey` を読み込む形に置き換え（引数なし `setup() -> (ProvingKey, PreparedVerifyingKey)` に変更、`build_setup_circuit()` は不要になり削除）。
  - **ハマりどころ（重要）**: これだけだと `test_prove_verify` が `verify` で失敗する。原因は `prove()` が `Groth16::<Bn254>::prove(...)`（arkworksデフォルトのQAP変換）のままだったこと。**snarkjs/circom生成の鍵を使うときは `Groth16::<Bn254, ark_circom::CircomReduction>::prove(...)` と reduction 型を明示する必要がある**（ark-circom 自身のテスト `zkey.rs::verify_proof_with_zkey_with_r1cs` で確認した正しい書き方）。`verify()`/`setup()` 側は `CircomReduction` 不要（検証側の計算は reduction に依存しない）。
  - `cargo test` 8本緑、`cargo run` で7メンバー全員 `verify=true`（**この鍵は on-chain の `Groth16Verifier.sol` と同一**なので、これで作った証明はそのまま on-chain でも検証できる状態）。
- [x] **calldata変換（2026-09-20 完了）**: `proof::to_solidity_calldata(proof) -> ([String;2], [[String;2];2], [String;2])`（[src/proof.rs](../src/proof.rs)）を実装、`main.rs` の7人ループに組み込み済み。
  - **ハマりどころ（重要）**: G1（`a`,`c`）は `x,y: Fq` なので `proof.a.x.into_bigint().to_string()` のように座標ごとに変換すればよいが、G2（`b`）は座標自体が `Fq2`（`x.c0,x.c1,y.c0,y.c1`）になっている。最初の実装は `[[x.c0,y.c0],[x.c1,y.c1]]` のように**x/yを跨いでc0同士・c1同士でグループ化してしまうバグ**があり、これは一見自然に見えるが誤り。正しくは `[0]`=x座標のペア・`[1]`=y座標のペアで分け、かつ snarkjs 生成の Verifier の慣習に合わせて各ペア内は `[c1, c0]` の順（`[[x.c1,x.c0],[y.c1,y.c0]]`）。
  - **検証方法**: `anvil` を起動 → `forge create` で本物の `Groth16Verifier.sol` をデプロイ → Rustで生成した実際の証明を `to_solidity_calldata` で変換し `cast call verifyProof(...)` で直接叩いて `true` が返ることを実測確認（4パターンの組み合わせを試して切り分けた）。`ark_ff::BigInt<N>` は `Display` 実装が内部で `num_bigint::BigUint` 経由の10進文字列を返すため、`.into_bigint().to_string()` だけで Solidity `uint256` にそのまま渡せる文字列になる（`num_bigint` を明示的にimportする必要はない）。
  - `cargo run` で7人全員 `verify=true` を維持したまま、calldata出力も正しく得られることを確認済み。
- [x] **World Chain Sepolia へのデプロイ（2026-09-20 完了）**。`cast wallet` のキーストア（`~/.foundry/keystores/deployer`）を使い `forge create --account deployer` で2件デプロイ（RPC: `https://worldchain-sepolia.g.alchemy.com/public`, chainId 4801）。
  - **Groth16Verifier**: `0x132a7dbd30784d2283b83D96BD45B731AF331c8a`
  - **FamilyRegistry**: `0xD06FcbB5CB9D3B094874855d3979C8ae8eA09144`（constructor: `verifierAddress`=上記, `initialRoot`=`13298919588855972999610419539218897354041933303975356579871528834317655849272`。この root は `secret="103"`/`salt="9003"` を5枚複製・depth4の木の root で決定論的に再現可能）
  - デプロイ後 `cast call` で `verifier()`/`owner()`/`familyRoot()` が期待値と一致することを確認済み
  - **ハマりどころ**: `cast wallet address --account deployer` はキーストア復号にパスワード入力が要るが、非対話環境（TTYなし）だと `No such device or address (os error 6)` で失敗する。パスワードが必要なコマンド（`cast wallet address`/`forge create --account`）は本人のターミナルで直接実行する運用にした
- [ ] **通知インフラ（設計済み・未着手）**: `SecretRevealed`/`PotentialLeak` イベントを監視して該当メンバーにメール通知。Rust の `alloy`（EVMログ取得・デコード）+ `reqwest`（Resend の REST API）+ `tokio` で `src/bin/notifier.rs` として実装する方針（Node.js ではなく Rust で統一）。デプロイ済みコントラクトが無いと作れないので Step 6 コア完了後に着手。
- [ ] **匿名統計の公開（2026-09-20 採用・設計済み・未着手）**: 上記 `notifier.rs`（`alloy` でのイベント監視基盤）を流用し、`RootUpdated`/`PotentialLeak` を集計して「日次の検知件数」だけを公開する。family root・address 等の個人/家族を特定できる情報は公開側に一切出さない（日次カウントのみ、個別イベント単位の時刻・アドレスは出さない）。目的は「表面化しづらいオレオレ詐欺の試行実態を、被害者・家族を特定せずに可視化する」こと。ピッチの Practicality/社会的インパクトの補強にもなる。詳細は [SPEC.md](SPEC.md) Step 6 に反映済み
  - **やらないと決めたこと**: 詐欺の手口（通話内容）をAIが要約して統計化する案は今回のスコープ外。現状の暗号設計は通話内容を一切扱わないため、実現には報告フォーム等の新規データ収集経路がゼロから必要になり、9/24までのコア完成を圧迫する。ピッチの「将来構想」スライドで触れる程度に留める（[SPEC.md](SPEC.md)「時間が余った場合の拡張候補」に記載済み）

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

Solidity 側（Foundry、初回は各PCで PATH 設定が要る）:

```bash
export PATH="$PATH:$HOME/.foundry/bin"   # ~/.bashrc に追記推奨。無ければ curl -L https://foundry.paradigm.xyz | bash && foundryup
cd contracts
forge build      # Groth16Verifier.sol / FamilyRegistry.sol / IGroth16Verifier.sol
forge test -vv   # ユニットテスト5本緑（Mock Verifier使用）
```

### git 管理の方針（2026-09-08 整理済み・2026-09-19 追記）

- **追跡する**: `main.circom` / `scripts/` / `WORKFLOW.md` / `package.json` / `package-lock.json` / `circuits/input.json` / 共有鍵 `main_final.zkey` `verification_key.json` `pot12_final.ptau` / **`circuits/verifier.sol`（新規追加）** / **`contracts/`（Foundryプロジェクト一式。`contracts/lib/forge-std` は `forge init` が clone する外部依存、サブモジュールとして扱うか通常ファイルとして追跡するかは commit 時に要確認）**
- **gitignore（各PCで再生成）**: `node_modules/` / `main_js/` / `main.r1cs` / `main.sym` / 中間 ptau / `main_0000.zkey` / `witness.*` / `proof.json` / `public.json`
- 回路を変えたら zkey/vkey は作り直して**両方コミット**（[WORKFLOW.md](../circuits/WORKFLOW.md) の「0→2」）。1つの鍵を両PCで共有するのが原則。

### ツール版

circom 2.2.3（`~/.cargo/bin`）/ snarkjs 0.7.6 / Node は nvm 管理（このWSLは v22.23.2）/ bn128 / `pot12`（2^12=4096、回路は ~2080 制約）。
