use serde::{Deserialize, Serialize};

// ---- 数据结构（与前端共享） ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
    pub bits_per_pixel: u32,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub source_name: String,
    pub friendly_name: String,
    pub current: Mode,
    pub modes: Vec<Mode>,
    pub hdr_supported: bool,
    pub hdr_enabled: bool,
}

/// 前端托管过来的"方案"：分辨率+刷新率+HDR+可选快捷键
#[derive(Debug, Clone, Deserialize)]
pub struct Plan {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub source_name: String,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub refresh: u32,
    #[serde(default)]
    pub hdr: Option<bool>,
    #[serde(default)]
    pub bits: Option<u32>,
    /// 形如 "Ctrl+Alt+F1"，空串/缺省表示未绑定
    #[serde(default)]
    pub shortcut: Option<String>,
}

// ---- Win32 实现（仅 Windows） ----

#[cfg(windows)]
mod win {
    use super::{Mode, MonitorInfo};
    use windows::core::{PCWSTR};
    use windows::Win32::Foundation::{HWND, LUID};
    use windows::Win32::Devices::Display::{
        DisplayConfigGetDeviceInfo, DisplayConfigSetDeviceInfo, GetDisplayConfigBufferSizes,
        QueryDisplayConfig, DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO,
        DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
        DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE,
        DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO, DISPLAYCONFIG_PATH_INFO,
        DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE, DISPLAYCONFIG_SOURCE_DEVICE_NAME,
        DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
    };
    use windows::Win32::Graphics::Gdi::{
        ChangeDisplaySettingsExW, EnumDisplaySettingsW, CDS_UPDATEREGISTRY, DEVMODE_FIELD_FLAGS,
        DEVMODEW, DM_BITSPERPEL, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT, DM_PELSWIDTH,
        ENUM_CURRENT_SETTINGS, ENUM_DISPLAY_SETTINGS_MODE,
    };

    fn u16_arr_to_string(arr: &[u16]) -> String {
        let end = arr.iter().position(|&c| c == 0).unwrap_or(arr.len());
        String::from_utf16_lossy(&arr[..end])
    }

    fn is_ok_i32(v: i32) -> bool {
        v == 0
    }

    /// 取当前活动显示路径（source gdi 名 + target 适配器/目标 id + 友好名）
    pub struct ActivePath {
        pub source_name: String,
        pub friendly: String,
        pub adapter_id: LUID,
        pub target_id: u32,
    }

