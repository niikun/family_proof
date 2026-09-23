// Day1 JS基礎練習
// 上から順に埋めてください。1問ずつ `node learn-worldid/day1_js_basics/exercises.mjs` で確認できます。
// 拡張子が .mjs なのは TODO6 の import/export を素のNodeで動かすためです（気にしなくてOK）。

console.log("=== TODO 1: const とテンプレート文字列 ===");
// 指示: name という定数に自分の好きな文字列（例: "たろう"）を入れて、
//       `こんにちは、${name}さん` という形でコンソールに表示してください。
// ヒント: const name = "...";  console.log(`...`);

// TODO 1 ここに書く
let name = "たろう";
console.log(`こんにちは、${name}`);


console.log("\n=== TODO 2: アロー関数 ===");
// 指示: 引数を2つ受け取り、足し算した結果を返す関数 add を
//       アロー関数の形（const add = (a, b) => { ... }）で定義し、
//       add(2, 3) の結果をコンソールに表示してください。

// TODO 2 ここに書く
const add = (a, b) => {
  return a + b; 
}
let c = add(2, 3);
console.log(`the ans = ${c}`);


console.log("\n=== TODO 3: オブジェクトの分割代入 ===");
// 指示: 以下の worldIdResult は「IDKitが証明成功時に返すデータ」を模した
//       ダミーのオブジェクトです。分割代入を使って proof と nullifier_hash を
//       それぞれ変数として取り出し、コンソールに表示してください。
const worldIdResult = {
  proof: "0xdummy_proof_data",
  nullifier_hash: "0xdummy_nullifier_hash",
  merkle_root: "0xdummy_merkle_root",
};

// TODO 3 ここに書く（ヒント: const { proof, nullifier_hash } = worldIdResult;）
const {proof, nullifier_hash} = worldIdResult;
console.log(`${proof}, ${nullifier_hash}`);

console.log("\n=== TODO 4: async/await の基本 ===");
// 以下の waitOneSecond はすでに実装済みです（1秒待ってから文字列を返す関数）。
// これをそのまま使って、TODO部分だけ埋めてください。
function waitOneSecond(message) {
  return new Promise((resolve) => {
    setTimeout(() => resolve(message), 1000);
  });
}

async function runTodo4() {
  
  // 指示: waitOneSecond("1秒待ちました") を await して、結果をコンソールに表示してください。

  // TODO 4 ここに書く
  let msg = await waitOneSecond("1秒待ちました");
  console.log(msg);
}
await runTodo4();


console.log("\n=== TODO 5: fetch でPOST + JSONを読む ===");
// 指示: 以下のダミーAPI（httpbin.org、練習用の公開エコーサービス）に
//       { action: "verify-family-member" } というJSONをPOSTし、
//       返ってきた{レスポンスをJSONとしてパースして、
//       その中の json.json（自分が送ったbodyがechoされて返ってくる）を表示してください。
//   1. fetch(url, { method: "POST", headers: {...}, body: JSON.stringify({...}) })
//   2. const data = await response.json();
async function runTodo5() {
  const url = "https://httpbin.org/post";
  const res = await fetch(url,{
    method: "POST", 
    headers:{"Accept": "application/json",
                "Content-Type": "application/json"
        },
    body: JSON.stringify({action: "verify-family-member"})
      });
  const data = await res.json();
  console.log(data);
  return data;
}
await runTodo5();


console.log("\n=== TODO 6: import/export（モジュール分割）===");
// 指示: 同じフォルダに todo6_helper.mjs というファイルを新規作成し、
//       export const greet = (name) => `Hello, ${name}!`; という1行を書いてください。
//       その後、このファイルの一番上（他のimportがあればその並び）に
//       import { greet } from "./todo6_helper.mjs"; を追加し、
//       ここで greet("World") の結果を表示してください。
// これがまさに IDKit で書く `import { IDKitRequestWidget } from "@worldcoin/idkit"` と同じ構文です。

// TODO 6 ここに書く（greet の呼び出し部分。importは自分でファイル先頭に追加すること）
