# FamilyProof 仕様書

ETHGlobal Tokyo 2026 提出プロジェクト
「秘密を一度も明かさずに、家族であることを証明する」オレオレ詐欺対策

ステータス（2026-09-21 更新）: Step 0〜6（ZK Merkle 証明・RLN・on-chain Registry・通知インフラ・
匿名統計）は完了し World Chain Sepolia に実デプロイ済み。Step 7（Trust Circle / Family Constitution
拡張、§11）は設計完了・実装中。最新の進捗は [HANDOFF.md](HANDOFF.md) を参照。

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
- オンボーディング時に World ID（IDKit）で人間性証明を要求し、詐欺師が偽メンバーを Merkle Tree に
  登録できないようにする
- RLN（Rate-Limiting Nullifier）を回路に組み込み、challenge 紐付け・リプレイ防止に加え、
  同一 secret の epoch あたり使用回数が limit を超えたら secret を復元・失効できるようにする
- なりすまし確認フローを Web デモ（2 画面：証明する側 / 検証する側）でシミュレーション
- 最小限の Solidity コントラクトで root 登録・proof 検証・RLN の nullifier/シェア記録と
  復元・失効を on-chain 実装。World Chain（Sepolia）にデプロイ
- 家族を ENS 名（`<family>.family.eth` 相当）で識別し、Merkle root を text record に格納

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
| 1inch / Uniswap Foundation のスポンサー統合 | スワップ・AMM の要素が本プロジェクトに無く、こじつけ統合になる（審査で減点対象） |
| Sui へのデプロイ | `sui::groth16` によるネイティブ Groth16 検証は魅力だが、on-chain 部分の Move 書き直しがソロ開発ではスコープリスク。EVM に集中する |

## 4. アーキテクチャ