    pub fn active_paths() -> Vec<ActivePath> {
        let mut out = Vec::new();
        unsafe {
            let mut n_path: u32 = 0;
            let mut n_mode: u32 = 0;
            let qflags = QDC_ONLY_ACTIVE_PATHS;
            if GetDisplayConfigBufferSizes(qflags, &mut n_path, &mut n_mode).is_err() {
                return out;
            }
            if n_path == 0 {
                return out;
            }
            let mut paths: Vec<DISPLAYCONFIG_PATH_INFO> = vec![std::mem::zeroed(); n_path as usize];
            let mut modes: Vec<windows::Win32::Devices::Display::DISPLAYCONFIG_MODE_INFO> =
                vec![std::mem::zeroed(); n_mode as usize];
            if QueryDisplayConfig(
                qflags,
                &mut n_path,
                paths.as_mut_ptr(),
                &mut n_mode,
                modes.as_mut_ptr(),
                None,
            )
            .is_err()
            {
                return out;
            }

            for path in &paths[..n_path as usize] {
                // 仅活动路径（DISPLAYCONFIG_PATH_ACTIVE = 1）
                if path.flags & 1 == 0 {
                    continue;
                }
                // 源 GDI 设备名
                let mut src = std::mem::zeroed::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>();
                src.header = windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
                    size: std::mem::size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32,
                    adapterId: path.sourceInfo.adapterId,
                    id: path.sourceInfo.id,
                };
                if DisplayConfigGetDeviceInfo(&mut src.header) != 0 {
                    continue;
                }
                let source_name = u16_arr_to_string(&src.viewGdiDeviceName);
                if source_name.is_empty() {
                    continue;
                }
                // 目标友好名
                let mut tgt = std::mem::zeroed::<DISPLAYCONFIG_TARGET_DEVICE_NAME>();
                tgt.header = windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
                    size: std::mem::size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as u32,
                    adapterId: path.targetInfo.adapterId,
                    id: path.targetInfo.id,
                };
                let mut friendly = source_name.clone();
                if DisplayConfigGetDeviceInfo(&mut tgt.header) == 0 {
                    let f = u16_arr_to_string(&tgt.monitorFriendlyDeviceName);
                    if !f.is_empty() {
                        friendly = f;
                    }
                }
                out.push(ActivePath {
                    source_name,
                    friendly,
                    adapter_id: path.targetInfo.adapterId,
                    target_id: path.targetInfo.id,
                });
            }
        }
        out
    }

    fn enum_modes(source_name: &str) -> Vec<Mode> {
        let mut modes = Vec::new();
        let mut name: Vec<u16> = source_name.encode_utf16().collect();
        name.push(0);
        unsafe {
            let mut i = 0u32;
            loop {
                let mut dm = DEVMODEW::default();
                if !EnumDisplaySettingsW(PCWSTR(name.as_ptr()), ENUM_DISPLAY_SETTINGS_MODE(i), &mut dm)
                    .as_bool()
                {
                    break;
                }
                modes.push(Mode {
                    width: dm.dmPelsWidth,
                    height: dm.dmPelsHeight,
                    refresh: dm.dmDisplayFrequency,
                    bits_per_pixel: dm.dmBitsPerPel,
                    index: i,
                });
                i += 1;
                if i > 1024 {
                    break;
                }
            }
        }
        modes
    }

    fn current_mode(source_name: &str) -> Mode {
        let mut name: Vec<u16> = source_name.encode_utf16().collect();
        name.push(0);
        unsafe {
            let mut dm = DEVMODEW::default();
            if EnumDisplaySettingsW(PCWSTR(name.as_ptr()), ENUM_CURRENT_SETTINGS, &mut dm).as_bool() {
                Mode {
                    width: dm.dmPelsWidth,
                    height: dm.dmPelsHeight,
                    refresh: dm.dmDisplayFrequency,
                    bits_per_pixel: dm.dmBitsPerPel,
                    index: u32::MAX,
                }
            } else {
                Mode {
                    width: 0,
                    height: 0,
                    refresh: 0,
                    bits_per_pixel: 0,
                    index: u32::MAX,
                }
            }
        }
    }

    /// 读取 HDR（高级颜色）状态：返回 (supported, enabled, forced)
    fn hdr_state(adapter_id: LUID, target_id: u32) -> (bool, bool, bool) {
        unsafe {
            let mut info = DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
                header: windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO,
                    size: std::mem::size_of::<DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO>() as u32,
                    adapterId: adapter_id,
                    id: target_id,
                },
                ..std::mem::zeroed()
            };
            if DisplayConfigGetDeviceInfo(&mut info.header) != 0 {
                return (false, false, false);
            }
            let value = info.Anonymous.value;
            let supported = (value & 0b001) != 0;
            let enabled = (value & 0b010) != 0;
            let forced = (value & 0b100) != 0;
            (supported, enabled, forced)
        }
    }

    fn find_target(source_name: &str) -> Option<(LUID, u32)> {
        active_paths()
            .into_iter()
            .find(|p| p.source_name == source_name)
            .map(|p| (p.adapter_id, p.target_id))
    }

    pub fn list_monitors() -> Vec<MonitorInfo> {
        active_paths()
            .into_iter()
            .map(|p| {
                let cur = current_mode(&p.source_name);
                let modes = enum_modes(&p.source_name);
                let (supported, enabled, _) = hdr_state(p.adapter_id, p.target_id);
                MonitorInfo {
                    source_name: p.source_name,
                    friendly_name: p.friendly,
                    current: cur,
                    modes,
                    hdr_supported: supported,
                    hdr_enabled: enabled,
                }
            })
            .collect()
    }

    pub fn apply_mode(source_name: &str, width: u32, height: u32, refresh: u32) -> Result<Mode, String> {
        let mut name: Vec<u16> = source_name.encode_utf16().collect();
        name.push(0);
        unsafe {
            // 先测试
            let mut i = 0u32;
            let mut matched: Option<DEVMODEW> = None;
            loop {
                let mut dm = DEVMODEW::default();
                if !EnumDisplaySettingsW(PCWSTR(name.as_ptr()), ENUM_DISPLAY_SETTINGS_MODE(i), &mut dm)
                    .as_bool()
                {
                    break;
                }
                if dm.dmPelsWidth == width
                    && dm.dmPelsHeight == height
                    && (refresh == 0 || dm.dmDisplayFrequency == refresh)
                {
                    matched = Some(dm);
                    break;
                }
                i += 1;
                if i > 1024 {
                    break;
                }
            }
            let mut dm = matched.ok_or_else(|| "未找到匹配的显示模式".to_string())?;
            dm.dmFields = DEVMODE_FIELD_FLAGS(
                DM_PELSWIDTH.0 | DM_PELSHEIGHT.0 | DM_BITSPERPEL.0 | DM_DISPLAYFREQUENCY.0,
            );
            let res = ChangeDisplaySettingsExW(
                PCWSTR(name.as_ptr()),
                Some(&dm as *const DEVMODEW),
                Some(HWND::default()),
                CDS_UPDATEREGISTRY,
                None,
            );
            if res.0 != 0 {
                return Err(format!("切换失败，系统返回代码 {}", res.0));
            }
            Ok(current_mode(source_name))
        }
    }

    pub fn set_hdr(source_name: &str, enable: bool) -> Result<bool, String> {
        let (adapter_id, target_id) =
            find_target(source_name).ok_or_else(|| "找不到该显示器活动路径".to_string())?;
        unsafe {
            let mut packet = DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
                header: windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_HEADER {
                    r#type: DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE,
                    size: std::mem::size_of::<DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE>() as u32,
                    adapterId: adapter_id,
                    id: target_id,
                },
                ..std::mem::zeroed()
            };
            packet.Anonymous = windows::Win32::Devices::Display::DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
                value: if enable { 1 } else { 0 },
            };
            let res = DisplayConfigSetDeviceInfo(&packet.header);
            if !is_ok_i32(res) {
                return Err(format!("HDR 设置失败，返回代码 {res}"));
            }
            let (_, enabled, _) = hdr_state(adapter_id, target_id);
            Ok(enabled)
        }
    }
}

