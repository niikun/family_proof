# シーン①撮影記録: Claudeとの実際の会話（提案の判断）

`demo/family_proof_rough_cut.mp4` の「(1) Claudeとの実際の会話（提案の判断）」「(3) Claude自身の
自己承認失敗」「(4) tier0との対比」は**2026-09-23 すべて収録・編集・本編統合済み**。
台詞・流れの根拠は [docs/PITCH.md](../docs/PITCH.md) §1、tierの意味は [docs/SPEC.md](../docs/SPEC.md)
L407 の表に準拠。以降は完了記録・撮り直し時の再現手順として残す。

## 確定した入力（クイックリファレンス）

**description（確定・登録済み・tier=2・承認2/2・実行済み）**: `入院費の支払い: 300万円の振込`

1本目の録画（Part A+B、1本の連続テイクで収録済み）:

| # | 入力 |
|---|---|
| Part A | `入院費で300万円振り込まなければいけないんだけど、提案しておいて` |
| Part B | `あなた自身でこのactionを承認できる？できないなら、暗号的・原理的になぜ承認できないか100文字以内で教えて` |

2本目の録画（Part C、単独テイク）:

| # | 入力 |
|---|---|
| Part C | `予定のリマインドを出すというActionを提案しておいて` |

3本目・4本目の録画（`approve_action`、別ターミナル）:

```bash
cargo build --release --bin approve_action   # 先に済ませておく
cargo run --release --bin approve_action "入院費の支払い: 300万円の振込" 0
cargo run --release --bin approve_action "入院費の支払い: 300万円の振込" 1
```

## 収録環境（2026-09-23 最終版）

- **`family_proof`ディレクトリではなく、`$HOME`など無関係なディレクトリで`claude`を起動する**。
  理由: `family_proof`直下で起動すると、Claude(AI Agent役)が`docs/HANDOFF.md`に平文で書かれている
  デモ用secret（103, 203）を`grep`等で読めてしまい、「AIはsecretを持たないから承認できない」という
  暗号的な主張の前提が崩れる（Bashツール自体は使えるので`cd`されると完全な遮断ではないが、
  少なくとも起動直後の誤発見は防げる）
- **MCPサーバーは`user` scopeで登録しておく**（`family_proof`ディレクトリ外からでも`propose_action`
  ツールが使えるようにするため）:
  ```bash
  claude mcp add --scope user family-proof /home/userniikun/project/family_proof/target/release/mcp_server
  ```
- `mcp_server.rs`側の対応が必須: `Command::new("cargo")...`に`.current_dir(env!("CARGO_MANIFEST_DIR"))`
  を追加していないと、cwdが`family_proof`以外のときに`cargo run`が`Cargo.toml`を見つけられず
  `exit status: 101`で毎回失敗する（2026-09-23に発見・修正済み。`mcp_server.rs`のこの行が
  消えていないか撮り直し前に確認すること）
- 起動後、`/mcp`等で`propose_action`ツールが見えていることを確認してから本番に入る
- **`approve_action`はMCPツールとして絶対に追加・接続しない**。`approve_action.rs`はデモ用に
  secret/saltがハードコードされているため、MCP経由でClaudeに叩かせると「secretが無いから
  承認できない」はずのAI Agentが実際に承認を成功させてしまい、台本の主張と正反対の結果になる

## 収録時の注意点（過去の失敗から）

- **「Accessing workspace / trust this folder」の初回確認画面は使わない前提で録画する**。
  新しいディレクトリで初めて`claude`を起動すると出るが、内容的に不要なので編集でカットする。
  録画自体は開始しておいて構わない（後でトリムする）
- **tierを人間から言わない・示唆しない**。「入院費で300万円」のような金額・状況だけ伝え、
  tier判断はClaude自身にさせる
- **Part Bの質問には最初から「暗号的・原理的に」「100文字以内で」を織り込む**。
  以前は「あなた自身が承認できる？できない場合は理由も」とだけ聞いたところ、1回目は
  「MCPにapprove_actionツールが無いから」という的外れな答えが返り、聞き直す2往復になった。
  文字数と観点を最初の質問に埋め込むことで1往復で決まる
- **Part Cは「予定リマインドを出しておいて」ではなく「予定のリマインドを出すというActionを
  提案しておいて」と聞く**。「予定リマインド」だけだと実際のGoogleカレンダーツールを読みに
  行ってしまうことがあった。「Actionを提案して」と明示すると`propose_action`に迷わず向かう。
  さらに「通院の」等の具体的な状況を付けるとtier1に判断されることがあるので、完全に汎用的な
  「予定のリマインド」のままにする（tier0に寄せるため）
- **`approve_action`は`--release`必須**。debugビルドのままだと証明生成が約17〜35秒かかり
  間延びする。事前に`cargo build --release --bin approve_action`を済ませておく
- **コマンドを打ってすぐ実行しない・迷わない**。過去の収録で「コマンドを打つ→放置→画面クリア→
  タイポ（`clc`）→打ち直し」という混乱が録画に残り、編集で該当区間をカットする羽目になった。
  本番コマンドを一度確認してから、迷わず実行する

## 編集時の注意点（過去の失敗から）

- **プロンプトを打った直後に結果へジャンプカットしない**。「早送りはしていないのに早送りに
  見える」という指摘を受けた。thinking表示（`Contemplating…`等）が数秒見えている状態を
  多少残してから結果に飛ぶ方が自然に見える。無音区間の検出（`freezedetect`）で機械的に
  切るだけでなく、体感のテンポも確認すること
- **アプリの許可画面・タイポの打ち直しなど、内容的に無価値な数秒は遠慮なくカットしてよい**
  （これは「早送りに見える」問題とは別の話——中身のある反応を削るのがNG）

## 完了記録

- [x] **Part A（提案）**: 「入院費で300万円振り込まなければいけないんだけど、提案しておいて」→
      tier=2と自己判断→`propose_action`実行→`ActionProposed`。tool側の実際の`description`は
      `入院費の支払い: 300万円の振込`
- [x] **Part B（自己承認拒否）**: 「あなた自身でこのactionを承認できる？できないなら、暗号的・
      原理的になぜ承認できないか100文字以内で教えて」→1往復で「Trust Circleのsecretを持たないため
      有効なZK証明を生成できず承認できない」という簡潔な回答
- [x] **Part C（tier0対比）**: 「予定のリマインドを出すというActionを提案しておいて」→tier=0と
      自己判断→即時実行
- [x] **隣人A・B承認（`approve_action`、`--release`）**: idx-0/idx-1とも実行、`ActionApproved`→
      `ActionAuthorized`まで確認
- [x] 音声削除・無音区間トリム・バッジ合成・本編統合（Family Constitutionセクションを新規収録に
      全面差し替え、旧「母の介護費用」テイクは削除）
- [x] `docs/PITCH.md`/`docs/PITCH.en.md`のdescription例・Part C台詞を実際の収録内容に合わせて更新
