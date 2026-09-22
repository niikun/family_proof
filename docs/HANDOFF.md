# HANDOFF — 別PCへの引き継ぎ

最終更新: 2026-09-22夜（Step 7 ①〜④完了に加え、README/PITCH/提出文の整備まで完了。**明日(9/23)からMCP化に着手**。残りはMCP化・デモ動画・デモリハーサル） / ブランチ: `main` / remote: `git@github.com:niikun/family_proof.git`

> ✅ **Step 6 コア完了（Verifier/Registry/テスト/calldata変換/World Chain Sepoliaデプロイ済み）**。デプロイ済みアドレスは下記「Step 6」節参照（`FamilyRegistry` は `0xa9f1A920...` が正、`0xD06FcbB5...` は重複デプロイの旧アドレスで放置）。追加拡張A/B/Cも完了（下記「残り期間での追加拡張」節）。
> **🆕 Step 7（Trust Circle / Family Constitution 拡張）を正式採用し、2026-09-21 夜から着手済み**（設計は [SPEC.md §11](SPEC.md) 完了）。進捗は下記「いまどこ」節参照。Claude はコーチのみ、設計を書いただけでコードは書いていない — 実装は引き続きユーザーが行う。
> **⚠️ 重要: ETHGlobal Tokyo 2026 の日程・提出ルールが確定済み（下記参照）。この5連休の位置づけが変わったので必読。**
> **🆕 ピッチの再定義（Trust Circle）を確定し、`README.md`・`docs/PITCH.md`・3枚スライドを作成、`SPEC.md §11.8`（委任権限のZK証明、将来構想）を追記。詳細は下記「🆕 Step 7」節の追加項目、および `README.md`・`docs/PITCH.md` 本体を参照。**

## ⚠️ ETHGlobal Tokyo 2026 日程・提出ルール（2026-09-19 確認）

- **イベント本体は 9/25〜27**。提出締切 **9/27 9:00 JST**、遅延提出不可。審査は7分（デモ4分＋Q&A3分）、基準は technicality / originality / practicality / UX / "WOW factor"
- **Continuity Track での提出が必須**。"Classic From Scratch" はイベント開始後（9/25〜）に書いたコードのみが対象で、Step 0〜4（9/6〜9/19に実装済み）は対象外になってしまう → Continuity Track を選び、「イベント前からの既存部分」と「イベント中に新規に作った部分」を明確に書き分けて提出する
- **AIツール利用ポリシー**: AI支援は許可されるが、人間の実質的な貢献を示しつつ明記が必要。[[no-writing-code]]（Claude はコーチのみ、コードは全部ユーザーが書く）の運用がそのままこの要件を満たす — README/提出文に明記すること
- デモ動画（2〜4分、720p以上）は**ETHGlobal提出ルール上は任意だが、ユーザー判断で必須項目に格上げ（2026-09-21）**。倍速・電話撮影・テキストのみ+音楽・AIナレーションは禁止。台本（シーン1・2・3の実演台本、上部「🆕 Step 5」節）が録画の元になる。
  - ~~順序変更（2026-09-21）: 動画をStep7の③④より先に撮る~~ → **同日中に撤回、③④を先に進める方針に戻す**。動画撮影は③④の後（または一段落してから）

**改訂後の進め方（2026-09-19時点の推奨、2026-09-21 順序再変更）**: 今日から9/24（イベント前日）までを「Continuity Track の“既存部分”」の仕上げに使う。Step 6 のコアは完了済み。**2026-09-21時点の最新方針: 残り2.5日は Step 5（デモUI・World ID音声クローン対策連携）の完成に充てる。イベント本番（9/25〜27）は「期間中に新規に作った部分」として Step 7（Trust Circle/Family Constitution拡張）・統合・デモ動画・Continuity提出文の執筆に充てる。** これで「動くデモ（Step5）」を確実に確保しつつ、「イベント中に作った説得力のあるストーリー（Step7）」も狙う。

## いまどこ

ロードマップ（[SPEC.md](SPEC.md) §7）で **Step 0〜4 完了、Step 5 事実上完了、Step 6 完了、Step 7 完了**（①〜④すべて済み。`FamilyConstitution.sol`を本番World Chain Sepoliaにデプロイし、AI提案→2人承認→`ActionAuthorized`を実チェーン上で実証済み）。README/README.en/PITCH/PITCH.en/SUBMISSION.enの整備も完了（2026-09-22夜）。**残るのはMCP化（明日9/23〜着手）・デモ動画・シーン3カットの要否判断・`submit_demo.rs`の残バグ修正**。詳細は下記「🆕 README/PITCH/提出文の整備」節参照。

| Step | 状態 |
|---|---|
| 0 Rust Merkle 骨格（ZKなし） | ✅ `cargo test` |
| 1 circom サンプル写経・compile→prove→verify | ✅ WSL でも全パイプライン疎通 |
| 2 MVP 回路を自ユースケースへ | ✅ circom 側 done / Rust パディングを `EMPTY_HASH` 固定に（commit `9c52de4`） |
| 3 `ark-circom` で Rust から proof 生成・検証 | ✅ **完了**。CLI の Merkle root が回路の public root と一致することまで実証済み |
| 4 RLN（§6.2） | ✅ **完了**（2026-09-19）。回路実装・Rust配線・2点復元テストまで完走。詳細は下記「Step 4」節 |
| 5 デモ UI（World ID部分はモック） | ✅ **2026-09-21 夜、事実上完了**。台本確定・バグ2件修正・`cast send`/S3自動化・シーン3実装・オンチェーンroot表示まで全て動作確認済み。詳細は下記「🆕 Step 5」節 |
| 6 on-chain（+ World ID ゲート） | ✅ **完了**。Verifier/Registry/ユニットテスト/本物データでの統合テスト/Rust鍵統一/calldata変換/World Chain Sepoliaへのデプロイ/通知インフラ/匿名統計公開まで全て済み。詳細は下記「Step 6」節 |
| 7 Trust Circle / Family Constitution 拡張 | ✅ **①〜④すべて完了**（2026-09-22）。`FamilyConstitution.sol`は本番World Chain Sepolia `0xf7f344E9399638b69DF158877F1e77a39A5F3D73` にデプロイ済み、`propose_action.rs`/`approve_action.rs`でtier0/tier1/tier2（承認2人、異なるnullifier）まで実チェーン上で実証済み。ピッチ再定義（`README.md`/`docs/PITCH.md`/スライド3枚/`SPEC.md §11.8`）も完了。残タスクは下記「🆕 Step 7」節（デモ動画・英語版・提出文・stretchのMCP化） |

**スコープ方針（2026-09-21 夜 再更新・Step7前倒し）**: Must = Step 4 RLN（済） / Step 6 コア（済） / Step 5（済） → **今からStep 7 Trust Circle/Family Constitutionの実装に着手**、縮退ラインは[SPEC.md §7 Step 7](SPEC.md)参照 → 間に合わなければ続きはイベント本番（9/25〜27）に持ち越し。Cut候補 = ENS 名解決・levels=20拡張。

## 🆕 Step 5: デモUI（2026-09-21 UI方針決定、シーン3はモックに最終決定）