// ---- 非 Windows 占位 ----

#[cfg(not(windows))]
mod win {
    use super::{Mode, MonitorInfo};
    pub fn list_monitors() -> Vec<MonitorInfo> {
        Vec::new()
    }
    pub fn apply_mode(_s: &str, _w: u32, _h: u32, _r: u32) -> Result<Mode, String> {
        Err("此功能仅支持 Windows".into())
    }
    pub fn set_hdr(_s: &str, _e: bool) -> Result<bool, String> {
        Err("此功能仅支持 Windows".into())
    }
}

// ---- Tauri 命令 ----

#[tauri::command]
fn list_monitors() -> Vec<MonitorInfo> {
    win::list_monitors()
}

#[tauri::command]
fn apply_mode(
    source_name: String,
    width: u32,
    height: u32,
    refresh: u32,
) -> Result<Mode, String> {
    win::apply_mode(&source_name, width, height, refresh)
}

#[tauri::command]
fn set_hdr(source_name: String, enable: bool) -> Result<bool, String> {
    win::set_hdr(&source_name, enable)
}

/// 真正退出应用：关闭确认弹窗里选择“直接关闭”时由前端调用
#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

// ---- 方案托管：托盘菜单 + 自定义全局快捷键 ----

use std::collections::HashSet;
use std::sync::Mutex;