```
[オンボーディング(一度だけ)]
  各メンバー: 端末内で secret と salt を乱数生成（外部送信しない）
  leaf = Poseidon([0, secret, salt])  // 先頭 0 はドメインタグ
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
| on-chain ネットワーク | World Chain（Sepolia テストネット） | World ID の nullifier モデルが本設計の RLN と同じ原始関数。スポンサー賞の必然性がある |
| on-chain | Solidity（`snarkjs` で生成した Groth16 Verifier + 自作 Registry） | Ethereum ハッカソンのため on-chain 要素を用意 |
| Sybil 対策（オンボーディング） | World ID（IDKit で人間性証明 → 家族登録時のゲート） | 詐欺師が偽メンバーを Merkle Tree に登録するのを防ぐ。RLN と同系統の技術で一貫性がある |
| 家族識別子 | ENS（`<family>.family.eth` 相当。Merkle root を text record に格納） | hex root より高齢者に覚えやすく、自作レジストリの代替／併用になる |
| デモ UI | 未確定（最小限の Web UI、Rust の CLI でも可） | 一人開発・Rust 中心のため実装コストが低いものを選ぶ |

## 6. 回路仕様

### 6.1 MVP版（まずこれだけ作る）

**目的**: secret が家族の Merkle Tree に含まれることのみを証明する（challenge との紐付けはまだ含めない）

- Private input: `secret`, `salt`, `pathElements[levels]`, `pathIndices[levels]`
- Public input: `root`
- 制約: `leaf = Poseidon([0, secret, salt])` を計算し、Merkle path を辿って `root` と一致することを検証

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
| Private input | `secret`, `salt`, `pathElements[levels]`, `pathIndices[levels]` |
| Public input | `root`, `epoch`, `challenge` |
| Public output | `y`, `nullifier` |

**回路内で計算・制約するもの**
- `leaf = Poseidon([0, secret, salt])` を計算し、Merkle path を辿って `root` と一致（MVP と同じ、先頭 0 はドメインタグ）
- `a1 === Poseidon(secret, epoch)` … 多項式の 1 次係数
- `x === Poseidon(challenge)` … 評価点。検証側の challenge に紐づき、prover は選べない
- `y === secret + a1 · x` … Shamir シェア（`a0 = secret`）。体は BN254 スカラー体
- `nullifier === Poseidon(a1)` … 同 epoch・同 secret で一致する識別子

**検証側 / on-chain レジストリの挙動**
- **前提: `epoch` の鮮度チェックは回路の外（検証側）の責務**。回路自身は「今が実際に何時か」を
  知る手段を持たず、`epoch` は prover が自己申告する public input に過ぎない。検証側で
  `epoch == floor(block.timestamp / 3600)`（または許容誤差内）を確認してから受理しないと、
  (a) 古い証明のリプレイが「新しい証明」として通ってしまう、(b) prover が毎回違う `epoch` を
  名乗ることで rate limit（nullifier 突合）そのものを回避できてしまう、の2つが成立してしまう
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

### Step 5: 最小限のデモ UI（2026-09-21: 着手タイミングを残り2.5日〈イベント前〉に変更、Step 7 と入れ替え）

**UI方針（2026-09-21 決定）**: 3シーンをすべてブラウザに寄せない。Groth16証明の生成（シーン1・2）は
arkworksが必要でブラウザ実行（WASM化）は新規スコープが大きすぎるため見送り、**既存の Rust CLI
（`cargo run` / `src/bin/submit_demo.rs` / `cast send` 等）の出力をターミナルでそのまま実演**する。
**シーン3のみ** `stats.html` と同じパターン（ビルドツールなし・単体HTML + vanilla JS、バックエンドなし）
で最小限のページを作る。新規サーフェスをHTML1ファイル分に抑える（World ID実SDKは使わずモック化する
決定の経緯は下記シーン3詳細参照）。

- **シーン1（家族なりすまし確認）**: ターミナルで実演。証明する側／検証する側は別ターミナル/別PCの
  `cargo run` 出力の対比で見せる（新規UI不要）
- **シーン2（RLN使い回し検知）**: 「攻撃者が同じ epoch に 2 回証明を試みる → secret が露出 → 失効される」
  流れも `submit_demo.rs`（challenge違いで2回送信）+ `notifier.rs`（`PotentialLeak`検知・secret復元・
  メール通知）の既存出力をターミナルで実演（新規UI不要、Step 6 で実証済みのフローを流用するだけ）
- **シーン3（World ID live-challenge、AI音声クローン対策）詳細設計（2026-09-21 決定、同日中に方針を
  「実SDK統合」→「モック」に最終確定）**: 唯一の新規UI。単体HTMLページ（`stats.html` と同パターン、
  ビルドツールなし・バックエンドなし）1枚に「証明する側」「検証する側」の2パネルを並べる。

  - **World ID実SDK（`@worldcoin/idkit`）は使わない（2026-09-21 最終決定）**: 現行の公式フローを
    docs.world.org で確認したところ、`RP_SIGNING_KEY` によるリクエスト署名を行う小さなバックエンドが
    実質必須（署名ロジックは公式SDK `@worldcoin/idkit-core/signing` 以外に仕様が無く、Rustでの自前実装は
    プロトコル不一致のリスクが高い）と判明。2.5日でシーン1・2の実演台本作成と並行してこれを検証・実装
    しきる確信が持てないため、**シーン3は自前JSでの概念実演（モック）に留める**。本物のIDKit統合設計
    自体は上記の通り確認済みなので、ピッチ/READMEには「本番実装はこの設計（RP署名＋v4 verify API）、
    デモは時間制約でモック」と正直に明記する。**この判断により World ID/Worldcoinのパートナー賞対象からは
    外れる可能性が高いが、コアの ZK+RLN（Step 0〜6）は実チェーン実装のままなので影響しない**
  - **モックの設計**: 本物の `nullifier_hash`（同一 `app_id`+`action` なら同一人物は毎回同じ値になる、
    World IDのSybil耐性つき匿名ID）の代わりに、UI上で「証明する側」の人物を
    プルダウン等で選ばせ（例: 「正規メンバー（娘）」「攻撃者」）、**人物ごとに固定した疑似
    `nullifier_hash`（JS内の定数、例えば人物名の簡単なハッシュ）を割り当てる**。「同じ人物なら毎回
    同じID、別人なら別ID」という World ID の本質的性質だけを再現する
  - **証明する側パネル**: 人物選択（プルダウン）＋「電話で聞いた合言葉」入力欄 → 「証明する」ボタン。
    `signal = <入力された合言葉>`（文字列そのまま比較で良い、field要素へのハッシュ化などIDKit内部処理は
    模倣しない）。ボタン押下で選択した人物の疑似`nullifier_hash`と`signal`を画面に表示
  - **検証する側パネル**: 「自分が実際に言った合言葉」入力欄（基準値）＋ 事前登録済みの「正規メンバーの
    疑似nullifier_hash」（固定値） → 「検証する」ボタン。判定は2条件: ① `signal` が基準値と一致するか、
    ② 疑似`nullifier_hash` が正規メンバーの登録値と一致するか。**①のみ真・②が偽 = 「合言葉は合っているが
    本人ではない」**という、AI音声クローン脅威そのものを可視化する分岐になる
  - **正規メンバーのデモ実行**: 「正規メンバー」を選択して実演 → ①②とも真 → 受理
  - **攻撃者（AIクローン音声）のデモ実行**: クローン音声が正しい合言葉を再生 → 「攻撃者」を選択し同じ
    合言葉を入力 → ①は真だが②が偽（別人物の疑似ID）→ 拒否。「secretを知っているだけでは通らない」を
    ライブで実演できる
  - **on-chain Registry状態との突き合わせは行わない（スコープ外に確定）**: モック化に伴い、判定ロジックが
    完全にクライアント内で完結するため不要と判断
- **AI音声クローンによる脅威デモ（2026-09-19 採用）**: 「正しい合言葉さえ言えば通ってしまう」ことの
  危険性を、実際にクローン音声で見せる。同意を得たチームメンバー本人の声でイベント前に音声クローン
  （TTS）を作成しておき（ライブ生成はデモ事故リスクが高いため事前生成）、デモ本番ではその音声で
  合言葉を再生 → World ID の live-challenge で弾かれる／検知される、という流れをライブで見せる。
  「secret を知っているだけでは通らない」を可視化する目的で、RLN 単体のデモ（漏洩 secret の使い回し
  検知）とは別シーンとして構成する
- **ゴール**: 「なりすまし確認」「使い回し検知」「AI音声クローンでの突破試行の検知」の3つの流れを
  人に見せられる状態にする

### Step 6: on-chain 連携
- `snarkjs zkey export solidityverifier` で Verifier.sol を生成
- FamilyRegistry コントラクトで root 登録・proof 検証を実装（テストネットへのデプロイ）
- `(epoch, nullifier) -> (x, y)` の記録、衝突時の 2 点補間による secret 復元、失効リスト管理を実装
- **匿名統計の公開（2026-09-20 採用）**: `notifier.rs`（`RootUpdated`/`PotentialLeak` イベント監視、
  §7 Step 6 TODO 参照）を拡張し、家族root・address 等の個人/家族を特定できる情報を一切含めない
  「日次の検知件数」だけを集計・公開する。表面化しづらいオレオレ詐欺の試行実態を、被害者・家族を
  特定せずに可視化することが目的。集計単位は日次カウントに留め、個別イベントの時刻・アドレスは
  公開側には出さない（再登場パターンからの家族特定を防ぐ）
- **ゴール**: on-chain 要素（登録・検証・使い回し検知・失効・匿名統計公開）を含めた提出物として完成させる

### Step 7: Trust Circle / Family Constitution 拡張（2026-09-21 採用、**2026-09-21 夜から着手**。Step 5 完了に伴い本番待ちを撤回）

「家族であることを証明する」から「Trust Circle が特定の Action を承認したことを証明する」への一般化。詳細な技術設計・データ構造・既知の制約は §11 を参照。ここではロードマップ上の位置づけのみ示す。

- **新規暗号要素は追加しない**方針で設計する。既存の Merkle+ZK+RLN 回路（circom側は無変更）をそのまま使い、`challenge` を「World ID live-challenge の spoken code」ではなく「`actionId`（Action ごとに off-chain で一度だけ算出する、`Fr` 体に還元済みの識別子）」として使う、という Step 6 で既に確立した「challenge の意味を用途ごとに使い分ける」パターンの第3の応用として位置づける
- 新規追加が必要なもの（すべて Rust/Solidity、回路・zkey の作り直し不要）:
  - `contracts/src/FamilyConstitution.sol`（Action 提案・ZK証明つき承認カウント・閾値到達で `ActionAuthorized` イベント発行。§11.4 参照）
  - Rust 側は既存の証明生成フロー（`proof::build_circuit_with_inputs` 等）を `challenge=actionId`（§11.3/§11.5参照）で呼ぶだけで新規バイナリが書ける
- **ゴール（デモシナリオ）**: 「AI Family Agent が高額 Action（例: 300万円送金）を提案 → Trust Circle メンバー N 人がそれぞれ ZK 証明で承認 → 閾値到達で `ActionAuthorized` が発行される」を World Chain Sepolia 上で実演する。低額 Action は AI 単独で実行できる（0 承認）ことも対比で見せる
- **時間が足りない場合の優先順位**（イベント本番3日間で間に合わなかった場合の縮退ライン、優先度高い順）:
  1. `FamilyConstitution.sol` の Solidity 実装 + Foundry ユニットテストのみ（on-chain ロジックの正しさを証明。UI・デモ動画なし）
  2. 上記 + 実チェーン（World Chain Sepolia）への実際の証明提出（`submit_demo.rs` 相当の拡張で実証）
  3. デモ UI・動画への組み込み（Step 5 の UI に Step 7 のシナリオを追加する形）
  4. それでも間に合わなければ §11 の設計のみをピッチ資料の "Prove trust, not identity" ビジョンとして提示するに留める（従来通りの縮退）

### 時間が余った場合の拡張候補
- 検知された詐欺手口（通話内容・スクリプト等）を家族からの任意報告として集め、AIで要約・統計化して
  公開する構想をピッチ資料上で提示（実装はしない）。現状の暗号設計は通話内容を一切扱わないため、
  実現には報告フォーム等の新規データ収集経路がゼロから必要になり、本提出のスコープ外
- モバイル UX（音声変換、SMS 連携）の設計をピッチ資料上で提示（実装はしない）
- SOS／デュレス機能の設計をピッチ資料上で提示
- **World ID の一意性を使った root 更新のマルチパーティ承認（2026-09-20 検討・実装見送り）**: 現状 `updateRoot` は owner 単独の秘密鍵で実行できる単一障害点（管理者端末が乗っ取られると木を書き換えられる）。通常のマルチシグは攻撃者が偽ウォレットを量産できるため防御にならないが、World ID は「1人1つしか発行できない」Sybil耐性を持つため、「異なる家族メンバーN人がそれぞれ自分のWorld IDで承認しないとroot更新できない」という仕組みが原理上成立する。ただし既存の owner 単独モデルを丸ごと作り直す規模のスコープになるため今回は実装せず、ピッチの将来構想として提示するに留める
- ~~Trust Circle / Family Constitution 構想（2026-09-21 検討・実装見送り）~~ → **2026-09-21 に方針転換、正式にスコープ採用。着手タイミングは同日中に2回変わり、最終的にStep5完了直後（本番待ちせず）に確定。詳細設計は Step 7（本節下）と §11。** 当初「新しい回路・コントラクト設計が必要な別プロジェクト規模」として見送ったが、既存の Merkle+ZK+RLN 回路を改変せずに `challenge` の意味を使い分けるだけで実現できる設計に気づいたため、現実的なスコープに収まると判断した（詳細は §11.3 の再利用設計）

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
- 量子耐性は無い。secret を隠す Poseidon ハッシュ自体は Grover のアルゴリズムによる二次的な
  高速化しか受けず実用上の影響は小さいが、証明方式に使っている Groth16（BN254 楕円曲線の
  ペアリング）は Shor のアルゴリズムで効率的に解ける離散対数問題に安全性が依存しており、
  十分な量子コンピュータがあれば理論上は偽の証明を構成できる。量子耐性のある証明系（STARK・
  格子暗号ベース等）への移行はツールチェーンごと別物になるため本提出のスコープ外

## 9. 審査基準へのマッピング

| 基準 | 対応内容 |
|---|---|
| Technicality | circom による Merkle 包含証明回路に RLN（回路内 Shamir シェア + nullifier）を統合、Rust（arkworks / `ark-ff` の体演算）による proof 生成・検証・2 点補間での secret 復元、on-chain の Verifier + nullifier 記録・復元・失効までを一気通貫で実装 |
| Originality | 「本人確認のための秘密が、確認のたびに漏洩リスクに晒される」という既存対策の矛盾を ZK の秘匿性で解決。さらに RLN により「盗まれた秘密は使い回すほど自壊する」という攻撃者非対称性を持ち込む |
| Practicality | 秘密を中央サーバーに預けない設計に加え、端末紛失時に「使いすぎた秘密を復元して失効する」現実的な revocation 経路を用意。詐欺対策アプリ自体が新たな漏洩リスクにならない |
| Usability | （MVP では簡易デモに留めるが）将来的には人間の操作を最小化する UX 案（着信連動等）を提示 |
| WOW Factor | 「秘密を一切言わせずに家族であることを証明する」直感に反する体験に加え、「同じ秘密で 2 回証明すると鍵が露出して無効化される」ライブデモ。さらに「AIクローン音声が正しい合言葉を言っても、live-challenge で弾かれる」デモで、合言葉方式だけでは AI 音声詐欺を防げないことを直感的に見せる |

## 10. 未確定事項（今後決めること）

- ~~デモ UI を Web にするか Rust ネイティブ（`egui` 等）にするか~~ → **2026-09-21 決定**: シーン1（家族なりすまし確認）・シーン2（RLN使い回し検知）は新規UIを作らず、既存の Rust CLI（`cargo run` / `cast send` 等）の出力をターミナルでそのまま実演する。シーン3（World ID live-challenge）のみ `stats.html` と同じパターン（ビルドツールなしの単体HTML + vanilla JS）で最小限のページを作る。理由・詳細は §7 Step 5 参照
- ピッチでモバイル UX 案（音声変換／SMS）をどこまで詳しく見せるか
- RLN の epoch のソース：クライアント時刻か、on-chain の `block.timestamp` か（検証側と証明側で
  epoch がズレると誤って失効/受理される可能性があるため、丸め幅と許容ズレも要検討）
- RLN の回路内ハッシュを Poseidon で通すか、パフォーマンス次第で Poseidon2 を検討するか

## 11. Trust Circle / Family Constitution 拡張仕様（2026-09-21 採用）

Step 7（§7）の詳細設計。**2026-09-21 夜、Step 5 が事実上完了したため本番待ちをやめて着手（詳細はHANDOFF参照）**。間に合わなければ続きはイベント本番（9/25〜27）に持ち越す。

### 11.1 背景・ビジョン（ピッチの軸にもなる要約）

現行 FamilyProof は「本人（血縁の家族）であることを証明する」設計だが、独居高齢者の増加を踏まえると
「家族がいない・少ない人は誰に守ってもらうのか」という問いが残る。そこで証明対象を一段抽象化する。

- **Before**: 「この人は家族ですか？」を証明する（Identity Proof）
- **After**: 「本人が選んだ信頼関係（血縁に限らない Trust Circle）が、この Action を承認したこと」を
  証明する（Action Authorization）

World ID は「スポンサー統合のための後付け機能」ではなく、**Trust Circle を構成する各メンバーが
一意な人間であること（Sybil耐性）を担保する本質的な部品**として位置づけ直す。さらに Trust Circle には
人間だけでなく「本人から権限を委譲された AI Family Agent」も参加しうるが、AI は対等なメンバーではなく
**代理人**（低リスク Action のみ単独実行可、高リスク Action は事前に取り決めた人数の人間承認 =
Family Constitution が必須）という権限モデルを採る。オレオレ詐欺対策自体も「声や合言葉を信じるか」
ではなく「高リスクな Action には Trust Circle の承認が要る」という同じ構造に一般化できる。
ピッチの締めは *"Prove trust, not identity."*

### 11.2 スコープの区切り方

| やること（2.5日で狙う） | やらないこと（明示的に外す） |
|---|---|
| 既存 Merkle+ZK+RLN 回路の**そのまま流用**（circom 無変更） | 新しい回路の設計・実装（例: N-of-M 閾値証明を1つの proof に畳み込む等） |
| `FamilyConstitution.sol`: Action 提案・ZK証明つき承認カウント・閾値実行 | LLM を実際に統合した AI Agent（今回は Rust の固定ロジックで「AI が提案する」ことをシミュレート） |
| ハードコードされた3階層の閾値（下記11.4） | Family Constitution 自体をメンバーが動的に編集する UI・ガバナンス機構 |
| World Chain Sepolia 上での実演（`cast send` / Foundry テスト） | AI Agent 自身に暗号的な ID を持たせる設計（§11.7 既知の限界で明記） |
| Trust Circle という呼称へのピッチ資料上の言い換え（「家族」→「信頼できる関係」も歓迎するナラティブ） | コードベース上の型・変数名の "Family" → "TrustCircle" 全面リネーム（今回のスコープでは不要な作業） |

### 11.3 再利用設計 — なぜ新しい回路がいらないか

既存回路の public input `challenge` は、Step 6 の設計時点ですでに「用途ごとに意味を使い分ける」箱として
扱われている。これまでの2つの用途に、3つ目を追加するだけで Action Authorization が成立する。

| 用途 | `challenge` の中身 | 既に実装済みか |
|---|---|---|
| ① RLN の基本用途（リプレイ防止） | 検証側が発行するランダム nonce | ✅ Step 4 |
| ② World ID live-challenge（AI音声クローン対策） | `hash(合言葉)`（World ID の `signal` に流用、circom 側は不使用） | 設計済み（HANDOFF参照）。**2026-09-21: デモ（Step 5 シーン3）は実SDKを使わずモック化決定。この用途②自体はcircomの`challenge`スロットに影響しないため、設計・③との関係は変わらない** |
| ③ **Action Authorization（本節、新規）** | `actionId`（後述、Action ごとに一度だけ off-chain で算出する `Fr` 体の元） | 未実装、本節で設計 |

**⚠️ 型の落とし穴（設計時に潰す）**: 回路の `challenge` public input は BN254 スカラー体の元＝`r`
（`2^254` 弱）未満の値。`keccak256` の出力（256bit フル）をそのまま `challenge`/コントラクトの
mapping key に使うと、`r` 以上の値になったときに Rust 側の `Fr::from_le_bytes_mod_order`（mod 還元）と
Solidity 側の生の `uint256` 比較が食い違いうる。対策は**「還元後の値を Action の唯一の識別子にする」**
こと（後述 `actionId`）— on-chain・off-chain 双方でハッシュを2回計算し直さない。

①と③は**同じ circom 回路・同じ challenge スロットを共有する**ため、コンパイル済みの `main_final.zkey` /
`verifier.sol` をそのまま使い回せる。②とは異なる経路（②は on-chain 検証を経由しない off-chain フロー、
③は on-chain の `FamilyConstitution.sol` で検証する）なので、①③の verifier 検証パスは Step 6 の
`FamilyRegistry.verifyMembership()` とほぼ同じロジックの再利用になる。

**RLN との相互作用（重要な既知の制約）**: 現行 RLN は `epoch = 1時間 / limit = 1` に固定されている
（§6.3）。これは「同一 secret が同一 epoch 内に異なる `challenge` で2回証明すると自己暴露する」設計
そのものなので、**同一メンバーが1時間以内に異なる2つの Action を承認すると、意図せず自分の secret を
露出させてしまう**。デモでは1メンバーにつき1 Action/デモシーン内で収める運用で回避するが、実運用では
Action Authorization 用に epoch・limit を別立てにする（RLN のインスタンスを用途ごとに分ける）等の設計変更が
必要になる。本番実装はスコープ外、§11.7 に既知の限界として明記する。

### 11.4 `FamilyConstitution.sol` 設計（関数シグネチャ・データ構造のみ、実装はユーザーが書く）

```solidity
struct ActionState {
    uint256 tier;              // 0=AI単独可, 1=人間1人, 2=人間2人, 3=Constitution変更(人間3人)
    uint256 requiredApprovals; // tier から決まる閾値
    uint256 approvalCount;
    bool executed;
    mapping(uint256 => bool) usedNullifiers; // このactionIdに対して既に使われたnullifier（二重承認防止）
}