**UI方針**: シーン1（家族なりすまし確認）・シーン2（RLN使い回し検知）は新規UIを作らず、既存の Rust
CLI（`cargo run` / `src/bin/submit_demo.rs` / `cast send`）の出力をターミナルでそのまま実演する。
シーン3（World ID live-challenge・AI音声クローン対策）のみ、`stats.html` と同じパターン（ビルド
ツールなし・単体HTML + vanilla JS・バックエンドなし）で最小限のページを作る。詳細は
[SPEC.md §7 Step 5](SPEC.md) 参照。

**⚠️ シーン3は World ID 実SDKを使わずモック化することに最終決定（2026-09-21）**: 一度は
「`@worldcoin/idkit` で本物のSDK統合をちゃんとやる」方針にしたが、docs.world.org で現行フローを
確認したところ `RP_SIGNING_KEY` によるリクエスト署名用の小さなバックエンドが実質必須と判明
（署名ロジックは公式SDK以外に仕様が無く、Rustでの自前実装はリスクが高い）。2.5日でシーン1・2の
実演台本作成と並行してやり切る確信が持てず、**自前JSでの概念実演（モック）に戻す**ことで最終決定。
`nullifier_hash`（同一人物なら毎回同じ値になるWorld IDの性質）の代わりに、UIで人物を選ばせて
人物ごとに固定の疑似IDを割り当てる。**この判断により World ID/Worldcoinパートナー賞の対象からは
外れる可能性が高い**（コアのZK+RLNは実チェーン実装のままなので影響しない）。ピッチ/READMEには
「本番実装の設計は確認済み（RP署名＋v4 verify API、SPEC.md §7参照）、デモは時間制約でモック」と
正直に明記する。

- [x] UI方針決定（本節・SPEC §7/§10）
- [x] **デモ前提の不整合2件を発見・修正済み（2026-09-21）**:
  1. `submit_demo.rs`が9/20のroot rotation（extension A）後も旧secret（`"103"`/`"9003"`を5枚複製）の
     ままで、on-chainの実際の`familyRoot()`（rotation後: `18800580...659`）と食い違っていた。
     修正: leavesを`old_leaf`（103/9003）4枚+`leaves[0]`だけ`new_leaf`（203/9203）という
     rotate.rsと同じ木構成に直し、`cargo run --bin submit_demo -- 777`の`pubs[0]`が
     on-chainの`familyRoot()`と一致することを実行して確認済み
  2. `notifier.rs:28`の`from_block`が固定値（`34672100`）で、確認時点で約39,400ブロック
     （3時間以上のポーリング）遅れていた。本番当日はさらに乖離する。修正:
     `provider.get_block_number().await.unwrap().saturating_sub(50)`で起動時の最新ブロック付近
     から開始するように変更、ビルド確認済み
- [x] **シーン1・2のターミナル実演台本 確定（2026-09-21）**:
  - 事前準備: ターミナルAで`cargo run --bin notifier`常時起動、ターミナルBはコマンド待機、
    `cast`キーストアパスワードを事前に手元で確認（`submit_demo`実行ごとに1回、計2回入力する）
  - シーン1（~20秒）: `cargo run --bin submit_demo -- 777` → パスワード入力 →
    `status: 1 (success)`を指して「secretを出さずに本物のテストネットで検証成功」と説明
  - シーン2（~40〜60秒）: `cargo run --bin submit_demo -- 888`（challengeのみ変更）→ tx成功確認 →
    ターミナルAで最大30秒以内に`PotentialLeak`検知・`recovered secret = 203`表示 →
    「盗まれたsecretは使うほど自壊する」
  - （時間が余れば）締め: `cargo run --bin rotate` → `cast send updateRoot(...)` →
    `cast call familyRoot()`で更新確認、「検知→復元→通知→再発行→root更新」の一気通貫を見せる
  - UIをWeb化する案は検討したが見送り（2026-09-21）: 技術的には`Command`ベースの簡易Webサーバーで
    可能だが、③の`cast send`/notifierのS3アップロードと同じ新規サーフェスが増えるだけで、
    生のターミナル出力・実tx hashがそのまま見える方が「本物であること」の説得力が高いと判断
- [x] **③ `submit_demo.rs`に`cast send`自動実行を追加・実チェーンで確認済み（2026-09-21）**:
      末尾で`std::process::Command::new("cast")`を`.status()`で呼び、`verifyMembership`の
      calldataをそのまま渡して送信するように変更（`Command::status()`はデフォルトでstdin/stdout
      inheritなので`--account deployer`のキーストアパスワード入力もそのまま動く）。
      `cargo run --bin submit_demo`で実行 → `status: 1 (success)`、`ProofVerified`イベント発行を
      実チェーン上で確認済み（tx: `0x6fbfdb37aa08e58c26907b6af473c35fde153a42cf329bfb5747ccc16f26825b`、
      block `34712220`）。手動コピペ無しでprove→submitが1コマンドで通せるようになった
- [x] **シーン3の詳細設計 完了（2026-09-21、SPEC.md §7 Step 5 に記載済み、モック版で確定）**:
  - 単体HTML1枚に「証明する側」「検証する側」の2パネル（1画面内の対比、実デバイス2台は不要）
  - **鍵となる設計**: `signal`（合言葉、文字列そのまま比較）だけでなく、人物選択プルダウンで割り当てた
    **疑似`nullifier_hash`（人物ごとに固定のJS内定数）を事前登録した正規メンバーの期待値と突き合わせる**
    ことで「合言葉は合っているが本人ではない」を判定する
  - 正規メンバー実行: 「正規メンバー」を選択 → signal一致 ✅ + 疑似nullifier_hash一致 ✅ → 受理
  - 攻撃者（AIクローン音声）実行: 「攻撃者」を選択し同じ合言葉を入力 → signal一致 ✅ だが
    疑似nullifier_hash不一致 ❌ → 拒否（「secretを知っているだけでは通らない」の可視化）
  - on-chain Registry状態との突き合わせは**行わない（スコープ外に確定）**。判定はクライアント内で完結
- [x] **音声クローン（TTS）の実制作は見送り（2026-09-21）**: ElevenLabsのInstant Voice Cloningが
      無料プランに無く（Starterプラン、月$6〜が必要）、シーン3が既にモックである以上、本物の
      クローン音声を再生する演出は無くても論理的説得力は変わらないと判断。デモでは「AIクローンが
      合言葉を言ったとして」と口頭で説明し、そのまま攻撃者ロールとして合言葉をUIに入力する運用に変更
- [x] **合言葉を確定（2026-09-21）**: `とらのもん`。シーン3の`signal`比較は単純な文字列一致だが、
      ライブでの入力ゆらぎ（大文字小文字・前後スペース）を吸収するため`trim().toLowerCase()`してから
      比較する設計にする（HTML実装時に反映）

## 🆕 第2の差別化ポイント: World ID live-challenge による AI音声クローン対策（2026-09-19 採用、"Option A"）

RLN（盗んだ secret の使い回し検知）と**脅威モデルを分離**。RLN が防げない攻撃 — 攻撃者が secret を知らずとも AI で声をクローンしてなりすます攻撃 — をこちらで防ぐ。「家族の合言葉」という定番の詐欺対策を、暗号的に堅牢な形にする、というピッチ。

