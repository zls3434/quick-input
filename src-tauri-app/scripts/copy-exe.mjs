// 构建产物分发：把打包出的绿色版单文件 exe 复制到项目根目录
//
// 约定（见 .trae/rules/project_rules.md「构建产物约定」）：
// - 打包完成后，src-tauri/target/release/quickinput.exe 复制为项目根目录 QuickInput.exe
// - 根目录 QuickInput.exe 仅供本地实测，已在 .gitignore 排除，禁止提交
// - 若上次测试的程序还在运行（文件被锁）：自动结束该进程 → 复制新版 → 自动重新启动新版
import { copyFileSync, statSync, existsSync } from 'node:fs';
import { execFileSync, spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const src = join(__dirname, '..', 'src-tauri', 'target', 'release', 'quickinput.exe');
const dest = join(__dirname, '..', '..', 'QuickInput.exe');

if (!existsSync(src)) {
  console.error('[copy-exe] 未找到打包产物: ' + src + '，请先执行 npm run tauri:build');
  process.exit(1);
}

// 同步 sleep（不额外起进程）
function sleep(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

// 按可执行文件路径精确匹配运行中的根目录实例
// （不误伤 target/debug 下的 dev 实例；路径经环境变量传入，免引号转义）
function findRunningPids() {
  const ps =
    'Get-CimInstance Win32_Process | ' +
    'Where-Object { $_.ExecutablePath -eq $env:QI_EXE } | ' +
    'Select-Object -ExpandProperty ProcessId';
  const out = execFileSync('powershell', ['-NoProfile', '-Command', ps], {
    env: { ...process.env, QI_EXE: dest },
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'ignore'],
  });
  return out
    .split(/\r?\n/)
    .map((s) => s.trim())
    .filter(Boolean)
    .map(Number)
    .filter(Number.isInteger);
}

// 结束指定进程（应用配置即改即存，强制结束安全）
function stopPids(pids) {
  const ps = 'Stop-Process -Id ' + pids.join(',') + ' -Force -ErrorAction SilentlyContinue';
  execFileSync('powershell', ['-NoProfile', '-Command', ps], { stdio: 'ignore' });
}

const pids = findRunningPids();
const wasRunning = pids.length > 0;
if (wasRunning) {
  console.log(`[copy-exe] 检测到根目录程序仍在运行 (pid=${pids.join(',')}), 自动关闭...`);
  stopPids(pids);
}

// 复制（带重试：进程退出与杀毒扫描可能短暂占用文件；残留进程每 ~1s 复查补杀）
let copied = false;
let lastErr = null;
for (let attempt = 0; attempt < 30 && !copied; attempt++) {
  try {
    copyFileSync(src, dest);
    copied = true;
  } catch (e) {
    lastErr = e;
    if (attempt > 0 && attempt % 5 === 0) {
      const again = findRunningPids();
      if (again.length > 0) stopPids(again);
    }
    sleep(200);
  }
}
if (!copied) {
  const hint =
    lastErr?.code === 'EPERM' || lastErr?.code === 'EBUSY'
      ? '文件仍被占用：若程序以管理员运行，请用管理员终端执行打包，或手动关闭后重试'
      : lastErr?.message;
  console.error('[copy-exe] 复制失败: ' + hint);
  process.exit(1);
}

const kb = Math.round(statSync(dest).size / 1024);
console.log(`[copy-exe] 已复制绿色版到项目根目录: QuickInput.exe (${kb} KB)`);

// 之前在运行 → 自动重启新版（分离进程，不阻塞构建命令退出）
if (wasRunning) {
  try {
    const child = spawn(dest, [], { detached: true, stdio: 'ignore', cwd: dirname(dest) });
    child.unref();
    console.log('[copy-exe] 已重新启动新版 QuickInput.exe');
  } catch (e) {
    console.warn('[copy-exe] 自动重启失败，请手动运行根目录 QuickInput.exe: ' + e.message);
  }
}
