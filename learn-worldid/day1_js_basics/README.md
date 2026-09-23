# Day1: JS基礎（IDKitで実際に使う分だけ）

JavaScript全部を勉強する必要はありません。Day2の `IDKitRequestWidget` の
コード例（`README.md`の「参考リンク」から見た公式サンプル）に出てくる書き方だけ
先に手を動かして慣れておきます。

## 準備: Node.jsが入っているか確認

```bash
node -v
```

エラーになったら教えてください（インストール手順を案内します）。

## 進め方

`exercises.js` を開くと、6個の `// TODO` があります。上から順に、
コメントの指示どおりに1〜2行ずつ埋めてください。書けたら都度これで実行して
確認できます。

```bash
node learn-worldid/day1_js_basics/exercises.js
```

「わからない」「これで合ってる？」となったら、その場で聞いてください。
答えを先に渡すのではなく、ヒントを出しながら一緒に進めます。

## 各TODOで学ぶこと（先に全体像だけ）

1. **TODO 1**: `const`（再代入しない変数）と テンプレート文字列 `` `こんにちは、${name}` ``
   → IDKitの `app_id="app_xxxxx"` のような文字列組み立てで毎回使う
2. **TODO 2**: 関数の書き方（アロー関数） `const f = (x) => { ... }`
   → `handleVerify={async (result) => { ... }}` のこの形そのもの
3. **TODO 3**: オブジェクトの分割代入 `const { a, b } = obj`
   → IDKitが返す `{ proof, nullifier_hash, merkle_root }` を受け取るときに必須
4. **TODO 4**: `async`/`await` の基本（`Promise`を待つ）
   → WorldIDのproof取得は全部非同期（サーバーと通信するので即座には終わらない）
5. **TODO 5**: `fetch` でPOSTリクエストを送り、`JSON`を読む
   → Day2の「バックエンドにRP署名をもらいに行く」処理そのもの
6. **TODO 6**: `import`/`export`（モジュール分割）
   → `import { IDKitRequestWidget } from "@worldcoin/idkit"` の意味がわかるようになる

やり終えたら教えてください。書いたコードを見て、必要ならその場でフィードバックします。
