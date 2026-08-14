// 构建后处理：对齐 index.html 与 runtime chunk 的 __sveltekit_ 标识符
//
// 背景：SvelteKit 的 `svelte-kit sync` 每次生成随机 __sveltekit_<hash> 标识符，
// 在 vite 构建 SSR/client 阶段可能发生 hash 漂移，导致 fallback index.html
// 设置的全局标识符与 runtime chunk 读取的不一致。结果：应用启动时
// `globalThis.__sveltekit_xxx` 为 undefined，kit.start 崩溃，页面空白。
// 修复：以 chunks 中实际使用的 hash 为准，同步 index.html 中的 hash。
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const buildDir = join(__dirname, '..', 'build');
const immutableDir = join(buildDir, '_app', 'immutable');
const indexPath = join(buildDir, 'index.html');

// 1. 扫描所有 JS chunk 中的 __sveltekit_ 引用
const hashes = [];
function walk(dir) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(full);
    } else if (entry.name.endsWith('.js')) {
      const content = readFileSync(full, 'utf8');
      const matches = content.match(/__sveltekit_[a-z0-9]+/g) || [];
      hashes.push(...matches);
    }
  }
}
if (readdirSync(buildDir).includes('_app')) {
  walk(immutableDir);
} else {
  console.log('[postbuild] build/_app 不存在，跳过');
  process.exit(0);
}

// 取出现次数最多的 hash 作为权威值
const counts = {};
for (const h of hashes) counts[h] = (counts[h] || 0) + 1;
let chunkHash = null;
let max = 0;
for (const [h, c] of Object.entries(counts)) {
  if (c > max) { max = c; chunkHash = h; }
}

// 2. 读取 index.html 中的 hash
let html = readFileSync(indexPath, 'utf8');
const htmlMatch = html.match(/__sveltekit_[a-z0-9]+/);

if (!chunkHash) {
  console.log('[postbuild] chunks 中未找到 __sveltekit_ hash，跳过');
  process.exit(0);
}
if (!htmlMatch) {
  console.log('[postbuild] index.html 中未找到 __sveltekit_ hash，跳过');
  process.exit(0);
}

const htmlHash = htmlMatch[0];
if (htmlHash !== chunkHash) {
  html = html.replaceAll(htmlHash, chunkHash);
  writeFileSync(indexPath, html);
  console.log(`[postbuild] hash 对齐: index.html ${htmlHash} -> ${chunkHash}（与 runtime chunk 一致）`);
} else {
  console.log(`[postbuild] hash 一致: ${htmlHash}，无需修改`);
}