use serde_json::json;
use tauri::{
    menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, Wry,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// 未收到前端同步前的默认循环快捷键
const DEFAULT_CYCLE_SHORTCUT: &str = "Ctrl+Alt+R";
const TRAY_ID: &str = "main";

/// 开机自启时附加的启动参数：带上它表示"静默启动"，只显示托盘图标。
/// 该常量同时用于注册自启项和解析命令行，两边必须一致。
const HIDDEN_FLAG: &str = "--hidden";

/// 判断本次是否应当静默启动（由开机自启拉起）
fn started_hidden() -> bool {
    std::env::args()
        .skip(1)
        .any(|a| a.eq_ignore_ascii_case(HIDDEN_FLAG) || a.eq_ignore_ascii_case("/hidden"))
}

#[derive(Default)]
struct AppData {
    plans: Vec<Plan>,
    cycle_shortcut: String,
    cycle_index: usize,
}

#[derive(Default)]
struct AppState(Mutex<AppData>);

fn norm_shortcut(s: &Option<String>) -> Option<String> {
    let t = s.as_deref().unwrap_or("").trim().to_string();
    if t.is_empty() {
        None
    } else {
        Some(t)
    }
}

/// 应用一个方案：切分辨率/刷新率，必要时设置 HDR
fn apply_plan_core(plan: &Plan) -> Result<String, String> {
    win::apply_mode(&plan.source_name, plan.width, plan.height, plan.refresh)?;
    if let Some(hdr) = plan.hdr {
        if let Err(e) = win::set_hdr(&plan.source_name, hdr) {
            eprintln!("HDR 设置失败（分辨率已切换）: {e}");
        }
    }
    Ok(format!(
        "{} · {}x{} @{}Hz",
        plan.name, plan.width, plan.height, plan.refresh
    ))
}

/// 收集要注册的快捷键：方案快捷键 + 循环快捷键（去重，冲突时方案优先）
fn collect_shortcuts(data: &AppData) -> (Vec<(String, String)>, String) {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for p in &data.plans {
        if let Some(sc) = norm_shortcut(&p.shortcut) {
            if seen.insert(sc.to_lowercase()) {
                out.push((sc, p.id.clone()));
            }
        }
    }
    let cycle = {
        let c = data.cycle_shortcut.trim().to_string();
        if c.is_empty() || seen.contains(&c.to_lowercase()) {
            String::new()
        } else {
            c
        }
    };
    (out, cycle)
}

/// 重建托盘右键菜单（含方案列表）。菜单更新统一投递到主线程执行。
fn rebuild_tray_menu(app: &AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || rebuild_tray_menu_inner(&handle));
}

fn rebuild_tray_menu_inner(app: &AppHandle) {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return;
    };
    let plans: Vec<Plan> = app
        .try_state::<AppState>()
        .and_then(|d| d.0.lock().ok().map(|x| x.plans.clone()))
        .unwrap_or_default();

    let build = (|| -> tauri::Result<()> {
        let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
        let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
        let sep_top = PredefinedMenuItem::separator(app)?;

        let mut items: Vec<Box<dyn IsMenuItem<Wry>>> = Vec::new();
        items.push(Box::new(show));
        items.push(Box::new(sep_top));
        for p in &plans {
            let label = if p.name.is_empty() {
                format!("{}x{} @{}Hz", p.width, p.height, p.refresh)
            } else {
                p.name.clone()
            };
            items.push(Box::new(MenuItem::with_id(
                app,
                format!("plan:{}", p.id),
                label,
                true,
                None::<&str>,
            )?));
        }
        if !plans.is_empty() {
            items.push(Box::new(PredefinedMenuItem::separator(app)?));
        }
        items.push(Box::new(quit));
        let refs: Vec<&dyn IsMenuItem<Wry>> = items.iter().map(|i| &**i).collect();
        let menu = Menu::with_items(app, &refs)?;
        tray.set_menu(Some(menu))
    })();
    if let Err(e) = build {
        eprintln!("重建托盘菜单失败: {e}");
    }
}

/// 托盘菜单与全局快捷键共用的方案应用入口
fn apply_plan_by_id(app: &AppHandle, id: &str) {
    let plan = app
        .try_state::<AppState>()
        .and_then(|s| s.0.lock().ok().and_then(|d| d.plans.iter().find(|p| p.id == id).cloned()));
    let Some(plan) = plan else { return };
    emit_apply(app, &plan.id, || apply_plan_core(&plan));
}

/// 循环切换到下一个方案（后端直接应用，不依赖前端脚本）
fn cycle_apply(app: &AppHandle) {
    let next = {
        let state = app.state::<AppState>();
        let Ok(mut d) = state.0.lock() else { return };
        if d.plans.is_empty() {
            return;
        }
        d.cycle_index = (d.cycle_index + 1) % d.plans.len();
        let p = d.plans[d.cycle_index].clone();
        let id = p.id.clone();
        (p, id)
    };
    emit_apply(app, &next.1, || apply_plan_core(&next.0));
}

