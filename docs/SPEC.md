# FamilyProof 仕様書

ETHGlobal Tokyo 2026 提出プロジェクト
「秘密を一度も明かさずに、家族であることを証明する」オレオレ詐欺対策

ステータス（2026-09-06 更新）: Step 0（ZK なしの Merkle 骨格）実装中。技術的差別化として
RLN（Rate-Limiting Nullifier）の導入を決定し、本仕様に反映済み。

---

## 1. 背景・課題

オレオレ詐欺（なりすまし電話詐欺）は、日本で被害額が高止まりしている社会問題である。

従来の対策（固定の合言葉、暗証番号など）には根本的な矛盾がある。「本人確認のために秘密を
言わせる」行為そのものが、電話越しに盗聴・録音されるリスクに秘密を晒すことになり、一度漏れると
その秘密は二度と使えなくなる。

また、詐欺対策アプリを作る会社が家族の秘密情報を中央サーバーに預かる設計にした場合、その
データベース自体が漏洩すれば「詐欺対策アプリが最大の攻撃対象になる」という皮肉な事態になる。

## 2. ソリューション概要

zk-SNARK（Groth16）を用い、以下を実現する。

- 各家族メンバーは端末内で秘密（secret）を生成し、外部に一切送信しない
- secret のハッシュを葉（leaf）とする Merkle Tree を家族単位で構築し、そのルート（root）
  だけを公開情報として扱う（将来的には on-chain に登録）
- 電話等でのなりすまし確認時、検証側が発行したワンタイムの challenge に対し、証明側は
  「secret が家族の Merkle Tree に含まれること」と「challenge に対する応答を正しく計算した
  こと」を、secret を一切明かさずに証明する（zk-SNARK）
- 詐欺師は secret を知らないため、有効な証明を生成できない
- 証明は毎回 challenge に紐付くため、過去の証明を盗聴・再利用（リプレイ攻撃）できない
- さらに RLN（Rate-Limiting Nullifier）により、同一 secret が 1 つの時間窓（epoch）内で
  規定回数（limit）を超えて使われると、その proof 群から secret が数学的に復元され、失効
  （revoke）できる。盗まれた secret を電話ごとに使い回す攻撃を「使うほど自壊する」構造にする

### この設計における ZK の必然性

単なる共有鍵ベースの日替わりパスワード（TOTP等）でも「詐欺師が言い当てられない」は実現できる。
しかし TOTP 型は検証側の端末も秘密（または秘密と同等の情報）を持たねばならず、結局「秘密を
どこかに置く」問題から逃れられない。

ZK 方式では **検証側は root（公開情報）だけを知っていればよく、secret を一切知らずに検証が
完結する**。これにより、詐欺対策アプリの提供者を含め、いかなる第三者も家族の秘密を預からずに
サービスを成立させられる。これが本プロジェクトにおける ZK の本質的な価値である。

## 3. スコープ

### 3.1 やること（MVP）

- circom で Merkle 包含証明の回路を実装
- Rust（`ark-circom` 等）で witness 生成・proof 生成・proof 検証を実装
- 家族オンボーディング（secret 生成 → Merkle Tree 構築 → root 出力）を Rust CLI または簡易 UI で実装
- RLN（Rate-Limiting Nullifier）を回路に組み込み、challenge 紐付け・リプレイ防止に加え、
  同一 secret の epoch あたり使用回数が limit を超えたら secret を復元・失効できるようにする
- なりすまし確認フローを Web デモ（2 画面：証明する側 / 検証する側）でシミュレーション
- 最小限の Solidity コントラクトで root 登録・proof 検証・RLN の nullifier/シェア記録と
  復元・失効を on-chain 実装

### 3.2 やらないこと（将来課題として明示）

- 本物の Android/iOS ネイティブアプリ化（着信スクリーニング連携、SMS 自動送受信）
- SP1 等 zkVM を用いたスマホ上でのリアルタイム proving（現時点では Groth16 に対して重く、
  リアルタイム UX に不向きと判断）