> **⚠️ 2026-09-21 追記: 以下は「本番実装として設計した」内容。実際のデモ（シーン3）は World ID 実SDKを
> 使わずモック化することに最終決定した（理由・詳細は上部「🆕 Step 5」節）。この節の設計自体は無効化
> していない — ピッチ/READMEで「設計はこれ、デモは時間制約でモック」と示す資料として残す。**IDKit
> widget・実機Orb不要のSimulatorに関する記述は「もし実SDK統合するなら」の設計であり、実際のデモ実装
> では使わない**。**

- **絶対に守る設計制約**: RLN の `challenge`/`epoch` を、この生の合言葉に**流用しない**。流用すると、正規メンバーが同一 epoch 内に別々の合言葉で2回正規の通話をしただけで RLN の自己暴露が発動し、secret が漏れてしまう。**完全に分離**すること — 合言葉は World ID の `signal` フィールド（`signal = hash(code)`）にのみ紐付ける。circom 側の変更は不要
- **検証は on-chain ではなく off-chain**。通話中にブロック確定を待つのは非現実的。フロー: 親が合言葉を読み上げる → 子の端末が (a) `signal=hash(code)` の World ID proof と (b) いつもの RLN proof（別物）を作成 → 親の端末が両方を off-chain 検証し、FamilyProof 側の結果は on-chain Registry の状態（現在の root・失効状況）と突き合わせる。Registry が唯一の正本、通話中の検証はそれに対する高速な off-chain チェック
- World ID には実機 Orb 不要の Simulator/staging モードがある → （本番実装するなら）デモは親子2役のブラウザ mini-app で十分（電話回線は不要）
- **スコープリスク（結果的に的中）**: これまで Rust + circom + Solidity だけだったスタックに JS/Web フロントエンド（IDKit widget、RP署名バックエンド）が新たに加わる懸念があり、2026-09-21 に実際に `RP_SIGNING_KEY` 署名の要件を確認して見送りモック化を決定した（詳細は上部「🆕 Step 5」節）
- 競合調査（2026-09-19時点）: ZK × World ID をオレオレ詐欺対策に組み合わせた既存プロジェクトは見つからず、独自性は高そう
- **ピッチの軸（2026-09-19 追加）**: 「合言葉にZKを足した」だけだとありがちなハッカソン構成（既存の地味な対策＋暗号を足すパターン）に見えるリスクがある。差別化点は技術要素そのものではなく問題設定の切り口 — 「秘密を知っているだけ」では AI 音声クローンに突破される、だから knowledge の証明(RLN)だけでなく liveness の証明(World ID live-challenge)が要る、という脅威分離のロジックをピッチの中心に置く
- **AI音声クローンデモ（2026-09-19 採用）**: 上記ロジックを言葉でなく体験として見せるため、Step 5 のデモに「クローン音声が正しい合言葉を言っても live-challenge で弾かれる」シーンを追加する。音声クローンは同意を得たチームメンバー本人の声でイベント前に事前生成（ライブ生成は音声合成の失敗・レイテンシ等のデモ事故リスクが高いため）、デモ本番ではその音声の再生＋検知部分のみライブで行う。RLN単体のデモ（漏洩secretの使い回し検知）とは別シーンとして構成。詳細は [SPEC.md](SPEC.md) Step 5 / §9 に反映済み

## 🆕 残り期間での追加拡張 A/B/C（2026-09-20 採用）

Step6コアがデプロイまで完了し、予定より前倒しで進んでいるため、9/20〜24の残り約4日を使って3つの拡張を追加することにした（Continuity Track の「既存部分」に含まれる — 当時はStep5デモ/World IDを「イベント中に新規に作る部分」として別枠にしていたが、**2026-09-21 にStep5は前倒しして「既存部分」側に含める方針に変更**。詳細は直下の進行計画・上部「Step 7」節参照）。優先順位はA→B→C。

**進行計画（2026-09-21 実態に合わせて再修正、同日中に2回再修正）**: A・Bとも9/20中に完了。CはTier1のみ9/21に完了、Tier2は見送り。A/B/Cすべて着地。~~本番まで残り4日はピッチ準備・バッファに充てる。~~ → ~~空いた時間は Trust Circle / Family Constitution 拡張（Step 7）に充てる。~~ → ~~空いた時間は Step 5 に充てる、Step 7 は本番に回す。~~ → **2026-09-21 夜 最終方針**: Step 5 が事実上完了したので、Step 7（下記節）を本番待ちにせず今から着手する。

- **A（最優先）: root rotation フロー ✅完了（2026-09-20）**。「secret漏洩検知→復元→失効」の話に、**実際にメンバーを木から除外して新rootをon-chainに反映する具体的な手順**が無かった穴を塞いだ。
  - **`src/bin/rotate.rs`を新規作成**: これまでのデモで実際に漏洩・復元した`secret="103"`（`FamilyRegistry`の現行root＝同secret/salt="9003"を5枚複製した木）のメンバーに、新しい`secret="203"`/`salt="9203"`を再発行し、`merkle::MerkleTree::from_leaves`で木を再構築して新root（`18800580504245480865872636926759076395359931162917691738659468170666629770659`）を算出。既存ロジックの延長のみ、新規暗号要素なし。
  - on-chain反映は`cast send 0xa9f1A920... "updateRoot(uint256)" <新root> --account deployer`（alloyでの署名実装はせず、Foundry付属の`cast`で十分「本物」。alloyは読み取り専用のBで使う、という役割分担を維持）。
  - デプロイ済み`FamilyRegistry`で実行し、**`familyRoot()`が新rootと一致・`RootUpdated`イベント発行（tx: `0x2762bce1...`）を確認済み**。これで「検知→復元→通知→再発行→木の再構築→on-chainでのroot更新」の全フローが実チェーン上で繋がったことを実証。
- **B: 通知インフラ＋匿名統計**。既存の「通知インフラ」「匿名統計の公開」タスク（下記Step6節）と同じもの。`notifier.rs` の `alloy` イベント監視基盤をAの検証にも使えないか要検討
- **C: 攻撃者の期待損失シミュレーション ✅Tier1のみで完了（2026-09-21）、Tier2は見送り**。
  - **Tier1（採用・完了）**: `src/bin/attacker_sim.rs`。「サービスなし」と「サービスあり（`block_rate`で決め打ち）」の2ケースで期待被害額を比較する決定論的な計算。出典を確認済みの警察庁統計（令和7年確定値、既遂1件あたり523.6万円/オレオレ詐欺サブタイプ785万円等。詳細は`familyproof-verified-scam-stats`メモリ参照）を使用。ピッチ用の数字はこれで確定。
  - **Tier2（見送り）**: 当初案は`epoch`/`limit`の設定を振って「攻撃者が使い回して捕まる確率」を小さいモンテカルロ（`src/bin/attack_sim.rs`に試作あり、`recover_secret`を実際に呼んで検知が成立することは実証済み）で定量化するものだった。検討の結果、(1) SPEC.md §3.2で「epoch長・limitのチューニングはやらない」と明記済み（デモはepoch=1時間/limit=1固定）でプロダクト設計と噛み合わない、(2) RLNの検知自体は決定論的（2回目に別challengeで証明したら100%失効）でモンテカルロで揺らす確率的要素が本質的に存在せず、乱数化できるのは「攻撃者が1epoch内に何回使うか」という**出典のない仮定**のみ、(3) 残り日程が逼迫（9/21時点で本番まで4日）、という3点から費用対効果が低いと判断し見送り。`src/bin/attack_sim.rs`は試作のまま残すが、Cの正式な成果物はTier1のみとする。