fn emit_apply<F: FnOnce() -> Result<String, String>>(app: &AppHandle, plan_id: &str, f: F) {
    let payload = match f() {
        Ok(desc) => json!({ "planId": plan_id, "ok": true, "msg": desc }),
        Err(e) => json!({ "planId": plan_id, "ok": false, "msg": e }),
    };
    let _ = app.emit("plan-applied", payload);
}

/// 前端修改方案/快捷键后调用：重新注册全局快捷键并刷新托盘菜单。
/// 必须 async：快捷键插件内部要等主线程，同步命令会死锁。
#[tauri::command]
async fn sync_plans(plans: Vec<Plan>, cycle_shortcut: String, app: AppHandle) -> Result<(), String> {
    {
        let state = app.state::<AppState>();
        let mut d = state.0.lock().map_err(|_| "状态锁定失败".to_string())?;
        d.plans = plans;
        d.cycle_shortcut = cycle_shortcut.trim().to_string();
    }
    let (plan_shortcuts, cycle) = {
        let state = app.state::<AppState>();
        let d = state.0.lock().map_err(|_| "状态锁定失败".to_string())?;
        collect_shortcuts(&d)
    };

    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    if !cycle.is_empty() {
        let res = gs.on_shortcut(cycle.as_str(), |app, _sc, event| {
            if event.state() == ShortcutState::Pressed {
                cycle_apply(app);
            }
        });
        if let Err(e) = res {
            eprintln!("注册循环快捷键 {cycle} 失败: {e}");
        }
    }

    for (sc, id) in plan_shortcuts {
        let plan_id = id.clone();
        let res = gs.on_shortcut(sc.as_str(), move |app, _s, event| {
            if event.state() == ShortcutState::Pressed {
                apply_plan_by_id(app, &plan_id);
            }
        });
        if let Err(e) = res {
            eprintln!("注册方案快捷键 {sc} 失败: {e}");
        }
    }

    rebuild_tray_menu(&app);
    Ok(())
}

// ---- 应用构建 ----

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![HIDDEN_FLAG]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AppState::default())
        .setup(|app| {
            // 系统托盘：后台常驻（初始化失败不阻断应用启动）
            let build_result = (|| -> tauri::Result<()> {
                let show = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
                let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show, &quit])?;

                let Some(icon) = app.default_window_icon().cloned() else {
                    return Ok(()); // 无内置图标则跳过托盘，不影响主窗口
                };
                TrayIconBuilder::with_id(TRAY_ID)
                    .icon(icon)
                    .menu(&menu)
                    .tooltip("分辨率快速切换")
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| {
                        let id = event.id.as_ref().to_string();
                        if id == "show" {
                            if let Some(w) = app.get_webview_window("main") {
                                let _ = w.unminimize();
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        } else if id == "quit" {
                            app.exit(0);
                        } else if let Some(plan_id) = id.strip_prefix("plan:") {
                            let handle = app.clone();
                            let plan_id = plan_id.to_string();
                            tauri::async_runtime::spawn(async move {
                                apply_plan_by_id(&handle, &plan_id);
                            });
                        }
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let tauri::tray::TrayIconEvent::Click {
                            button: tauri::tray::MouseButton::Left,
                            button_state: tauri::tray::MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(w) = app.get_webview_window("main") {
                                let _ = w.unminimize();
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                    })
                    .build(app)?;
                Ok(())
            })();
            if let Err(e) = build_result {
                eprintln!("托盘初始化失败（不影响主窗口）: {e}");
            }

            // 不带 --hidden 即视为手动启动：显示主窗口。
            // 同时兼容旧版本注册的自启项（不带 --hidden）：已开启自启时重写一次，把参数补上，
            // 这样下次登录开机就能真正静默进托盘。
            if !started_hidden() {
                let al = app.autolaunch();
                if al.is_enabled().unwrap_or(false) {
                    if let Err(e) = al.enable() {
                        eprintln!("刷新开机自启参数失败: {e}");
                    }
                }
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                }
            }

            // 启动先挂上默认循环快捷键；前端加载完成后由 sync_plans 接管
            if let Err(e) = app.global_shortcut().register(DEFAULT_CYCLE_SHORTCUT) {
                eprintln!("注册默认循环快捷键失败: {e}");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_monitors,
            apply_mode,
            set_hdr,
            sync_plans,
            quit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