- SOS／脅迫時のデュレス（duress）用ダブルシークレット機能（機能過多と判断し拡張案として保留）
- 生体認証・Secure Enclave 連携
- RLN の epoch 長・limit の動的／適応的チューニング（デモは epoch = 1 時間 / limit = 1 に固定）

### 3.3 検討したが不採用にした案（記録として）

| 案 | 不採用理由 |
|---|---|
| QRコードでの証明提示 | 高齢者にとって操作ハードルが高い |
| 声を変換し、証明成功時のみ地声に戻す | 通話経路の乗っ取り・専用アプリ通話が前提になり実装コストが高い。デモ範囲を超える |
| 単純な日替わり合言葉（TOTP） | 検証側も秘密相当の情報を持つ必要があり、ZK の必然性を説明できなくなる |
| 着信時に検証側端末が自動で単語を表示 | 上記と同じ理由で、検証側が秘密を持つ前提になってしまう |

## 4. アーキテクチャ

```
[オンボーディング(一度だけ)]
  各メンバー: 端末内で secret を乱数生成（外部送信しない）
  leaf = Hash(secret)
  全メンバーの leaf から Merkle Tree 構築 → root
  → root を FamilyRegistry コントラクトに登録（改ざん防止の公開情報）

[なりすまし確認時]
  検証側: challenge（ランダム nonce）を発行。現在の epoch も定める
  証明側: secret + Merkle path + challenge + epoch から Groth16 proof を生成
          proof の public output として RLN のシェア (x, y) と nullifier が出る
  検証側: proof を受け取り、
          (a) ローカルで検証（オフライン・低遅延）
          (b) または on-chain の FamilyRegistry.verifyMembership() で検証
          さらに (epoch, nullifier) を記録。同 epoch で同 nullifier・別 x の
          proof が 2 つ出たら、2 点補間で secret を復元し当該 leaf を失効
  → ✅ 成功 = 家族グループの正規メンバー
    ❌ 失敗 = secret を持たない = なりすましの可能性
    ⚠ 同一 secret の使いすぎ = secret 露出 → 失効（盗難 secret の使い回し検知）
```

## 5. 技術スタック

| レイヤー | 技術 | 理由 |
|---|---|---|
| 回路（circuit） | circom | Merkle 包含証明のサンプルが豊富。触った経験がある |
| proof 生成・検証 | Rust + `ark-circom`（`arkworks` ベース） | Rust で学習を継続しつつ、circom の資産を活かせる |
| ハッシュ関数 | Poseidon（回路内）／将来 on-chain 連携では keccak256 も検討 | 回路内演算に適したハッシュ |
| on-chain | Solidity（`snarkjs` で生成した Groth16 Verifier + 自作 Registry） | Ethereum ハッカソンのため on-chain 要素を用意 |
| デモ UI | 未確定（最小限の Web UI、Rust の CLI でも可） | 一人開発・Rust 中心のため実装コストが低いものを選ぶ |

## 6. 回路仕様

### 6.1 MVP版（まずこれだけ作る）

**目的**: secret が家族の Merkle Tree に含まれることのみを証明する（challenge との紐付けはまだ含めない）

- Private input: `secret`, `pathElements[levels]`, `pathIndices[levels]`
- Public input: `root`
- 制約: `leaf = Poseidon(secret)` を計算し、Merkle path を辿って `root` と一致することを検証

### 6.2 拡張版：RLN（MVP が動いてから追加する）

**目的**: MVP の検証に加え、(1) 証明を特定の challenge・epoch に紐付けてリプレイを防ぎ、
(2) 同一 secret が 1 epoch 内で limit 回を超えて使われたら、その proof 群から secret を
数学的に復元して失効できるようにする。

**パラメータ（デモ設定）**
- `epoch = floor(unixtime / 3600)`（1 時間窓）。公開入力。
- `limit = 1`（1 epoch あたり同一 secret の証明は 1 回まで）。よって Shamir 多項式は 1 次
  `y = a0 + a1 · x` で足りる。

**入出力**

| 種別 | 変数 |
|---|---|
| Private input | `secret`, `pathElements[levels]`, `pathIndices[levels]` |
| Public input | `root`, `epoch`, `challenge` |
| Public output | `y`, `nullifier` |

