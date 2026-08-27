// 生成 tauri-plugin-updater 所需的 latest.json 元数据（发布流程用）
// usage: node scripts/make-update-json.mjs <version> <setupExePath> <sigPath> [notes]
// 输出: <setupExe 同目录>/latest.json（默认），或第 4 个参数指定路径
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const [v, setup, sig, notesArg] = process.argv.slice(2);
if (!v || !setup || !sig) {
  console.error("usage: node scripts/make-update-json.mjs <version> <setupExe> <sig> [notes]");
  process.exit(1);
}
const signature = readFileSync(sig, "utf8").trim();
const url = `https://github.com/zls3434/quick-input/releases/download/v${v}/QuickInput_${v}_x64-setup.exe`;
const json = {
  version: v,
  notes: notesArg ?? `QuickInput v${v}`,
  pub_date: new Date().toISOString(),
  platforms: {
    "windows-x86_64": { signature, url },
  },
};
const outPath = resolve(dirname(setup), "latest.json");
writeFileSync(outPath, JSON.stringify(json, null, 2) + "\n");
console.log("written:", outPath);
