# Day2: IDKitに実際に触る

## 前提

- Day1のTODO1〜6が終わっていること
- `learn-worldid/README.md` の「最優先」でSandboxを申請済みであること（未承認でも
  コードは書き進められます。承認が来たら実機で試す、という順番でOK）
- Developer Portal（https://developer.world.org/）でアプリを作成済みで、`app_id` を
  持っていること

## 全体構成（なぜこの形になるか）

FamilyProof本編（`voice_challenge.html`）はサーバー無し・単体HTMLでしたが、
本物のIDKitは**RP署名をバックエンドで作る必要がある**ため、今回だけは小さなサーバーが要ります
（`README.md`の用語集「RP signing」参照）。この制約が、2026-09-21にHANDOFFで
「実SDK統合は見送り」と判断した直接の理由でした。今回はその制約を実際に体験してみます。

```
[ブラウザ: public/index.html ]
   │ 1. 「証明する」ボタン
   ▼
[サーバー: server.js の /api/rp-signature]  ← ここでRP署名を作る（秘密鍵はサーバーだけが持つ）
   │ 2. 署名付きのrp_contextを返す
   ▼
[ブラウザ: IDKitにrp_contextを渡してリクエスト送信]
   │ 3. Sandboxアプリでスキャン → proofが返ってくる
   ▼
[サーバー: server.js の /api/verify-proof]  ← World公式のv4 verify APIに転送
   │ 4. 検証結果（本人確認OK/NG）
   ▼
[ブラウザに結果を表示]
```

## セットアップ（あなたが実行するコマンド）

```bash
cd learn-worldid/day2_idkit_practice
npm install
```

`package.json` に書いてある3つのパッケージが入ります:
- `express`: サーバーを立てるための最小限のフレームワーク
- `@worldcoin/idkit-server`: バックエンドでRP署名を作るための公式ヘルパー（`signRequest()`）
- `dotenv`: `.env` ファイルから環境変数（`app_id`や署名鍵）を読み込むための小さなライブラリ

## `.env` を自分で作る（絶対にコミットしない）

`day2_idkit_practice/.env.example` をコピーして `.env` を作り、Developer Portalで
取得した値を入れてください。

```bash
cp .env.example .env
```

`.gitignore` にはプロジェクト全体で `.env` が既に除外設定されているはずですが、
念のため `git status` で `.env` が出てこないか確認する癖をつけてください。

## やること（`server.js` と `public/index.html` のTODOを埋める）

1. `server.js` の TODO 1〜3: Express サーバーの起動、`/api/rp-signature` エンドポイント
   （`signRequest()` を呼ぶ）、`/api/verify-proof` エンドポイント（v4 verify APIへの転送）
2. `public/index.html` の TODO 1〜3: 「証明する」ボタンを押したら
   (a) `/api/rp-signature` を叩いてrp_contextをもらう
   (b) `@worldcoin/idkit-core` の `IDKit.request()` にapp_id・action・rp_contextを渡す
   (c) 返ってきた `nullifier_hash` を画面に表示する
3. `node server.js` で起動し、ブラウザで `http://localhost:3000` を開いて確認する

各TODOのすぐ上に、公式ドキュメントのどのページ・どの関数を見ればいいかを書いてあります。
詰まったらそこを読んでから聞いてください（読んでもわからなければもちろん聞いてOKです）。

## 確認できたら

- 同じ端末で2回証明してみて、`nullifier_hash` が毎回同じ値になることを確認する
  （＝これがFamilyProofの `challenge`/`nullifier` の発想と同じ仕組みだと実感する）
- Sandboxアプリの承認がまだなら、承認が来るまではここで一旦止めてOKです
- 終わったら、本番中にどこまでFamilyProofへ本組み込みするか（あるいはQ&A説明用の
  理解だけに留めるか）を一緒に判断しましょう
