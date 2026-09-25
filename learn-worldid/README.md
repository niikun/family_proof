# WorldID 練習用サンドボックス（本番提出物とは無関係）

このディレクトリは、本番（ETHGlobal Tokyo 2026, 9/25〜27）で WorldID 実SDK統合に
挑戦する前に、2026-09-23〜24 の2日間で軽く触って感覚を掴むための **練習場** です。

- FamilyProof 本編（`src/`, `circom/`, `contracts/`）とは完全に独立しています。ここのコードは
  Continuity Track の提出物には含めません（`git add` もしないでください）。
- 進め方は本編と同じ「コーチ方式」: Claude は設計・課題・参考リンクだけ用意します。
  **実際にコードを打つのはあなたです。** JavaScriptは今回が初めてという前提で、
  1行ずつ意味を説明しながら伴走します。詰まったらそのまま聞いてください。

## ⚠️ 最優先で今すぐやること（2026-09-23夜 更新: Simulatorを使う方針に変更）

当初は「Sandboxアプリ（実機のiOS/Android）の承認を先に申請」としていましたが、
**実機・アプリのインストールが一切不要なブラウザ版シミュレータ**が別にあることが
判明したため、練習用途ではこちらを使う方針に変更しました:

- https://simulator.worldcoin.org/ — 実機World Appの代わりになるブラウザツール。
  リクエスト時に `environment: "staging"` を指定するだけで使える
- Sandboxアプリの申請（iOS TestFlight / Android Google Play）は**不要**。
  （2026-09-23、Developer PortalでAndroidタブが選べずiOSしか出ない事象に遭遇したが、
  この方針変更によりそもそも申請自体が不要になったので解消済み）
- ただし**RP署名（バックエンドでの`signRequest()`呼び出し）は変わらず必須**。
  「実機が要らない」だけで、「バックエンドが要らない」わけではない

必要なのは以下だけです（Developer Portal自体は既に完了済みのはず）:

1. https://developer.world.org/ でDeveloper Portalのアカウントを作る（済）
2. アプリを1つ作成し、`app_id`・`rp_id`・署名鍵を取得する（済）
3. Day2の`server.js`/`public/index.html`のTODOを埋めて、`environment: "staging"`で
   simulator.worldcoin.orgと繋げてテストする

## 全体像（用語集）

| 用語 | 意味 |
|---|---|
| **App ID** | Developer Portalで作ったアプリの識別子。WorldIDに「どのアプリからのリクエストか」を伝える |
| **Action** | 「ユーザーが何の一意性を証明しようとしているか」を表す文字列。例: `"verify-family-member"`。同じActionに対しては同じ人は同じnullifierを出す |
| **Signal** | 証明に埋め込む追加データ（任意）。FamilyProofの設計では合言葉の`hash(code)`をここに使う予定だった（`HANDOFF.md`参照） |
| **nullifier** | 「App ID + Action」から導出される、その人固有の値。同一人物・同一Actionなら毎回同じ値になる ＝ 二重登録や使い回しを検知できる（`voice_challenge.html`の疑似実装と同じ考え方） |
| **Verification Level** | Orb（虹彩スキャン済み、最高保証）か Device（スマホの生体認証のみ）かの区別 |
| **RP signing** | Relying Party（＝あなたのアプリ）がバックエンドの秘密鍵でリクエストに署名する仕組み。2026年時点のIDKitでは**ほぼ必須**（クライアントに秘密鍵を置くと偽造されるため）。これが「Sandboxの前にバックエンドが要る」理由 |
| **Sandbox** | 本番の身元データに触れず、テスト用のリセット可能なアカウントで統合フローを試せる環境（実機アプリが必要。今回は使わない） |
| **Simulator** | https://simulator.worldcoin.org/ 。実機World Appの代わりになるブラウザツール。`environment: "staging"`を指定するだけで使え、アプリの承認待ちが不要。今回の練習で実際に使うのはこちら |

WorldIDの世界観がFamilyProof本編とどう繋がるかは `docs/HANDOFF.md` の
「🆕 第2の差別化ポイント」節と `docs/SPEC.md` §7 Step5 に書いてあります
（合言葉の`signal`、RLNの`challenge`とは完全に分離する設計など）。復習しておくと
Day2のIDKit実装が腑に落ちやすいはずです。

## 2日間ロードマップ

### Day 1（今日）: JS基礎
- [x] Developer Portalでアカウント作成・アプリ作成（`app_id`/`rp_id`/署名鍵取得、完了済み）
- [ ] `day1_js_basics/README.md` の小さい練習問題をこなす（`const`/`let`、関数、
      アロー関数、`async`/`await`、`fetch`、JSONの読み書き — IDKit統合で必ず使う分だけに絞ってあります）
- [ ] （余裕があれば）`voice_challenge.html` を読んで、すでにある疑似WorldID実装が
      何をしているか自分の言葉で説明できるようにする

### Day 2（明日）: IDKitに触る
- [ ] `day2_idkit_practice/NOTES.md` を読んで全体の流れを掴む
- [ ] `day2_idkit_practice/server.js` のTODOを埋めて、RP署名エンドポイントを自分で書く
- [ ] `day2_idkit_practice/public/index.html` のTODOを埋めて、IDKitウィジェットを組み込む
      （`environment: "staging"`を指定するのを忘れずに）
- [ ] https://simulator.worldcoin.org/ を開いた状態でリクエストを送り、実際にproofが
      返ってくるところまで確認する（実機・Sandboxアプリは不要）
- [ ] 終わったら本番当日にどこまでFamilyProofに組み込むか（またはQ&A説明だけに留めるか）を判断する

## 参考リンク（2026-09-23確認済み）

- 全体索引: https://docs.world.org/llms.txt
- IDKit統合ガイド: https://docs.world.org/world-id/idkit/integrate
- JavaScript(素のJS)版API: https://docs.world.org/world-id/idkit/javascript
- React版API: https://docs.world.org/world-id/idkit/react
- RP署名の仕様: https://docs.world.org/world-id/idkit/signatures
- Sandboxとは: https://docs.world.org/world-id/sandbox/what-is-sandbox
- Sandboxアクセス申請: https://docs.world.org/world-id/sandbox/sandbox-access
- エラーコード一覧: https://docs.world.org/world-id/idkit/error-codes
