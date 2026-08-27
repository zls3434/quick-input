// 生成 tauri-plugin-updater 所需的 latest.json 元数据（发布流程用）
// usage: node scripts/make-update-json.mjs <version> <setupExePath> <sigPath> [notes]
// 输出: <setupExe 同目录>/latest.json（默认），或第 4 个参数指定路径
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

// 更新包下载前置镜像（方案 A：GitHub 直连国内不可达，经公益镜像 ghfast.top 代理；
// 签名校验仍由 updater 用内置公钥完成，镜像仅传输层）。迁移到 Gitee/OSS 时改此处即可。
const DOWNLOAD_MIRROR = "https://ghfast.top/";
// 空字符串 = GitHub 直连（海外用户或镜像整体故障时回退形态）

const [v, setup, sig, notesArg] = process.argv.slice(2);
if (!v || !setup || !sig) {
  console.error("usage: node scripts/make-update-json.mjs <version> <setupExe> <sig> [notes]");
  process.exit(1);
}
const signature = readFileSync(sig, "utf8").trim();
const url = `${DOWNLOAD_MIRROR}https://github.com/zls3434/quick-input/releases/download/v${v}/QuickInput_${v}_x64-setup.exe`;
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
