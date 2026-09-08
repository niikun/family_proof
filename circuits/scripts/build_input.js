// 回路用 input.json を生成する雛形（Poseidon / 深さ 4 固定）
//
//   cd circuits && npm i circomlibjs
//   node scripts/build_input.js
//
// 回路 main.circom の前提:
//   - 葉のハッシュは Poseidon([0, secret, salt])（3入力、先頭 0 はドメインタグ）
//   - 節（内部ノード）のハッシュは Poseidon([L, R])（2入力）
//   - nLevels = 4  → 葉は 2^4 = 16 枚
//   - pathIndices[k] = 0: 自分が左 / 1: 自分が右
//
// 出力: circuits/input.json  ＋ 標準出力に root

const fs = require("fs");
const path = require("path");
const { buildPoseidon } = require("circomlibjs");

const LEVELS = 4;
const WIDTH = 1 << LEVELS; // 16

// --- TODO 1: メンバー集合 ---------------------------------------------------
// 実際の family メンバーを表す値。ここでは数値の羅列をダミーで置いている。
const MEMBERS = [
  {secret: "101", salt: "9001"},
  {secret: "102", salt: "9002"},
  {secret: "103", salt: "9003"},
  {secret: "104", salt: "9004"},
  {secret: "105", salt: "9005"}
];

// --- TODO 2: 証明したい葉のインデックス ------------------------------------
const TARGET_INDEX = 2;

// --- TODO 3: パディング値 ------------------------------------------------------
// 「最後の葉を複製」は N と N+1 が同じ root を作れる偽造余地があるため使わない。
// 固定の空値で埋める（必要ならドメイン分離した定数に変更）。
const EMPTY_LEAF = 0n;

async function main() {
  const poseidon = await buildPoseidon();
  const F = poseidon.F;
  const H2 = (a, b) => F.toObject(poseidon([a, b])); // BigInt を返す

  // --- TODO 4: 葉の作り方 --------------------------------------------------
  // 葉 = Semaphore 型コミットメント Poseidon([0, secret, salt])。
  // 先頭 0 はドメインタグ（葉と節でハッシュのレンジを分離）。
  // salt はメンバーごとの 2 つ目の乱数で、secret と一緒に端末内に保存し外部送信しない。
  const leafOf = (m) => F.toObject(poseidon([0n,BigInt(m.secret),BigInt(m.salt)]));

  if (MEMBERS.length > WIDTH) throw new Error(`メンバーが ${WIDTH} を超えている`);
  if (TARGET_INDEX >= MEMBERS.length) throw new Error("TARGET_INDEX が範囲外");

  // level[0] = 葉（固定値でパディングして 16 枚に）
  const levels = [[]];
  for (let i = 0; i < WIDTH; i++) {
    levels[0].push(i < MEMBERS.length ? leafOf(MEMBERS[i]) : EMPTY_LEAF);
  }

  // ボトムアップに 4 段構築
  for (let k = 0; k < LEVELS; k++) {
    const cur = levels[k];
    const next = [];
    for (let i = 0; i < cur.length; i += 2) {
      next.push(H2(cur[i], cur[i + 1]));
    }
    levels.push(next);
  }
  const root = levels[LEVELS][0];

  // 対象の葉の証明パスを取り出す
  const pathIndices = [];
  const siblings = [];
  let idx = TARGET_INDEX;
  for (let k = 0; k < LEVELS; k++) {
    const bit = idx & 1; // 0: 自分が左 / 1: 自分が右
    pathIndices.push(bit.toString());
    siblings.push(levels[k][idx ^ 1].toString()); // 兄弟ノード
    idx >>= 1;
  }

  const input = {
    leaf: levels[0][TARGET_INDEX].toString(),
    pathIndices,
    siblings,
  };

  const outPath = path.join(__dirname, "..", "input.json");
  fs.writeFileSync(outPath, JSON.stringify(input, null, 2) + "\n");

  console.log("wrote", outPath);
  console.log("root =", root.toString());
  console.log(JSON.stringify(input, null, 2));
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