## 🆕 Step 7: Trust Circle / Family Constitution 拡張（2026-09-21 採用、**2026-09-21 夜から着手**）

設計は [SPEC.md §11](SPEC.md) に完了済み（データ構造・関数シグネチャ・デモ台本・既知の制約まで記載）。
**着手タイミングは2回変わった**: 当初「本番待ち」→ Step5優先のため一旦「本番中」に→ **Step 5 が
2026-09-21夜に事実上完了したため、本番を待たず今から着手**する最終方針に確定。Step5未完のまま
2本同時進行するリスクを避けるための「本番待ち」だったが、そのリスクは解消された。
Step 7自体は新しい暗号要素・新SDKを足さないSolidity/Rustの積み増しなので、着手タイミングが
前後しても難易度は変わらない。万一時間切れなら続きはイベント本番（9/25〜27）に持ち越す
（Continuity Trackの「イベント中に新規に作った部分」としてもそのまま成立する）。
以下は実装 TODO。**Claude はコード（`.sol`/`.rs`）を書かない** — 設計・レビュー・
`cargo build`/`cargo test`/`forge test` の実行確認のみ。

- [x] **① `contracts/src/FamilyConstitution.sol` 完成・`forge build`通過（2026-09-21）**:
  - `ActionState` struct、`actions` mapping（キーは `uint256 actionId`。§11.3の型の落とし穴通り、
    `bytes32`ハッシュではなく`Fr::from_le_bytes_mod_order`還元済みの値を唯一の正とする）
  - `ActionProposed`/`ActionApproved`/`ActionAuthorized` イベント、`proposeAction`（`onlyAgent`）、
    `approveAction`（root/epoch鮮度/`verifier.verifyProof()`/`challenge==actionId`/nullifier二重承認防止）、
    `getActionState`（`ActionState`がmappingを含むため`actions`をpublicにできず手書きgetterで対応）
  - **`registry`は`familyRoot`を自前で持たず、既存デプロイ済み`FamilyRegistry`を`IFamilyRegistry`
    インターフェース経由で参照する設計**（root rotation時の二重管理・ズレを回避）
  - 実装時に見つかった不具合3件（ユーザー自身で修正・コーチが指摘）: importのセミコロン抜け、
    `msg.sender`のタイプミス、**`verifier.verifyProof()`の呼び出し自体が抜けていた**（ZKの安全性が
    素通りになる重大な抜けだったが修正済み）。加えて全角スペース混入・`actionId`のタイプミスも修正
- [x] **② `contracts/test/FamilyConstitution.t.sol` 完成・`forge test`全14件PASS（2026-09-21）**:
      `MockVerifier`（`FamilyRegistry.t.sol`と同じ）＋新規`MockRegistry`（`IFamilyRegistry`実装、
      固定rootを返すだけ）で6テスト: tier0即実行 / tier2で1人目承認だけでは未実行 / 2人目承認で
      `ActionAuthorized` / 同一nullifierの二重承認はrevert / challengeが別Action用だとrevert /
      agent以外からの`proposeAction`はrevert。**「secretを持たない攻撃者は有効proofを作れない」は
      MockVerifierが常にtrueを返すためユニットテストでは検証不可**（`FamilyRegistry`と同じ構造上の理由）。
      本物の証明での検証は④（testnetでの一気通貫デモ）で自然にカバーされる
- [x] **③ `src/bin/propose_action.rs`（提案）/ `src/bin/approve_action.rs`（承認）: 両方実装・実チェーン(フォーク)でtier0/tier1/tier2まで動作確認済み（2026-09-22 完了）**:
  - **運用決定（2026-09-22）**: `agentAddress`用に`deployer`とは別の`cast`キーストア（`agent`、
    アドレス`0xF44de778B27FAe7554b582BC629AfF7238c3e519`）を新規作成
  - **anvil forkでのリハーサル方式を採用（2026-09-22）**: 実Sepoliaに直接デプロイする前に、
    `anvil --fork-url https://worldchain-sepolia.g.alchemy.com/public`でSepoliaをフォークした
    ローカルチェーンを使う。既存の`Groth16Verifier`/`FamilyRegistry`はフォーク元にそのまま存在する
    ため再デプロイ不要、`agent`への資金注入は`cast rpc anvil_setBalance <addr> <wei> --rpc-url
    http://127.0.0.1:8545`でfaucet待ちせず即座に可能。理由: これまでの開発で「実際動かして初めて
    見つかるバグ」（calldata座標のバグ、`verifyProof()`呼び忘れ、下記のtierハッシュのバグ等）が
    繰り返し起きているため、実Sepoliaのgas・待ち時間を消費する前に無料で何度もリハーサルできる場を
    用意した
  - `FamilyConstitution.sol`をforkにデプロイ（`forge create --rpc-url http://127.0.0.1:8545
    --account deployer --broadcast ...`。**`--broadcast`忘れでdry runのまま`Deployed to:`が出ない
    ハマりどころに遭遇・解決**）: フォーク上のアドレス`0x560726714672c28657c6a54421446a64EDfC2232`
  - `propose_action.rs`実装: `actionId`算出は`alloy::primitives::keccak256(description)` →
    `Fr::from_le_bytes_mod_order` → `.into_bigint().to_string()`で10進文字列にし、
    `cast send proposeAction(uint256,uint256) <actionId> <tier> --account agent`で送信。
    実装中に見つかった不具合2件（ユーザー自身で修正・コーチが指摘）:
    **`tier`まで`actionId`と同じくkeccakハッシュしてしまい`requiredApprovalsForTier`が常にrevert
    するバグ**（`tier`は0/1/2/3のenum値なのでハッシュ不要、生の整数のまま渡す）／送信先アドレスが
    `FamilyRegistry`のままで`FamilyConstitution`に向いていなかったバグ（selector不一致でempty revert）
  - **動作確認済み（フォーク上、2026-09-22）**: tier0（description="schedule reminder"）で
    `ActionProposed`+`ActionAuthorized`が同一tx内で発火、承認不要の即時実行を実チェーン上で確認。
    tier1（description="203"）は`proposeAction`は成功したが`approveAction`が未実装のため
    `executed=false`のまま（`getActionState`で確認済み）
  - **`approve_action.rs`実装（2026-09-22 完了）**: `submit_demo.rs`と同じ
    `build_circuit_with_inputs`→`setup`→`prove`→`to_solidity_calldata`の流れを使い、`challenge`は
    `description`文字列を`propose_action.rs`と全く同じ手順（`keccak256`→`Fr::from_le_bytes_mod_order`）
    でハッシュ化した`actionId`と一致させる。送信先は`FamilyConstitution.approveAction`、`cast send`の
    第1引数に`actionId`を明示的に渡す（`onlyAgent`制約は無いので送信者は`deployer`でよい）。
    CLI引数に承認者を選ぶ`leaf_idx`（0〜4）を追加し、複数メンバーでの承認をシミュレート可能にした
  - **実装中に見つかった不具合（ユーザー自身で修正・コーチが指摘、2026-09-22）**:
    `tree.proof(leaf_idx)`でMerkleパスは`leaf_idx`に応じて切り替えていたのに、証明に使う`secret`/`salt`
    自体が`leaf_idx`によらず常に`(203, 9203)`固定のままだったバグ。パスとsecret/saltの組み合わせが
    食い違うと回路が計算するrootが実際の木のrootと一致せず`"Invalid root"`でrevertする。
    `leaf_idx==0`なら`(203, 9203)`、それ以外は`(103, 9003)`を使うよう分岐させて解消
  - **重要な制約（2026-09-22）**: `nullifier`は`secret`のみ（＋`epoch`）で決まり、Merkle
    パスや`salt`には依存しない。現在on-chainの木はindex1〜4が全部同じ`secret="103"`の複製なので、
    承認2人分に使える「nullifierが別になる」実質的な組み合わせは`(secret="203", index0)`と
    `(secret="103", index1〜4のどれか)`の**2種類のみ**。tier2（承認2人）のデモにはちょうど足りるが、
    3人以上必要なActionは今の木では組めない（tier3はSPEC上デモでは省略可のため許容）
  - **動作確認済み（フォーク上、2026-09-22）**:
    - tier0（description="schedule reminder"）: `ActionProposed`+`ActionAuthorized`が同一tx内で発火
    - tier1（description="go to school", tier=1）: `approve_action`をindex0（secret=203）で1回実行 →
      `ActionApproved(1/1)`＋`ActionAuthorized`が同一txで発火
    - tier2（description="go to univ", tier=2、想定）: `approve_action`をindex0→index1の順で2回実行 →
      1回目は`ActionApproved(1/2)`のみ、2回目は`ActionApproved(2/2)`＋`ActionAuthorized`が発火。
      2回のnullifierが異なる値になっていることも確認済み（別人格からの承認であることの実証）