**回路内で計算・制約するもの**
- `leaf = Poseidon(secret)` を計算し、Merkle path を辿って `root` と一致（MVP と同じ）
- `a1 === Poseidon(secret, epoch)` … 多項式の 1 次係数
- `x === Poseidon(challenge)` … 評価点。検証側の challenge に紐づき、prover は選べない
- `y === secret + a1 · x` … Shamir シェア（`a0 = secret`）。体は BN254 スカラー体
- `nullifier === Poseidon(a1)` … 同 epoch・同 secret で一致する識別子

**検証側 / on-chain レジストリの挙動**
- Groth16 を検証したうえで `(epoch, nullifier) -> (x, y)` を記録する
- 同 `nullifier` が未記録 → 受理して保存
- 同 `nullifier`・同 `x` → 単なるリプレイとして拒否（新情報なし）
- 同 `nullifier`・別 `x` → 2 点 `(x1,y1),(x2,y2)` から
  `a1 = (y2 - y1) / (x2 - x1)`、`secret = y1 - a1 · x1`（いずれも体の演算）で secret を復元。
  当該 leaf を失効リストに入れ、イベントを発行

**攻撃者の非対称性**（本方式の要）
盗んだ secret を持つ攻撃者は 1 epoch 内で、
- 証明しない → 2 人目の親族の確認に通らず、なりすまし失敗
- 2 回目を証明する → challenge が異なり `x` が異なる → secret 復元 → 失効

どちらでも攻撃者が損をする。`x` は検証側 challenge 由来なので、攻撃者が衝突を避けて
復元を回避することはできない。

### 6.3 レベル数（Merkle Tree の深さ）

デモ用途では `levels = 4〜6`（家族16〜64人規模で十分）から始め、動作確認後に本番想定の
`levels = 20` に拡張する。levels を小さくすることで、学習中の回路デバッグやローカルでの
proving 時間を短縮できる。

## 7. 実装ロードマップ（学習しながら進める前提）

Rust 4 か月目であることを踏まえ、ZK 特有の概念（制約・witness など）に慣れる時間を
見込んだ段階分けにする。各ステップは前のステップが動くことを確認してから次に進む。

### Step 0: ZK なしで骨格を作る
- Rust で Merkle Tree 構築（ハッシュ関数は何でもよい、まずは標準ライブラリの `sha2` 等）
- secret のハッシュが Tree に含まれるかを、ただの Rust 関数（ループと比較）で判定する
- **ゴール**: ZK を使わない「ロジックの正しさ」を先に固める

### Step 1: circom のサンプルを写経して動かす
- circom の公式・コミュニティにある Merkle 包含証明のサンプル回路をそのままコンパイル・実行
- `circom` CLI と `snarkjs`（または `ark-circom`）で proof 生成・検証を一通り体験する
- **ゴール**: 自分で回路を書く前に、動く一例を手元で再現する

### Step 2: MVP 回路（6.1）を自分のユースケースに合わせて実装
- Step 1 のサンプルを、Step 0 で決めた secret / leaf の構造に合わせて改造
- **ゴール**: `secret` を渡すと Merkle 包含証明の proof が生成・検証できる状態

### Step 3: Rust 側の proof 生成・検証コードを整備
- `ark-circom` を使い、circom がコンパイルした r1cs / wasm を Rust から読み込む
- witness 生成 → proof 生成 → 検証、を Rust の関数として実装
- **ゴール**: Rust の CLI から一連の流れを実行できる状態

### Step 4: RLN（6.2）を追加
- 回路に `epoch` / `challenge` を追加し、`a1 = Poseidon(secret, epoch)`、`x = Poseidon(challenge)`、
  `y = secret + a1 · x`、`nullifier = Poseidon(a1)` を制約として実装
- Rust 側に 2 点からの復元（体の割り算、`ark-ff` の `Fr`）を実装
- **ゴール**:
  - 同じ secret でも challenge / epoch が変われば proof が変わることを確認
  - 同 epoch・別 challenge の 2 証明から secret を復元できることをテストで確認
  - 1 epoch に 1 証明だけなら secret が漏れないことを確認

