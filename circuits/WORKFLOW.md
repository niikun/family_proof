# family_proof — Groth16 回路フロー

Merkle 包含証明回路 [`main.circom`](main.circom)（`MerkleTreeInclusionProof(4)`）を
コンパイルして証明・検証するまでの手順。作業ディレクトリは `circuits/`。

環境: circom 2.2.3 / snarkjs 0.7.6（グローバル） / 曲線 bn128。

---

## 回路の構造

| signal | 種別 | 説明 |
|---|---|---|
| `leaf` | input | 証明したい葉の値 |
| `pathIndices[4]` | input | 各階層で自分が左(`0`)か右(`1`)か。回路が `x*(1-x)===0` を課す |
| `siblings[4]` | input | 各階層の隣ノードのハッシュ |
| `root` | output | `leaf` から Poseidon(2) を4回通した結果。**output なので常に public** |

- 制約数: 約 2080（`snarkjs r1cs info main.r1cs` で確認）。`pot12`（2^12=4096）で足りる。
- `root` を既知値と照合する制約は**回路内に無い**。検証側が `public.json` の `root` を
  正しい木のルートと突き合わせる前提。
- `component main {public [...]}` に書けるのは **input のみ**（output は自動 public）。
  プライバシーを保つなら `leaf` / `siblings` / `pathIndices` は public リストに入れない。

---

## 0. コンパイル

```bash
circom main.circom --r1cs --wasm --sym -l node_modules
```

生成物:
- `main.r1cs` … 制約系（証明系が扱う形）
- `main_js/main.wasm` … 入力から witness を計算するプログラム
- `main.sym` … signal 名の対応表（デバッグ用）

**回路を編集したら 0 と 2 をやり直す。** 1（ptau）は再利用可。

---

## 1. Powers of Tau（Phase 1 / 回路非依存）

どんな回路にも使い回せる共通セットアップ。`pot12_*.ptau` が既にあれば省略可。

```bash
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="1st" -v
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v
```

- `12` = 2^12 制約まで。回路が超えたら上げる。
- `contribute` は秘密乱数を混ぜて直後に破棄する（toxic waste）。1人でも正直なら安全。

---

## 2. Phase 2（回路ごと / zkey）

`main.r1cs` と `pot12_final.ptau` から、この回路専用の鍵を作る。

```bash
snarkjs groth16 setup main.r1cs pot12_final.ptau main_0000.zkey
snarkjs zkey contribute main_0000.zkey main_final.zkey --name="1st" -v
snarkjs zkey export verificationkey main_final.zkey verification_key.json
```

- `main_final.zkey` … prover 用の鍵
- `verification_key.json` … verifier 用。公開してよい
- **回路を変えたらこの zkey は無効。必ず作り直す。**

---

## 3. 入力（main_js/input.json）

`nLevels = 4` なので配列は長さ4。数値は **10進の文字列**。

```json
{
  "leaf": "12345",
  "pathIndices": ["0", "1", "1", "0"],
  "siblings": ["11111", "22222", "33333", "44444"]
}
```

- `pathIndices[i]` は `"0"` か `"1"` のみ（それ以外は制約違反でエラー）。
- 上は疎通確認用のダミー。実運用では実際の Poseidon Merkle ツリーを JS で構築し
  （`circomlibjs` の `buildPoseidon` など）、対象の葉の `pathIndices` / `siblings` を
  木から取り出して出力する。

---

## 4. witness 計算

```bash
node main_js/generate_witness.js main_js/main.wasm main_js/input.json witness.wtns
```

`witness.wtns` に全 signal の値が入る。中身確認は
`snarkjs wtns export json witness.wtns witness.json`。index 1 が `root`。

---

## 5. 証明生成

```bash
snarkjs groth16 prove main_final.zkey witness.wtns proof.json public.json
```

- `proof.json` … 3つの楕円曲線点（π_A, π_B, π_C）
- `public.json` … public signal の値。最低でも `root`

---

## 6. 検証

```bash
snarkjs groth16 verify verification_key.json public.json proof.json
```

`[INFO] snarkJS: OK!` で成功。`verification_key.json` + `public.json` + `proof.json` の
3ファイルだけで検証できる（witness も zkey も不要）。

---

## 7.（任意）オンチェーン検証

```bash
snarkjs zkey export solidityverifier main_final.zkey verifier.sol
snarkjs zkey export soliditycalldata public.json proof.json
```

- `verifier.sol` … `verifyProof(...)` を持つコントラクト
- 2つ目が吐く calldata をそのまま `verifyProof` に渡す

---

## 回路変更後の最小手順

`0 → 2 → 4 → 5 → 6`（ptau は再利用）。