- [x] **World Chain Sepolia に `FamilyConstitution.sol` をデプロイし、§11.6 のデモシナリオ（AI提案→
      メンバー2人承認→`ActionAuthorized`）を実チェーン上で1回通す（2026-09-22 完了）**:
  - デプロイ: `forge create --account deployer --broadcast`（constructor引数は既存の
    `Groth16Verifier`/`FamilyRegistry`/`agent`の本番アドレスをそのまま使用）→
    `FamilyConstitution` = **`0xf7f344E9399638b69DF158877F1e77a39A5F3D73`**。
    `verifier()`/`registry()`/`owner()`/`agent()`をすべて`cast call`で期待値と一致することを確認済み
  - 事前準備: `agent`アカウントの本番Sepolia残高が0だったため、ユーザーの個人ウォレットから`deployer`へ
    0.1 ETH、`deployer`から`agent`へ0.02 ETH送金（`cast send --value ...`）してガス代を確保
  - `propose_action.rs`/`approve_action.rs`のハードコード2箇所（宛先アドレス・`--rpc-url`）を
    フォーク向けから本番Sepolia向けに書き換え（ユーザー自身が編集）
  - 実行: `propose_action "play work" 2`（tier2で`ActionProposed`）→
    `approve_action "play work" 0`（`ActionApproved(1/2)`）→
    `approve_action "play work" 1`（`ActionApproved(2/2)`＋`ActionAuthorized`）。
    `getActionState`で`tier=2, requiredApprovals=2, approvalCount=2, proposed=true, executed=true`を確認。
    tx: propose `0xc76a25bf...` / approve1 `0xa421b7f1...` / approve2 `0xdfee0dd2...`
  - これでStep 7（Trust Circle / Family Constitution拡張）は①〜④すべて完了。残る作業はデモ動画・
    英語版README/PITCH・ETHGlobal提出文・（stretchで）MCP化のみ
- [ ] SPEC.md §8（脅威モデル）に §11.3 で触れた RLN epoch/limit 共有問題を正式追記するかは、Step 7 の
      実装が固まった時点で判断（現状は §11.7 に既知の限界として記載済み）
- [ ] 時間切れの場合は SPEC §7 Step 7 の縮退ライン（Solidity実装のみ→testnet実証→デモ組み込みの順で削る）に従う

### 🆕 README/PITCH/提出文の整備（2026-09-22 夜、完了）・明日からMCP化に着手

Step 7 ①〜④完了後、審査員向けドキュメント一式を整備した。

- [x] **README.md 全面改稿**: 冗長だった語り口を削り、構造化された簡潔な形に書き直し（ユーザー主導のドラフトをベースにClaudeが最新状況の反映漏れ・正確性を指摘して修正）。修正点:
  - `In progress`節が古いままだった（承認CLI・本番デプロイ・実演は実際は完了済み）→ `Current Status`に統合
  - `Live Contracts`に`FamilyConstitution`のアドレスが抜けていた → 追加
  - World IDが他の実装済みprimitiveと同列に書かれていた（実際は自前JSモック）→ 明示的に区別する注記を追加
  - 見出し「RLN: Stolen Secrets Self-Destruct」→「RLN: Reusing a Stolen Secret Backfires」に変更（「自壊」という不正確な表現は2026-09-21に一度排除済みだったのが再発していたため）
  - AI利用方針の開示・既知の限界セクション・オレオレ詐欺の統計出典（削られていた）を簡潔な形で復元
  - S3公開版ダッシュボードのURL（`https://niikun.net/family_proof/`）を追記
- [x] **`README.en.md`（新規）**: 日本語版を主、英語版を副とする方針で作成。相互リンクを両ファイル冒頭に追加
- [x] **`docs/PITCH.en.md`（新規）**: 本番の実演自体は日本語で行う前提（英語版は資料用）で全訳
- [x] **「Step 0」「Step 7」等の内部管理番号を審査員向け文書から全削除**（ユーザー指摘: 「審査する側からは全く興味ないのでは」）。README/README.en/PITCH/PITCH.en/SUBMISSION.enの該当箇所を「Family Constitution（tier別Action Authorization）」のような機能名ベースの説明に置き換え。HANDOFF.md/SPEC.mdは開発ログなのでStep番号のまま維持
- [x] **PITCH.md/PITCH.en.mdの実態不一致を修正**:
  - シーン4前提が「デプロイ・実演はイベント本番のTODO」のままだったのを「イベント前に完了済み、本番はMCPサーバー化のみ」に修正
  - デモ手順を「MCP対応時／未対応時（フォールバック）」の2パターン併記に変更
  - 「間に合わなかった場合」節を「MCPが間に合わなかった場合」に作り直し（ベースのCLIデモがすでに実証済みなので、フォールバックの安全度が大幅に向上）
  - Q&Aの「イベント前／イベント中」の区切りを実態（Step0〜7＝イベント前、MCP＝イベント中）に修正
  - **サンプルコマンドのバグ2件を修正**: ①`--rpc-url`が抜けていて実際に接続エラーを再現した、②`actionId`を人間が事前計算して`cast send`に埋める前提の非現実的なコマンドだったのを、`description`文字列だけで完結する`cargo run --bin propose_action/approve_action`に置き換え（`propose_action`/`approve_action`が内部で自動的にactionIdを計算するため、人間が数値を扱う必要が無くなった）
  - ステップ4・5（隣人A・Bの承認）にも具体的な`cargo run --bin approve_action "<description>" <leaf_idx>`コマンドを追加（`leaf_idx`違いで別のnullifierになる点も明記）