mapping(uint256 => ActionState) public actions; // actionId => state
// actionId は bytes32 のハッシュではなく、Fr（BN254スカラー体）に還元済みのuint256。
// Rust側で「keccak256(action description) を Fr::from_le_bytes_mod_order で還元した値」として
// 一度だけ算出し、on-chain にはこの値だけを渡す（Solidity側での再ハッシュ・再還元は行わない）。

event ActionProposed(uint256 indexed actionId, uint256 tier, uint256 requiredApprovals);
event ActionApproved(uint256 indexed actionId, uint256 approvalCount, uint256 requiredApprovals);
event ActionAuthorized(uint256 indexed actionId); // 閾値到達時に1回だけ発行

function proposeAction(uint256 actionId, uint256 tier) external; // 呼べるのは agent アドレスのみ（onlyAgent）
function approveAction(
    uint256 actionId,
    uint256[2] calldata pA, uint256[2][2] calldata pB, uint256[2] calldata pC,
    uint256[5] calldata pubSignals // [root, y, nullifier, epoch, challenge] — FamilyRegistryと同一順序
) external; // 検証ロジックはFamilyRegistry.verifyMembership()とほぼ同じ（root/epoch鮮度/verifier呼び出し）
            // + pubSignals[4] == actionId を require（"challenge が この Action 用ではない"を弾く。
            //   Poseidon等の再ハッシュはしない — challenge 自体が actionId そのもの）
            // + usedNullifiers[nullifier] が false であることを確認してから true にする（同一メンバーの二重承認防止）
