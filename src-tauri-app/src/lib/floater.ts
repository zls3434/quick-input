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

export function showTooltip(text: string, anchor: AnchorRect): void {
  currentKind = "tooltip";
  void invoke("show_floater", { kind: "tooltip", text, anchor }).catch((e) =>
    console.error("显示 tooltip 失败", e),
  );
}

export function hideTooltip(): void {
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