- [x] **`docs/SUBMISSION.en.md`（新規）**: ETHGlobal提出フォーム用の下書き（タイトル・タグライン・description・Continuity Track区切り・tech stack・ライブコントラクト表）。パートナー賞は`ethglobal.com/events/tokyo2026/prizes`を検索して調査した結果、**World（`Best Use of IDKit` / `Best Use of World ID for Agents`、各$7,500）以外はFamilyProofの技術スタックと合わず候補外**と判断。World IDは自前モックなので要件を満たさない可能性が高いが、「同一パートナーの複数トラックは1枠としてカウント」というETHGlobalのルールにより応募コストがゼロなため、正直に開示した上でダメ元応募する方針に決定
- [x] **`propose_action.rs`/`approve_action.rs`のCLI出力を仕上げ**:
  - 成功/失敗の分岐バグを修正（以前は`status.success()`のチェック前に成功バナーを出力していたため、失敗時にも成功したように見えた）
  - 両ファイルの成功バナーを78文字幅・`# ラベル #`形式に統一
  - タイポ修正（`faild`→`failed`、`spwan`→`spawn`）
  - `approve_action.rs`: `.status()`を`.output()`に変更してcastの標準出力をキャプチャし、`ActionAuthorized`イベントのトピックハッシュ（`0xb402c6ca...`）を文字列検索することで、閾値到達で実行されたかどうか（`🎉 ActionAuthorized!` / `(pending — threshold not yet reached)`）をCLI上で分かりやすく表示するように改修
- [x] **`submit_demo.rs`のCLI出力も改善**: 成功時に`Membership Verified`バナー・`nullifier`の短縮表示（先頭8桁+末尾6桁、フルの78桁は読みにくいため）を追加
  - ~~未修正の既知バグ: `eprintln!("cast send failed: ...")`が`if status.success() {}`の中に紛れ込んでおり`else`が無い~~ → **2026-09-22中に修正済み**。`eprintln!`を`else`節に正しく移動、成功時はバナーのみ・失敗時はエラーメッセージのみが出ることを確認済み
- [x] **`.env`の不備を修正**: 末尾に`KEY=VALUE`形式でない生のtx hash行が残っており、`dotenvy::dotenv().ok()`（`notifier.rs`が使用）が`.ok()`でエラーを握りつぶすため`.env`全体が読み込まれなくなるリスクがあった → ユーザーが修正済み。`FamilyConstitution=0xf7f344...`の行も追加
- [x] **運用判断: `approve_action.rs`は`propose_action.rs`と同じ`agent`キーストアを使い続ける（隣人A・B用に別アカウントは作らない）**。理由: `FamilyConstitution.approveAction`には`onlyAgent`のような制限が無く、認証の実体は`msg.sender`ではなくZK証明（secret+Merkle path）なので、送信アカウントを分ける必要は無い。デモの見栄え上「隣人A/Bが同じアドレスから送信している」ことに気づかれるリスクはあるが、突っ込まれても正しく説明できるため許容
- [x] **本番デモでは`cargo run --release`を使うことを推奨**として記録。理由: Groth16の`prove`はデバッグビルドだと大幅に遅く、4分の持ち時間を圧迫するため。事前に`cargo build --release`しておき、本番中は`--release`付きで実行する運用
- [ ] **未決定: シーン3（World IDモック）をデモから丸ごとカットするか**。ユーザーから「デモが長すぎて分かりづらい、Family Constitutionの方が面白い」という指摘があり、Claudeはシーン3カット＋Family Constitutionに2分程度まで拡大する再配分案を提示したが、**最終決定はまだ**。次回セッションで確認すること
- [ ] **明日(2026-09-23)からMCP化に着手**。設計方針（2026-09-21時点の検討）: `propose_action`/`approve_action`を`rmcp`クレート（公式Rust MCP SDK、[modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk)、2026-09-22時点の最新版3.4系）でMCPサーバー化し、Claude Code自身がツール呼び出しでオンチェーン送信まで行えるようにする。ユーザーへのチュートリアル形式でのコーチングを開始済み（Step 1: `Cargo.toml`に`rmcp = { version = "3.4", features = ["server", "transport-io"] }` / `schemars = "0.8"` / `serde = { version = "1", features = ["derive"] }`を追加するところまで指示済み、実装はまだ）。最小サンプル構成（`#[tool_router(server_handler)]` + `#[tool(description = "...")]`付きメソッド + `stdio()`トランスポート + `ServiceExt::serve`）は調査済み。MCPサーバーが立ち上がったら、Claude Codeの設定（`.mcp.json`など）に登録して実際にツール呼び出しができるかを確認するのが次のマイルストーン

### 🆕 ピッチ再定義: Trust Circle を前面に出す（2026-09-21 深夜、完了）

Step 7の実装と並行して、ピッチ全体の枠組みを「オレオレ詐欺対策」から「血縁・婚姻・同居を前提としない
Trust Circleインフラ」（Family はその一実装）へ再定義した。[SPEC.md §11.1](SPEC.md) の
"Prove trust, not identity." を軸に、以下をすべて作成・公開済み:

- [x] **`README.md`（リポジトリ直下、新規）**: Trust Circle再定義を冒頭に、原点であるオレオレ詐欺対策、
      「✅実装済み／🚧本番TODO／🔭ビジョン」の正直な線引き（Continuity Track対策）、アーキテクチャ、
      Sepoliaデプロイ済みアドレス、セットアップ手順、既知の限界まで収録。日本語のみ（英語版は
      **日本語が固まってから翻訳する方針、まだ未着手**）
- [x] **`docs/PITCH.md`（新規）**: 4分デモの秒刻み台本（フック→シーン1〜4→締め）＋Q&A想定問答一式。
      シーン4は**Claude自身が実際にAI Agent役としてライブで登場する演出**に変更済み（3ターミナル構成:
      Claude/AI Agent・隣人A・隣人B。Claudeに自然文で相談→tier判断→コマンド提示→自分では承認できない
      ことを自分の口で説明、というメタな見せ場を追加。新規コード不要、`onlyAgent`のEOA設計は無変更）。
      ネットワーク不通時のミニフォールバックも記載
- [x] **スライド3枚（Slides Artifact、v3）**: URL `https://claude.ai/artifact/2dFv5fmLNMKJxw2hZbp9Cz`
      （private、要共有設定）。1枚目=フック（Trust Circle関係図＋「血縁・住所・氏名では"誰を信頼するか"は
      証明できない」の橋の一文）、2枚目=AI提案→リスク判定→低リスクAI実行/高リスクTrust Circle承認、という
      1本の意思決定フロー図（暗号技術の羅列ではなく）、3枚目=ビジョン（Trust Circleの5用途例＋タグライン）