```

- **`tier` → `requiredApprovals` のデモ用固定表**（実運用は Family Constitution として可変にすべきだが、
  今回は定数でよい）:

  | tier | 意味 | 必要承認人数 |
  |---|---|---|
  | 0 | 日常対応・少額（例: 予定管理、1万円未満の支払い） | 0（AI Agent が単独実行、`proposeAction` と同時に `executed=true`） |
  | 1 | 中額（例: 10万円未満の送金） | 1 |
  | 2 | 高額（例: 100万円以上の送金） | 2 |
  | 3 | Trust Circle 構成の変更（新規メンバー追加、AI Agentの権限変更） | 3（デモでは省略可） |

- **AI Agent の権限モデル**: AI Agent は Trust Circle の Merkle Tree に leaf を持たない（＝ZK証明を
  生成できない）。`proposeAction` を呼べる特別な EOA（`onlyAgent` 修飾子）として実装するだけで、
  「AI は Action を提案できるが、tier 1 以上は人間の ZK 証明なしには実行されない」が
  **コントラクトレベルで強制される**（AI が `approveAction` を有効な proof 付きで呼ぶことは、secret を
  持たない以上できない——ZK の健全性がそのままここでも権限モデルの土台になる）。

### 11.5 Rust 側設計（新規バイナリ、既存コードの再利用のみ）

- `src/bin/propose_action.rs`: Action の説明文字列（例: `"send 3000000 JPY to xxx"`）を受け取り、
  `keccak256` → `Fr::from_le_bytes_mod_order(&hash_bytes)` で **`actionId` を一度だけ算出**する
  （これが以後 on-chain の Action 識別子・`challenge` 双方を兼ねる、単一の正）。
  `proposeAction(actionId, tier)` の calldata を作る、または直接 `cast send` で呼ぶ。
  承認側は既存の `proof::build_circuit_with_inputs` にこの `actionId` の `Fr` 値をそのまま
  `challenge` として渡すだけで proof 生成できる（回路側の `x = Poseidon(challenge)` 計算は無変更）。
  `to_solidity_calldata` も Step 6 の実装をそのまま呼べる。
- 複数メンバーの承認をシミュレートするには、`submit_demo.rs` と同様に `cast send` で
  `FamilyConstitution.approveAction(...)` を人数分連続実行すればよい。

### 11.6 デモシナリオ（想定台本）

1. AI Family Agent が「300万円送金」Action を提案（tier=2）→ `ActionProposed` イベント
2. 攻撃者（secretを持たない）が `approveAction` を試みる → 有効な proof を作れず失敗（証明できないことを見せる）
3. Trust Circle メンバー1人目が ZK 証明で承認 → `ActionApproved(1/2)`
4. メンバー2人目が承認 → `ActionApproved(2/2)` → `ActionAuthorized` 発行
5. 対比として、AI Agent が「予定リマインド送信」（tier=0）を提案 → 即 `executed=true`（人間の承認不要）

### 11.7 既知の限界（§8 に準ずる追記）

- **AI Agent 自身は暗号的な ID を持たない**。今回のデモでは「`proposeAction` を呼べる特定の EOA」という
  アクセス制御レベルの権限に留める。本来は AI Agent 自身も Sybil耐性のある形で識別・失効できるべきだが
  （侵害された AI Agent を Trust Circle から切り離す等）、鍵管理・attestation の設計が別途必要になるため
  2.5日のスコープでは扱わない
  - **将来構想（2026-09-21、MPCによる鍵管理）**: 今のAI Agentは「1つのEOA＝1つの秘密鍵」なので、その鍵が
    漏れれば即座にAgentの権限が乗っ取られる。本番実装するなら、AI Agent自身の署名鍵を単一の場所に置かず、
    MPC（閾値署名）で複数の主体（複数サーバー・複数クラウド事業者など）に分散し、署名のたびに閾値分の
    シェアを集めて計算する構成にするのが定石（Fireblocks等の本番AIエージェント×ウォレット運用で実際に
    使われている手法）。これは Trust Circle メンバー側の非同期なZK承認フロー（§11.4）には一切影響しない
    ——あくまで「AI自身の鍵の守り方」というレイヤーの話であり、§11.8の委任権限（権限の中身をZKで制約）
    と組み合わせると、「何ができるか（§11.8）」と「その鍵自体がどう守られているか（本項）」の両方を
    カバーできる。今回は実装しない
  - **検討したが不採用: Trust Circle側の承認集約をMPC（閾値署名）にする案**。t-of-nのメンバーが共同で
    1つの閾値署名を作りコントラクトはそれ1つを検証する方式も検討したが、閾値署名は参加者が同時に
    オンラインで協調する必要があり、今の「隣人Aは今日中に、隣人Bは気づいたときに、それぞれ非同期で
    承認する」というUXを壊してしまう。on-chain検証コストは下がる（Groth16検証N回→署名検証1回）が、
    このプロジェクトの利用文脈（高齢者を含む非同期なTrust Circle）には合わないと判断し不採用
- **RLN の epoch/limit 共有問題**（§11.3 既述）: Action Authorization に既存の RLN インスタンスをそのまま
  使うと、1時間以内に複数 Action を承認した正規メンバーの secret が意図せず露出しうる。デモでは発生しない
  順序で進行するが、実運用では別インスタンス化が必要
- **Family Constitution 自体は可変ではない**（tier→閾値のマッピングはコントラクトの定数）。本来のビジョンで
  ある「Trust Circle が自分たちで閾値を決める」ガバナンス機構は将来課題として提示するに留める

### 11.8 将来構想: 委任権限そのものをZKで証明する（今回は実装しない、次の研究方向）

2026-09-21、AI Agent にも secret を持たせるべきかという議論から派生した検討。**結論: secret を持たせるのは
不採用**（下記「不採用の理由」）。代わりに、より技術的に深い方向性として以下を将来構想として記録する。

**現状の限界**: `proposeAction` を呼べる権限は、ZKではなく単なるアクセス制御リスト（`onlyAgent` で固定した
1つのEOA）で守られている。ZKが関与するのは承認（`approveAction`）側だけで、提案側には一切関与しない。

**不採用の理由（AIにTrust Circleのsecretを持たせる案）**: AIが Merkle Tree の leaf を持てば、AIの証明が
tier の必要承認人数の1票としてカウントされてしまい、閾値が実質1つ下がる（tier=2の送金が「AI1票＋人間1人の
共謀」で通る）。さらにLLMはプロンプトインジェクションで判断を乗っ取られうるため、「AIのsecret」は本質的に
「プロンプト経由で引き出されうる秘密」になる——これは本プロジェクトの出発点である「電話越しに秘密を言わ
される・盗聴される」というオレオレ詐欺の構図と同型の脆弱性を、AI側に持ち込むことになる。

**より筋が良い方向性: ZK-provable delegated capability（Actionごとに暗号的に制約される委任権限の証明）**。
AIに「Trust Circleのメンバーである」ことを証明させるのではなく、「Trust Circleから、特定の制約つきで権限を
委任されている」ことをActionごとに証明させる。

- **委任クレデンシャル**: Trust Circle 用の Merkle Tree とは別に、「委任」を表す小さな Merkle Tree を用意する。
  各 leaf は `Poseidon([tier上限, 金額上限, 有効epoch, nonce])` のような、委任の制約をコミットした値
- **委任の発行**: 新しい信頼の起点（trusted setup や中央管理者）を増やさないために、**委任の発行自体を
  既存の Action Authorization の仕組みで表現する**——「AIにこの制約で権限を委任する」こと自体を1つのAction
  （tier=3相当）として Trust Circle が承認し、閾値到達時に委任 leaf が委任用 Merkle Tree に追加され、
  その root が on-chain 更新される。新しい信頼モデルを持ち込まず、既存の閾値承認をそのまま「委任の発行」に
  転用できるのが利点
- **Actionごとの証明**: AIが `proposeAction` を呼ぶ際、単なるEOAチェックではなく、
  「委任 Merkle Tree に自分の委任 leaf が含まれる」＋「今回のAction（金額・tier）がその委任の制約を
  満たす」ことをZK証明として要求する。金額の上限チェックは、上限の値自体を明かさずに
  `action_amount ≤ 委任された上限` を示す**レンジ証明**（circomlib の `LessThan`/`GreaterEqThan` 等の
  comparator gadget で実装可能）、有効期限は既存の epoch鮮度チェックと同じパターンを流用できる
- **失効**: 委任 leaf も root rotation と同じ仕組み（Trust Circle が新しい委任へ差し替え、root 更新）で
  失効できる——侵害されたAIの委任を、Trust Circleの合意で即座に切り離せる

**スコープ評価**: Step 7（Family Constitution）が2.5日で収まったのは「既存回路の `challenge` スロットを
使い回すだけで新しい回路が不要だった」ためだが、この委任権限の仕組みは提案側にも新たにZKを持ち込むため、
**新しい回路（Merkle包含＋レンジ制約）・新しいzkey/vkey・新しいVerifier・新しいテスト一式が必要**になる。
Step 7とは規模が異なる、本当に新しい暗号設計であり、今回のイベント期間（9/25〜27）のスコープには含めない。
ピッチでは「次の研究方向」として提示するに留める。
