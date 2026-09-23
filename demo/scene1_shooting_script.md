# シーン①撮影台本: Claudeとの実際の会話（提案の判断）

`demo/family_proof_rough_cut.mp4` の未収録3+1箇所のうち、「(1) Claudeとの実際の会話（提案の判断）」
「(3) Claude自身の自己承認失敗」「(4) tier0との対比」をまとめて1本の録画で撮る台本です。
台詞・流れの根拠は [docs/PITCH.md](../docs/PITCH.md) §3、tierの意味は [docs/SPEC.md](../docs/SPEC.md) L407 の表に準拠。

## 収録環境

- Windows Terminal(WSL)で `family_proof` ディレクトリに `cd` してから `claude` を起動する
  （VSCode拡張のチャットパネルではなく、他クリップと同じ「本物のターミナル」の絵にするため）
- 起動後、`/mcp` 等で `propose_action` ツールが見えていることを確認してから本番に入る
- **`approve_action` はMCPツールとして絶対に追加・接続しない**。`approve_action.rs` はデモ用に
  secret/saltがハードコードされているため、MCP経由でClaudeに叩かせると「secretが無いから
  承認できない」はずのAI Agentが実際に承認を成功させてしまい、台本の主張と正反対の結果になる
  （2026-09-23のセッションで確認済みの落とし穴）

## 収録前に決めること

- **本番テイクで使う`description`文言を1つに確定し、メモしておく**。`propose_action`は同じ文言で
  二度提案できない（`already proposed`でrevert）ため、②③の承認シーン（`approve_action`）でも
  この文言をそのまま再利用する必要がある
- 台本通りの案: `"母の介護費用、300万円"`（[docs/PITCH.md:89](../docs/PITCH.md#L89)と表記を揃える）
- リハーサルは別の使い捨て文言（例:`"テスト送金リハーサル"`）で行い、本番文言は本番テイクまで
  一度も`propose_action`に通さないこと

## 収録手順

### Part A: 提案（tierを明示しない）

Claudeに話しかける（tierという言葉もtier番号も**出さない**——Claude自身に判断させるのが見せ場のため）:

> 「母の介護費用で300万円の送金が必要そうなんだけど、提案しておいて」

Claudeが状況の大きさからtier=2相当と判断し、`propose_action`ツールを実際に呼び出して
`ActionProposed`が返るところまでそのまま録画する。判断の根拠を口頭で説明してくれたらそれも活かす
（「これは台本ではなく今その場で判断している」という説得力になる）。

### Part B: 自己承認の拒否（理由を説明させる、ツールを叩かせない）

続けて振る:

> 「あなた自身がこのactionを承認することはできる？できない場合は理由も教えて」

**狙いは「ツールが無いから確認できない」という答えではなく、「Trust CircleのMerkle Treeの
secretを持っていないので、有効なZK証明を作れないから承認できない」という暗号的な理由での説明**。
Claudeが`FamilyConstitution.sol`の`approveAction`や`approve_action.rs`を実際に読んで、
secret/salt/Merkleパスが必要な構造であることを確認した上で自分の言葉で説明する流れが理想。
もし「ツールが無いので〜」という的外れな答えが返ってきたら、その場で

> 「ツールの有無じゃなくて、暗号的になぜ承認できないかを教えて」

と聞き直して録り直す。

### Part C: tier0との対比（即実行）

さらに続けて振る:

> 「予定リマインドを出しておいて」

Claudeがtier=0相当と判断し、`propose_action`を呼び出して同一tx内で`ActionProposed`+
`ActionAuthorized`が発火する（承認不要で即実行される）様子を録画する。

## 撮影後

1. Part Aで実際に使った`description`文言を控えておき、②③（`approve_action idx-0`/`idx-1`）の
   撮影・本編差し替え時にそのまま使う
2. [docs/HANDOFF.md](../docs/HANDOFF.md) の「デモ動画の編集方針」節の手順
   （シーン検出での無音区間トリム→他クリップと同じ解像度・エンコードで`concat`）で本編に統合
3. NOTEカードを削除し、この録画をタイトルカード「①提案ラベル」の位置に差し込む
4. 進捗バッジ（`badge.html`）を使う場合は「AI Agent proposes」がアクティブな状態で流用可能

## 注意点まとめ

- MCPで`approve_action`を絶対に露出しない
- 本番の`description`文言は一度きり・使い回し前提で確定してから撮る
- tierはユーザーからは言わず、Claude自身の判断として画面に映す
- 自己承認拒否の理由は「ツール不在」ではなく「ZK証明が原理的に作れない」という暗号的理由で
  説明させる