- [x] **`SPEC.md §11.8`（新規節）**: 「AIにもTrust Circleのsecretを持たせるか」を検討し不採用に
      （承認閾値が実質1つ下がる／LLMはプロンプトインジェクションで乗っ取られうるためオレオレ詐欺と
      同型の脆弱性を持ち込む）。代わりに**「ZK-provable delegated capability」**（AIがTrust Circleから
      委任された制約つき権限をActionごとにレンジ証明で示す、委任の発行はAction Authorizationの
      仕組みを流用）を将来研究方向として設計・明記。今回は実装しない
- [x] **`SPEC.md §11.7` 追記（MPC）**: 「AI Agentにも承認権を持たせる」ではなく「AI Agent自身の署名鍵を
      MPC（閾値署名）で複数主体に分散して守る」案を将来構想として追記（Trust Circle側の非同期ZK承認フロー
      には影響しない、鍵管理レイヤーの話として§11.8と補完関係）。逆に「Trust Circle側の承認集約自体を
      MPCにする」案は、非同期承認UXを壊すため検討したが不採用、と明記。README にも1行反映済み
- [ ] **英語版README/PITCH**: 日本語版確定後に着手する方針（まだ未着手）
- [ ] **ETHGlobalプラットフォームへの提出文自体**（GitHub repo・description・partner-prize選択）はまだ書いていない
- [ ] **🆕 時間が余ったら（stretch）: シーン4のMCP化**（2026-09-21 深夜、検討）。今のシーン4は
      「Claudeが会話でコマンドを提案→人間がコピペで`cast send`を実行」という設計だが、`propose_action`等を
      MCPサーバーとしてラップし、**Claudeが実際にツールを叩いてon-chainに送信する**形に格上げできないか
      という案。MPC/委任権限（ZK）と違って暗号を一切触らない薄いラッパーなので技術的難易度は低いが、
      (a) Rust向けMCP SDKは未経験の新規依存、(b) 本番会場でMCPサーバーの接続を維持する必要があり、
      シーン4の不確定要素（ネットワーク・LLM揺れ）がさらに1つ増える、という2点がリスク。
      **優先順位: 必ず③Rust CLI（`propose_action.rs`、手動cast版）を先に完成・通しリハーサルまで
      済ませてから、時間が余った場合にのみ着手する**。「advise（助言）→execute（実行）」への格上げ演出、
      という位置づけ。
      **なぜ意義があるか（2026-09-21 深夜、後日追記）**: MCPのツール定義（何を呼べるか＝インターフェース層）
      と、ZK証明＋Trust Circleの承認（何が実際に許可されるか＝信頼の層）が疎結合になる。`propose_action`
      ツールをClaudeに渡しても、`approve_action`は呼べる形にしても有効な証明（＝secret）を作れず安全に
      失敗する——インターフェースをどれだけ広げても信頼の層が独立して守ってくれる、という構造そのものが
      ピッチの一言として使える: **"MCP defines the interface; ZK defines the trust."**
      （"Prove trust, not identity." と並ぶ副タグライン候補。実装する場合はPITCH.md/READMEにも反映する）

Claudeはこれらすべて`.md`のドキュメント執筆のみ（コーチ）で、`.sol`/`.rs`は一切書いていない。

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
  - **FamilyRegistry**: `0xa9f1A920A96c42BC4aA37DcB513CA615A3B7557d`（constructor: `verifierAddress`=上記, `initialRoot`=`13298919588855972999610419539218897354041933303975356579871528834317655849272`。この root は `secret="103"`/`salt="9003"` を5枚複製・depth4の木の root で決定論的に再現可能）
    - ⚠️ 同じ内容で `0xD06FcbB5CB9D3B094874855d3979C8ae8eA09144` にも1回デプロイ済み（2026-09-20、再デプロイにより重複）。以後は `0xa9f1A920...` を正とする。旧アドレスは放置（テストネットなので実害なし）
  - デプロイ後 `cast call` で `verifier()`/`owner()`/`familyRoot()` が期待値と一致することを確認済み
  - **ハマりどころ**: `cast wallet address --account deployer` はキーストア復号にパスワード入力が要るが、非対話環境（TTYなし）だと `No such device or address (os error 6)` で失敗する。パスワードが必要なコマンド（`cast wallet address`/`forge create --account`）は本人のターミナルで直接実行する運用にした
