//! 浮层窗口模块 (M2)
//!
//! 负责 QuickInput 的 tooltip 与右键菜单外置浮层窗口：
//! 内容渲染在独立透明窗口（floater）中，定位在悬浮窗外、屏幕工作区内，
//! 彻底摆脱悬浮窗尺寸约束（WebView2 内容在窗口物理边界处裁剪）。

/// 浮层窗口 label（与 tauri.conf.json 中的 label 一致）
pub const FLOATER_WINDOW_LABEL: &str = "floater";

/// 浮层与锚点之间的间距（物理像素）
const FLOATER_GAP: i32 = 8;

/// 浮层类型
#[derive(Clone, Copy, PartialEq)]
pub enum FloaterKind {
    /// 提示气泡：鼠标穿透、不抢焦点
    Tooltip,
    /// 右键菜单：可点击
    Menu,
}

/// 计算浮层窗口左上角位置（纯函数，物理像素，可单测）
///
/// 输入（均物理像素）：
/// - `anchor`: 锚点矩形（悬浮窗内按钮）`(left, top, right, bottom)`
/// - `size`: 浮层内容尺寸 `(width, height)`
/// - `work_area`: 悬浮窗所在显示器工作区 `(left, top, right, bottom)`
/// - `kind`: 浮层类型（决定垂直优先方向与水平对齐方式）
///
/// 规则：
/// - 垂直：tooltip 优先显示在锚点上方（保持原视觉）；menu 优先显示在
///   锚点下方（右键位置惯例）。一侧空间不足则翻转到另一侧；两侧都不足
///   则贴工作区底部完整显示（内容高于工作区时贴顶）。
/// - 水平：menu 左对齐锚点左缘；tooltip 相对锚点水平居中；越界时
///   钳制到工作区，保证浮层完整可见。
/// - 最终位置整体钳制在工作区内。
pub fn compute_floater_placement(
    anchor: (i32, i32, i32, i32),
    size: (i32, i32),
    work_area: (i32, i32, i32, i32),
    kind: FloaterKind,
) -> (i32, i32) {
    let (al, at, ar, ab) = anchor;
    let (w, h) = size;
    let (wal, wat, war, wab) = work_area;

    let above = at - FLOATER_GAP - h;
    let below = ab + FLOATER_GAP;
    let y = match kind {
        FloaterKind::Tooltip => {
            if above >= wat {
                above
            } else if below + h <= wab {
                below
            } else {
                (wab - h).max(wat)
            }
        }
        FloaterKind::Menu => {
            if below + h <= wab {
                below
            } else if above >= wat {
                above
            } else {
                (wab - h).max(wat)
            }
        }
    };

    let x = match kind {
        FloaterKind::Menu => al,
        FloaterKind::Tooltip => {
            let cx = al + (ar - al) / 2;
            cx - w / 2
        }
    };

    // 整体钳制到工作区：保证完整可见（浮层比工作区大时贴左/上）
    let x_min = wal;
    let x_max = (war - w).max(x_min);
    let y_min = wat;
    let y_max = (wab - h).max(y_min);
    (x.clamp(x_min, x_max), y.clamp(y_min, y_max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_places_below_when_room() {
        // 锚点 (400,300)-(500,340)，工作区 0,0~1920,1040，菜单 160x90
        let pos = compute_floater_placement((400, 300, 500, 340), (160, 90), (0, 0, 1920, 1040), FloaterKind::Menu);
        assert_eq!(pos, (400, 348)); // 340 + 8
    }

    #[test]
    fn test_menu_flips_above_when_below_crowded() {
        // 锚点底部紧贴工作区底：下方放不下，翻上方
        // （计划原数据 940 下方实际放得下：948+90=1038<=1040，故修正为 943 使放不下成立）
        let pos = compute_floater_placement((400, 900, 500, 943), (160, 90), (0, 0, 1920, 1040), FloaterKind::Menu);
        assert_eq!(pos, (400, 802)); // 900 - 8 - 90
    }

    #[test]
    fn test_menu_sticks_to_bottom_when_both_crowded() {
        // 工作区高度 100，菜单 90 高：上方 20 < 90、下方不足 → 贴底
        let pos = compute_floater_placement((0, 0, 100, 100), (160, 90), (0, 0, 1000, 100), FloaterKind::Menu);
        assert_eq!(pos, (0, 10)); // 100-90=10 完整显示
    }

    #[test]
    fn test_menu_clamps_horizontal_when_overflows_right() {
        // 锚点贴工作区右缘：左对齐越界 → 钳制回工作区内
        let pos = compute_floater_placement((1900, 300, 1920, 340), (160, 90), (0, 0, 1920, 1040), FloaterKind::Menu);
        assert_eq!(pos, (1760, 348)); // x: 1920-160
    }

    #[test]
    fn test_tooltip_places_above_when_room() {
        // tooltip 优先上方：100x40
        let pos = compute_floater_placement((400, 300, 500, 340), (100, 40), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos, (400, 252)); // 300-8-40；水平居中 450-50
    }

    #[test]
    fn test_tooltip_flips_below_when_above_crowded() {
        // 锚点贴工作区顶部：上方放不下 → 翻下方
        let pos = compute_floater_placement((400, 0, 500, 40), (100, 40), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos, (400, 48)); // 40 + 8
    }

    #[test]
    fn test_tooltip_clamps_horizontal_when_center_overflows_left() {
        // 锚点贴工作区左缘，tooltip 居中越左界 → 钳制回 0
        let pos = compute_floater_placement((0, 300, 100, 340), (120, 40), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos.0, 0); // 50-60=-10 → 0
    }

    #[test]
    fn test_float_larger_than_work_area_sticks_to_top_left() {
        // 浮层 2000x1100 大于工作区 1920x1040 → 贴 (0,0)
        let pos = compute_floater_placement((100, 100, 200, 140), (2000, 1100), (0, 0, 1920, 1040), FloaterKind::Tooltip);
        assert_eq!(pos, (0, 0));
    }
}
