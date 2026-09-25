// Day2: RP署名 + 検証用の最小サーバー
// 参考ドキュメント（このファイルを書く前にざっと目を通しておくとやりやすいです）:
//   - RP署名の作り方: https://docs.world.org/world-id/idkit/signatures
//   - proof検証の流れ: https://docs.world.org/world-id/idkit/integrate

import express, { json } from "express";
import dotenv from "dotenv";
import { signRequest } from "@worldcoin/idkit-server";

dotenv.config();

const app = express();
app.use(express.json());
app.use(express.static("public"));

// ---------------------------------------------------------------
// TODO 1: サーバーを起動する
// ヒント: app.listen(3000, () => { console.log(...) });
// ポート番号は public/index.html 側のfetch先（http://localhost:3000/...）と揃えること
// ---------------------------------------------------------------

app.listen(3000,() =>{
  console.log("...");
});

// ---------------------------------------------------------------
// TODO 2: RP署名エンドポイント
// やること:
//   1. req.body.action を受け取る（無ければ process.env.ACTION を使う）
//   2. signRequest({ signingKeyHex: process.env.RP_SIGNING_KEY, action, ttl: 300 }) を呼ぶ
//      → { nonce, created_at, expires_at, signature } が返ってくる（Promiseなのでawait）
//   3. res.json() で rp_id（process.env.RP_ID）と一緒に返す
//      例: res.json({ rp_id: process.env.RP_ID, ...署名結果 })
// ---------------------------------------------------------------
app.post("/api/rp-signature", async (req, res) => {
  const action = req.body.action || process.env.ACTION;
  const signReq = await signRequest({ signingKeyHex: process.env.RP_SIGNING_KEY, action, ttl: 300 });
  const { nonce, createdAt, expiresAt, sig } = signReq;   // ← 実際のキー名で受け取る
  res.json({
    rp_id: process.env.RP_ID,
    nonce,
    created_at: createdAt,   // camelCase → snake_case に変換して返す
    expires_at: expiresAt,
    signature: sig,
  });
});


// ---------------------------------------------------------------
// TODO 3: proof検証エンドポイント
// やること:
//   1. req.body.idkitResponse を受け取る（ブラウザ側がIDKitから受け取った結果をそのまま送ってくる）
//   2. fetch で POST https://developer.world.org/api/v4/verify/${process.env.RP_ID} に転送する
//      body は idkitResponse をそのまま JSON.stringify する
//   3. 返ってきたレスポンスをJSONとしてパースし、res.json() でブラウザに返す
// ---------------------------------------------------------------
app.post("/api/verify-proof", async (req, res) => {
  const request = req.body.IDKitResponse;
  const url = `https://developer.world.org/api/v4/verify/${process.env.RP_ID}`;
  const response = await fetch(url,{
    method:"POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request)
  });
  const data = await response.json();
  res.json(data);
});