- [ ] **通知インフラ（2026-09-20 着手・進行中）**: `ProofVerified`/`PotentialLeak` イベントを監視して該当メンバーにメール通知。Rust の `alloy`（EVMログ取得・デコード）+ `reqwest`（Resend の REST API）+ `tokio` で `src/bin/notifier.rs` として実装中（Node.js ではなく Rust で統一）。
  - **`src/lib.rs`を新規作成**（`pub mod merkle; pub mod proof;`）し、`main.rs`/`src/bin/*.rs` 間で `merkle`/`proof` を共有できるように再構成済み（`main.rs`側は`crate::merkle`→`family_proof::merkle`に変更）。
  - **`src/bin/submit_demo.rs`を新規作成**: 固定木（`secret="103"`/`salt="9003"`を5枚複製・depth4、index0）で現在時刻ベースのepoch・challenge=777の本物の証明を作り、`to_solidity_calldata`でcalldata化。これを実際に`cast send`でデプロイ済み`FamilyRegistry`(`0xa9f1A920...`)の`verifyMembership`に送信し、**`status: 1 (success)`・`ProofVerified`イベント発行を実チェーン上で確認済み**（tx: `0xdf823cf0124ae7b9ba43595efb09ee923d479ea9a9747249aad056999e767c66`）。回路→Rust→calldata変換→testnetデプロイの全レイヤーが実際に繋がっていることの最終実証。
  - **`notifier.rs`で実際に`ProofVerified`ログを取得できることを確認済み**（`Filter::new().address(...).event_signature(ProofVerified::SIGNATURE_HASH).from_block(...).to_block(...)`）。
  - **ハマりどころ**: Alchemyの公開RPC(`worldchain-sepolia.g.alchemy.com/public`)は`eth_getLogs`のブロック範囲が**最大100ブロックまで**に制限されている。`from_block`を指定せず`to_block`も無いと「実質最新ブロックのみ」を見る扱いになり空の結果になる。本番の`notifier`としては「直近ブロック番号を都度取得し100ブロックずつ遡ってポーリングする」ような実装が必要（現状はまだ固定範囲のハードコードで実証しただけ、未実装）。
  - **`PotentialLeak`シナリオも実チェーン上で実証済み（2026-09-20）**: `submit_demo.rs`にコマンドライン引数でchallengeを変えられるようにし（`cargo run --bin submit_demo -- <challenge>`）、同じsecret/salt/epoch(497192)で`challenge=777`→`888`の2つの証明を生成、`cast send`で連続投入。1回目は`ProofVerified`のみ、**2回目は`PotentialLeak`＋`ProofVerified`の両方が発行**され、`nullifier`が2回とも完全一致することを確認（tx: `0x26f4caa9...`→成功、`0xbc9efc59...`→`PotentialLeak`）。RLNの「使い回し検知」がcircom回路→Rust→本物のtestnet上のコントラクトまで一気通貫で動くことの最終実証。
  - **ログのデコード（`SolEvent::decode_log`）は`notifier.rs`側で対応中**。`get_logs`が返す`Vec<Log>`から各`log.inner`を`ProofVerified::decode_log`/`PotentialLeak::decode_log`に渡して構造化データ化する。
  - **secret復元も本物のイベントデータで実証済み（2026-09-20）**: `notifier.rs`で`PotentialLeak::decode_log`した`challenge1,y1,challenge2,y2`（`alloy`の`U256`型）を`Fr::from_str(&x.to_string())`で`ark_bn254::Fr`に変換 → `merkle::hash_single`/`proof::recover_secret`に通し、**`recovered secret = 103`**（実際にsubmitしたsecretと完全一致）を確認。回路→Rust生成の証明→本物のtestnet上のコントラクト→イベントログ→off-chainでのsecret復元、という設計上の全レイヤーが実データで繋がっていることの最終実証。
  - ハマりどころ: `alloy`側の型（`U256`= `alloy_primitives::Uint<256,4>`）と`arkworks`側の型（`ark_bn254::Fr`）は同じ数値でも別crateの別型なので直接渡せない。既存コードと同じ「10進文字列を介した変換」（`Fr::from_str(&u256_value.to_string())`）で統一。
  - **Resend APIでのメール送信も実証済み（2026-09-20）**: `dotenvy`で`.env`の`RESEND_API_KEY`/`NOTIFY_TO`を読み込み、`recover_secret`の結果を`reqwest`経由で`https://api.resend.com/emails`にPOST。送信元は検証不要の`onboarding@resend.dev`を使用。実際にメール受信を確認済み。これで「漏洩検知 → secret復元 → 家族への通知」の一連の流れが`notifier.rs`単体で完結することを実証。
  - **ポーリングループ化 完了（2026-09-20）**: `from_block`を持ち回しながら`loop { get_block_number → from_block~min(from_block+99,latest)でget_logs → 処理 → from_block=to_block+1 → 30秒sleep }`という形に変更。100ブロック制限に対応。
    - ハマりどころ: `if`ブロックの中に誤って`let mut from_block = ...`をもう1回書いてしまい、外側の`from_block`をシャドーイングして「見た目は動くが実際は永遠に同じ範囲を見続けメールを送り続ける」バグが発生。`cargo check`の`unused_mut`/`value assigned...is never read`警告が発見の手がかりになった。
  - **匿名統計（日次カウント）完了（2026-09-20）**: `Filter`から`event_signature`指定を外して`address`のみで絞り込み、`log.topics()[0]`で`ProofVerified`/`PotentialLeak`を判定。`block_timestamp / 86400`を日次キーにして`HashMap<u64,(u64,u64)>`（`(ProofVerified件数, PotentialLeak件数)`）に集計、`stats.json`に書き出す。アドレス・nullifierは一切保存しない設計通り。
    - ハマりどころ: 最初`day`を`block_timestamp`の生値のままキーにしてしまい、秒単位でバラけて実質「イベント1件=1エントリ」になる（日次集計の意図から外れる）バグがあった。`/86400`で日単位に丸めて解消。
  - **Bのタスク（通知インフラ＋匿名統計）はこれで完了**。`notifier.rs`が「detect → recover → notify → aggregate」を1つのポーリングループで実行する。
- [x] **匿名統計の公開（2026-09-20 採用・2026-09-21 公開完了）**: 上記 `notifier.rs`（`alloy` でのイベント監視基盤）を流用し、`RootUpdated`/`PotentialLeak` を集計して「日次の検知件数」だけを公開する。family root・address 等の個人/家族を特定できる情報は公開側に一切出さない（日次カウントのみ、個別イベント単位の時刻・アドレスは出さない）。目的は「表面化しづらいオレオレ詐欺の試行実態を、被害者・家族を特定せずに可視化する」こと。ピッチの Practicality/社会的インパクトの補強にもなる。詳細は [SPEC.md](SPEC.md) Step 6 に反映済み
  - **公開ダッシュボード `stats.html`（新規作成・未コミット）**: `stats.json`（`notifier.rs` が書き出す日次集計）を `fetch` して棒グラフ表示する単一HTMLファイル。verified（ProofVerified）/leak（PotentialLeak）を日次で可視化。家族root・address等は一切含まない設計通り。
  - **S3へのアップロード完了（2026-09-21）**: `s3://niikun.net/family_proof/stats.json`（`index.html`として`stats.html`も）にアップロードして一般公開。CloudFront経由（`niikun.net`）だが現行のIAMユーザー（`s3user`）には`cloudfront:CreateInvalidation`権限が無く、キャッシュ削除は今回は見送り（TTL任せ、デモの成否には影響しない範囲と判断）。
  - **更新運用: notifier.rsからのS3自動アップロードに変更（2026-09-21、手動運用から再変更）**: `notifier.rs`が`stats.json`書き込み後に`aws s3 cp`を`Command`経由で自動実行するように変更済み（`aws` CLIは認証情報設定済みで動作確認済み、追加の認証情報渡しは不要）
  - **ローカルでのデモ視聴用に`stats.js`も追加（2026-09-21）**: `stats.html`は元々`fetch('stats.json')`していたため`file://`で直接開くとCORSで失敗し、`python3 -m http.server`が必要だった。会場Wi-Fi等への依存を避けるため、`notifier.rs`が`stats.json`と同時に`window.STATS_DATA = {...};`という中身の`stats.js`も書き出すように変更し、`stats.html`側は`<script src="stats.js"></script>`＋`window.STATS_DATA`読み込みに変更（`fetch`を廃止）。**これで`stats.html`をダブルクリックするだけでサーバー無しに開けるようになった**（動作確認済み）。デモ本番は`stats.html`をローカルで直接開く運用とし、S3上の公開版はキャッシュ状況に関わらず「もう一般公開もしている」と口頭で触れる程度に留める（[SPEC.md](SPEC.md)に対応する記述は無いためHANDOFFのみに記録）
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

- **追跡する**: `main.circom` / `scripts/` / `WORKFLOW.md` / `package.json` / `package-lock.json` / `circuits/input.json` / 共有鍵 `main_final.zkey` `verification_key.json` `pot12_final.ptau` / **`circuits/verifier.sol`（新規追加）** / **`contracts/`（Foundryプロジェクト一式。`contracts/lib/forge-std` は `forge init` が clone する外部依存、サブモジュールとして扱うか通常ファイルとして追跡するかは commit 時に要確認）** / **`stats.json` `stats.html`（2026-09-21 新規、匿名統計の公開ダッシュボード。S3にもアップロード済みだが repo 側でも追跡する）**
- **gitignore（各PCで再生成）**: `node_modules/` / `main_js/` / `main.r1cs` / `main.sym` / 中間 ptau / `main_0000.zkey` / `witness.*` / `proof.json` / `public.json`
- 回路を変えたら zkey/vkey は作り直して**両方コミット**（[WORKFLOW.md](../circuits/WORKFLOW.md) の「0→2」）。1つの鍵を両PCで共有するのが原則。

### ツール版

circom 2.2.3（`~/.cargo/bin`）/ snarkjs 0.7.6 / Node は nvm 管理（このWSLは v22.23.2）/ bn128 / `pot12`（2^12=4096、回路は ~2080 制約）。
