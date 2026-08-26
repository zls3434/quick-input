// 浮层共享模块：悬浮窗前端通过此模块控制外置浮层窗口（tooltip / 右键菜单 / 顶栏）。
// 浮层渲染在独立透明窗口（floater）中，避免内容超出悬浮窗被裁剪。
import { invoke } from "@tauri-apps/api/core";

export interface AnchorRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface MenuItem {
  id: string;
  label: string;
  disabled: boolean;
  hint: string | null;
}

// 顶栏浮层（控制按钮组 + 分组标签）载荷：由悬浮窗前端构建并下发给浮层页面渲染
export interface ToolbarTab {
  name: string;
  active: boolean;
}

export interface ToolbarPayload {
  layout: "vertical" | "horizontal";
  opacityPct: number;
  alwaysOnTop: boolean;
  tabs: ToolbarTab[];
}

type Kind = "tooltip" | "menu" | "toolbar";

// 当前浮层承载类型：menu 打开期间按钮 mouseleave 不应触发关闭
// （tooltip 的 hide 仅在浮层仍为 tooltip 时生效），避免误关右键菜单。
let currentKind: Kind | null = null;

// 显示令牌：每次 show 递增。hide 只响应"自己那次 show"发出的 hide，
// 防止快速滑过多个按钮时，旧按钮的延迟隐藏误杀新按钮刚显示的 tooltip。
let currentToken = 0;

export function showTooltip(text: string, anchor: AnchorRect): number {
  const token = ++currentToken;
  currentKind = "tooltip";
  void invoke("show_floater", { kind: "tooltip", text, anchor }).catch((e) =>
    console.error("显示 tooltip 失败", e),
  );
  return token;
}

export function hideTooltip(token?: number): void {
  // 未显示过（token 为空）或令牌不匹配（已被更新的 show 覆盖）：
  // 该 hide 不针对当前浮层内容，忽略，防止快速滑过时旧按钮的
  // 延迟隐藏误杀新按钮刚显示的 tooltip。
  if (token === undefined || token !== currentToken) return;
  if (currentKind !== "tooltip") return;
  currentKind = null;
  void invoke("hide_floater").catch(() => {});
}

export function showMenu(items: MenuItem[], anchor: AnchorRect): void {
  currentKind = "menu";
  void invoke("show_floater", { kind: "menu", items, anchor }).catch((e) =>
    console.error("显示右键菜单失败", e),
  );
}

export function showToolbar(payload: ToolbarPayload): void {
  currentKind = "toolbar";
  // 顶栏锚点由后端直接取悬浮窗窗口矩形（show_floater Toolbar 分支），
  // 忽略前端锚点，传虚拟值即可。
  void invoke("show_floater", {
    kind: "toolbar",
    toolbar: payload,
    anchor: { x: 0, y: 0, w: 0, h: 0 },
  }).catch((e) => console.error("显示顶栏浮层失败", e));
}

// 顶栏自动隐藏前的淡出：通知浮层播放淡出动画，动画结束后由浮层自行隐藏。
// 仅当浮层当前承载顶栏时有效（tooltip/menu 走各自立即隐藏路径）。
export function fadeOutToolbar(): void {
  if (currentKind !== "toolbar") return;
  void invoke("floater_fade_out").catch(() => {});
}

// 右键菜单是否打开（悬浮窗前端据此抑制顶栏触发，避免误覆盖菜单）
export function isMenuVisible(): boolean {
  return currentKind === "menu";
}

export function hideFloater(): void {
  currentKind = null;
  void invoke("hide_floater").catch(() => {});
}
