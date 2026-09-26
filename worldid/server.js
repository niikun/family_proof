import express from "express";
import dotenv from "dotenv";
import { signRequest } from "@worldcoin/idkit-server";
import { keccak256, stringToBytes, toHex } from "viem";

dotenv.config();

const familyNullifiers = new Set(process.env.FAMILY_NULLIFIERS.split(","));

const app = express();
app.use(express.json());
app.use(express.static("public"));


app.listen(3000,() =>{
  
  console.log("http://localhost:3000/voice_challenge.html");
});


app.post("/api/rp-signature", async (req, res) => {
  const action = req.body.action || process.env.ACTION;
  const signReq = await signRequest({ signingKeyHex: process.env.RP_SIGNING_KEY, action, ttl: 300 });
  const { nonce, createdAt, expiresAt, sig } = signReq;   
  res.json({
    rp_id: process.env.RP_ID,
    nonce,
    created_at: createdAt,   
    expires_at: expiresAt,
    signature: sig,
  });
});


app.post("/api/verify-call", async (req, res) => {
  const request = req.body.IDKitResponse;
  const phrase = req.body.phrase?.trim() || process.env.PASSPHRASE;
  const url = `https://developer.world.org/api/v4/verify/${process.env.RP_ID}`;
  const response = await fetch(url,{
    method:"POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(request)
  });
  const data = await response.json();
  const humanOk = data.success === true;
  const reqSignalPhrase = request?.responses?.[0]?.signal_hash;
  const phraseOk = reqSignalPhrase ===  signalHashOf(phrase);
  const reqNullifier = request?.responses?.[0]?.nullifier;
  const memberOk = familyNullifiers.has(reqNullifier);
  res.json({humanOk, phraseOk, memberOk});
});

const signalHashOf = (signal) => {
  const bytes = stringToBytes(signal);
  const hash = BigInt(keccak256(bytes)) >> 8n;
  const hex = toHex(hash, {size: 32});
  return hex;
}
