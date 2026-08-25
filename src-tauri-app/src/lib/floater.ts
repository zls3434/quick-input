// 浮层共享模块：悬浮窗前端通过此模块控制外置浮层窗口（tooltip / 右键菜单）。
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

type Kind = "tooltip" | "menu";

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

export function hideFloater(): void {
  currentKind = null;
  void invoke("hide_floater").catch(() => {});
}