### Step 5: 最小限のデモ UI
- 証明する側／検証する側の 2 画面を用意（Web か CLI かは実装コストで判断）
- 「攻撃者が同じ epoch に 2 回証明を試みる → secret が露出 → 失効される」シーンも見せる
- **ゴール**: 「なりすまし確認」と「使い回し検知」の流れを人に見せられる状態にする

### Step 6: on-chain 連携
- `snarkjs zkey export solidityverifier` で Verifier.sol を生成
- FamilyRegistry コントラクトで root 登録・proof 検証を実装（テストネットへのデプロイ）
- `(epoch, nullifier) -> (x, y)` の記録、衝突時の 2 点補間による secret 復元、失効リスト管理を実装
- **ゴール**: on-chain 要素（登録・検証・使い回し検知・失効）を含めた提出物として完成させる

### 時間が余った場合の拡張候補
- モバイル UX（音声変換、SMS 連携）の設計をピッチ資料上で提示（実装はしない）
- SOS／デュレス機能の設計をピッチ資料上で提示

## 8. 脅威モデルと既知の限界

- secret が端末の紛失・盗難等で漏洩した場合、RLN により「1 epoch に limit 回を超えて使われた
  secret は数学的に復元・失効できる」まで緩和される。ただし limit 以内の使用（デモ設定では
  1 時間に 1 回まで）では露出せず、その範囲でのなりすましは依然として成立しうる（残存リスク）。
  epoch を短くする・limit を上げる等はトレードオフで、根本解決には Secure Enclave 等との連携が
  必要（本提出のスコープ外）
- RLN の副作用として、正規メンバーが同一 epoch 内に本当に limit 回を超えて証明する必要がある
  場合（別々の親族から短時間に複数回）も secret が露出する。epoch 長・limit の設定でしか
  調整できず、UX との綱引きになる
- 検証側（受け手）の端末・アプリ自体が改ざんされていた場合、検証結果の表示自体が信用できなく
  なる（UI レイヤーの改ざん耐性は本提出のスコープ外）
- 本設計は「秘密を言わずに済む」ことが前提だが、詐欺師が「アプリが使えないので確認できない」
  と別の口実を作る社会的攻撃には技術だけでは対応できず、啓発とのセットが必要

## 9. 審査基準へのマッピング

| 基準 | 対応内容 |
|---|---|
| Technicality | circom による Merkle 包含証明回路に RLN（回路内 Shamir シェア + nullifier）を統合、Rust（arkworks / `ark-ff` の体演算）による proof 生成・検証・2 点補間での secret 復元、on-chain の Verifier + nullifier 記録・復元・失効までを一気通貫で実装 |
| Originality | 「本人確認のための秘密が、確認のたびに漏洩リスクに晒される」という既存対策の矛盾を ZK の秘匿性で解決。さらに RLN により「盗まれた秘密は使い回すほど自壊する」という攻撃者非対称性を持ち込む |
| Practicality | 秘密を中央サーバーに預けない設計に加え、端末紛失時に「使いすぎた秘密を復元して失効する」現実的な revocation 経路を用意。詐欺対策アプリ自体が新たな漏洩リスクにならない |
| Usability | （MVP では簡易デモに留めるが）将来的には人間の操作を最小化する UX 案（着信連動等）を提示 |
| WOW Factor | 「秘密を一切言わせずに家族であることを証明する」直感に反する体験に加え、「同じ秘密で 2 回証明すると鍵が露出して無効化される」ライブデモ |

## 10. 未確定事項（今後決めること）

- デモ UI を Web にするか Rust ネイティブ（`egui` 等）にするか
- on-chain のネットワーク（テストネット選定）
- ピッチでモバイル UX 案（音声変換／SMS）をどこまで詳しく見せるか
- RLN の epoch のソース：クライアント時刻か、on-chain の `block.timestamp` か（検証側と証明側で
  epoch がズレると誤って失効/受理される可能性があるため、丸め幅と許容ズレも要検討）
- RLN の回路内ハッシュを Poseidon で通すか、パフォーマンス次第で Poseidon2 を検討するか
