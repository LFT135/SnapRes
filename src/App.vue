<script setup>
import { ref, reactive, computed, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  enable as enableAutostart,
  disable as disableAutostart,
  isEnabled,
} from "@tauri-apps/plugin-autostart";

// ---------- 语言 ----------
const lang = ref(localStorage.getItem("lang") || "zh");
const I18N = {
  zh: {
    appName: "分辨率快速切换",
    autostart: "开机自启",
    dark: "深色模式",
    langBtn: "中文 / EN",
    plans: "我的方案",
    newPlan: "新建方案",
    noPlans: "还没有方案，点击右上角新建",
    monitor: "显示器",
    resolution: "分辨率",
    refresh: "刷新率",
    hdr: "HDR",
    bits: "色深",
    current: "当前",
    edit: "方案详情",
    planName: "方案名称",
    save: "保存",
    apply: "应用",
    saved: "已保存",
    applied: "已应用",
    delete: "删除",
    editPlan: "编辑",
    detailEmpty: "点击左侧方案进行查看与编辑",
    hdrOn: "开",
    hdrOff: "关",
    hdrUnsupported: "不支持",
    newPlanTitle: "新建方案",
    cancel: "取消",
    confirm: "确定",
    namePlaceholder: "输入方案名称",
    unsaved: "有未保存的修改",
    confirmDelete: "确定删除该方案？",
    bitsUnit: "bit",
    hzUnit: "Hz",
    needName: "请输入方案名称",
    notFound: "所选显示器已断开",
    shortcut: "快捷键",
    bind: "点击绑定",
    unbind: "清除",
    pressKey: "请按下组合键",
    needMod: "需包含 Ctrl / Alt / Shift / Win 之一",
    taken: "该快捷键已被占用",
    bindFail: "快捷键注册失败，可能被其他程序占用",
    cycleShortcut: "循环切换快捷键",
    trayTip: "右键托盘图标可直接应用方案",
    settings: "设置",
    appearance: "外观",
    startup: "启动",
    shortcutSection: "快捷键",
    windowSection: "窗口行为",
    darkDesc: "在浅色与深色主题之间切换",
    autostartDesc: "登录 Windows 后自动启动程序",
    autostartOn: "已开启开机自启",
    autostartOff: "已关闭开机自启",
    autostartBusy: "正在设置…",
    autostartOnInfo: "已开启 · 登录系统后将自动启动",
    autostartOffInfo: "已关闭 · 不会随系统启动",
    autostartFail: "开机自启设置失败",
    cycleDesc: "按顺序循环切换到下一个方案",
    shortcutDesc: "点击后按下组合键即可录制，需包含 Ctrl / Alt / Shift / Win",
    escHint: "Esc 取消",
    closePrompt: "关闭窗口时询问",
    closePromptDesc: "关闭时先询问是退出程序还是隐藏到托盘",
    closeActionWhenOff: "不询问时执行",
    closeTray: "隐藏到托盘",
    closeExit: "直接退出",
    closeTitle: "关闭程序？",
    closeText: "你想直接关闭程序，还是隐藏到托盘在后台继续运行？",
    remember: "记住我的选择，不再询问",
    rememberHint: "之后可在设置中重新开启询问",
    quitApp: "直接关闭",
    hideToTray: "隐藏到托盘",
    hiddenToTray: "已隐藏到托盘，可从托盘图标重新打开",
    back: "返回",
  },
  en: {
    appName: "Resolution Switcher",
    autostart: "Autostart",
    dark: "Dark Mode",
    langBtn: "EN / 中文",
    plans: "My Plans",
    newPlan: "New Plan",
    noPlans: "No plans yet. Click New Plan.",
    monitor: "Monitor",
    resolution: "Resolution",
    refresh: "Refresh",
    hdr: "HDR",
    bits: "Color depth",
    current: "Current",
    edit: "Plan Details",
    planName: "Plan name",
    save: "Save",
    apply: "Apply",
    saved: "Saved",
    applied: "Applied",
    delete: "Delete",
    editPlan: "Edit",
    detailEmpty: "Select a plan on the left to view & edit",
    hdrOn: "On",
    hdrOff: "Off",
    hdrUnsupported: "N/A",
    newPlanTitle: "New Plan",
    cancel: "Cancel",
    confirm: "OK",
    namePlaceholder: "Enter plan name",
    unsaved: "Unsaved changes",
    confirmDelete: "Delete this plan?",
    bitsUnit: "bit",
    hzUnit: "Hz",
    needName: "Plan name is required",
    notFound: "Selected monitor is disconnected",
    shortcut: "Shortcut",
    bind: "Click to bind",
    unbind: "Clear",
    pressKey: "Press a key combo",
    needMod: "Must include Ctrl / Alt / Shift / Win",
    taken: "Shortcut already in use",
    bindFail: "Failed to register shortcut, maybe taken by another app",
    cycleShortcut: "Cycle shortcut",
    trayTip: "Right-click the tray icon to apply a plan",
    settings: "Settings",
    appearance: "Appearance",
    startup: "Startup",
    shortcutSection: "Shortcuts",
    windowSection: "Window",
    darkDesc: "Switch between light and dark themes",
    autostartDesc: "Start the app after signing in to Windows",
    autostartOn: "Autostart enabled",
    autostartOff: "Autostart disabled",
    autostartBusy: "Applying…",
    autostartOnInfo: "On · launches automatically after sign-in",
    autostartOffInfo: "Off · will not launch with Windows",
    autostartFail: "Failed to change autostart",
    cycleDesc: "Cycle to the next plan in order",
    shortcutDesc: "Click, then press a combo; must include Ctrl / Alt / Shift / Win",
    escHint: "Esc to cancel",
    closePrompt: "Ask when closing",
    closePromptDesc: "Ask whether to quit or hide to the tray when closing",
    closeActionWhenOff: "When not asking",
    closeTray: "Hide to tray",
    closeExit: "Quit",
    closeTitle: "Close the app?",
    closeText: "Quit the app, or hide it to the tray and keep running in the background?",
    remember: "Remember my choice, don't ask again",
    rememberHint: "You can re-enable this prompt in Settings",
    quitApp: "Quit",
    hideToTray: "Hide to tray",
    hiddenToTray: "Hidden to tray · reopen from the tray icon",
    back: "Back",
  },
};
const t = computed(() => I18N[lang.value]);
function toggleLang() {
  lang.value = lang.value === "zh" ? "en" : "zh";
  localStorage.setItem("lang", lang.value);
}

// ---------- 深色模式 ----------
const dark = ref(localStorage.getItem("dark") !== "0");
function applyTheme() {
  document.documentElement.dataset.theme = dark.value ? "dark" : "light";
  localStorage.setItem("dark", dark.value ? "1" : "0");
}
watch(dark, applyTheme);

// ---------- 自启 ----------
const autostart = ref(false);
const autostartBusy = ref(false);
const autostartErrorMsg = ref("");
const autostartStatus = computed(() => {
  if (autostartBusy.value) return t.value.autostartBusy;
  if (autostartErrorMsg.value)
    return `${t.value.autostartFail}: ${autostartErrorMsg.value}`;
  return autostart.value ? t.value.autostartOnInfo : t.value.autostartOffInfo;
});
const autostartStatusKind = computed(() =>
  autostartBusy.value
    ? "pending"
    : autostartErrorMsg.value
      ? "error"
      : autostart.value
        ? "ok"
        : "off",
);
async function toggleAutostart(v) {
  if (autostartBusy.value) return;
  autostartBusy.value = true;
  autostartErrorMsg.value = "";
  try {
    if (v) await enableAutostart();
    else await disableAutostart();
    autostart.value = v;
    toast(v ? t.value.autostartOn : t.value.autostartOff);
  } catch (e) {
    console.error(e);
    autostart.value = await isEnabled().catch(() => false);
    autostartErrorMsg.value = String(e);
    toast(t.value.autostartFail);
  } finally {
    autostartBusy.value = false;
  }
}

// ---------- 显示器 ----------
const monitors = ref([]);
async function refreshMonitors() {
  try {
    monitors.value = await invoke("list_monitors");
  } catch (e) {
    console.error(e);
  }
}
const monitorOf = (sourceName) =>
  monitors.value.find((m) => m.source_name === sourceName);

// ---------- 分辨率 / 刷新率列表（拆分两个下拉） ----------
// 常用分辨率置顶，其余按像素面积升序
const PREFERRED = [
  [1920, 1080],
  [2560, 1440],
  [3840, 2160],
];
function priRank(r) {
  const i = PREFERRED.findIndex(([w, h]) => w === r.w && h === r.h);
  return i < 0 ? PREFERRED.length : i;
}
function resolutions(sourceName) {
  const mon = monitorOf(sourceName);
  if (!mon) return [];
  const map = new Map();
  for (const m of mon.modes) {
    const k = `${m.width}x${m.height}`;
    if (!map.has(k)) map.set(k, { w: m.width, h: m.height });
  }
  const arr = [...map.values()];
  arr.sort((a, b) => a.w * a.h - b.w * b.h); // 面积升序
  arr.sort((a, b) => priRank(a) - priRank(b)); // 常用置顶（稳定排序保留升序）
  return arr;
}
function refreshes(sourceName, res) {
  const mon = monitorOf(sourceName);
  if (!mon || !res) return [];
  const [w, h] = res.split("x").map(Number);
  const set = new Set();
  for (const m of mon.modes) {
    if (m.width === w && m.height === h && m.refresh > 0) set.add(m.refresh);
  }
  return [...set].sort((a, b) => a - b);
}
function bitsOf(sourceName, res, refresh) {
  const mon = monitorOf(sourceName);
  if (!mon || !res) return 0;
  const [w, h] = res.split("x").map(Number);
  const m =
    mon.modes.find(
      (x) => x.width === w && x.height === h && x.refresh === refresh,
    ) || mon.modes.find((x) => x.width === w && x.height === h);
  return m ? m.bits_per_pixel : 0;
}
function preferredRefresh(sourceName, res) {
  const list = refreshes(sourceName, res);
  if (!list.length) return 0;
  const mon = monitorOf(sourceName);
  if (
    mon &&
    res === `${mon.current.width}x${mon.current.height}` &&
    list.includes(mon.current.refresh)
  ) {
    return mon.current.refresh;
  }
  return list[list.length - 1]; // 该分辨率下最高刷新率
}

// ---------- 方案（localStorage 持久化 + 后端托管快捷键/托盘菜单） ----------
const plans = ref(JSON.parse(localStorage.getItem("plans") || "[]"));
for (const p of plans.value) if (p.shortcut === undefined) p.shortcut = "";
function persistPlans() {
  localStorage.setItem("plans", JSON.stringify(plans.value));
}
const cycleShortcut = ref(
  localStorage.getItem("cycleShortcut") || "Ctrl+Alt+R",
);
watch(cycleShortcut, (v) => localStorage.setItem("cycleShortcut", v || ""));

function planPayload(p) {
  return {
    id: p.id,
    name: p.name,
    source_name: p.sourceName,
    width: p.width,
    height: p.height,
    refresh: p.refresh || 0,
    hdr: !!p.hdr,
    bits: p.bits || 0,
    shortcut: p.shortcut || null,
  };
}
let syncTimer = null;
function scheduleSync() {
  clearTimeout(syncTimer);
  syncTimer = setTimeout(async () => {
    try {
      await invoke("sync_plans", {
        plans: plans.value.map(planPayload),
        cycleShortcut: cycleShortcut.value || "",
      });
    } catch (e) {
      console.error("sync_plans 失败", e);
    }
  }, 250);
}
watch(plans, scheduleSync, { deep: true });
watch(cycleShortcut, scheduleSync);

// ---------- 设置面板 ----------
const showSettings = ref(false);
function toggleSettings() {
  showSettings.value = !showSettings.value;
}
function selectPlan(p) {
  showSettings.value = false;
  selectedId.value = p.id;
}

// ---------- 关闭行为（退出 / 隐藏到托盘） ----------
const closePrompt = ref(localStorage.getItem("closePrompt") !== "0");
const closeAction = ref(localStorage.getItem("closeAction") || "tray");
watch(closePrompt, (v) => localStorage.setItem("closePrompt", v ? "1" : "0"));
watch(closeAction, (v) => localStorage.setItem("closeAction", v));

const showCloseDialog = ref(false);
const rememberClose = ref(false);
const appWindow = getCurrentWindow();

async function hideToTray() {
  showCloseDialog.value = false;
  try {
    await appWindow.hide();
    toast(t.value.hiddenToTray);
  } catch (e) {
    console.error(e);
  }
}

async function quitApp() {
  showCloseDialog.value = false;
  try {
    await invoke("quit_app");
  } catch (e) {
    console.error(e);
  }
}

function chooseClose(action) {
  if (rememberClose.value) {
    closePrompt.value = false;
    closeAction.value = action;
  }
  rememberClose.value = false;
  if (action === "tray") hideToTray();
  else quitApp();
}

async function onCloseRequested(event) {
  if (!closePrompt.value) {
    // 不询问：按设置直接执行，避免走 JS 默认的 window.destroy 分支
    event.preventDefault();
    if (closeAction.value === "tray") await hideToTray();
    else await quitApp();
    return;
  }
  event.preventDefault();
  rememberClose.value = false;
  showCloseDialog.value = true;
}

const selectedId = ref(plans.value[0]?.id ?? null);
const selected = computed(
  () => plans.value.find((p) => p.id === selectedId.value) || null,
);
const dirty = ref(false);
const detail = reactive({
  name: "",
  sourceName: "",
  res: "",
  refresh: 0,
  hdr: false,
  bits: 0,
  shortcut: "",
});

function detailSnapshot(d) {
  return JSON.stringify([
    d.name,
    d.sourceName,
    d.res,
    d.refresh,
    d.hdr,
    d.shortcut,
  ]);
}
let savedSnap = "";

function loadDetail(plan) {
  if (!plan) return;
  detail.name = plan.name;
  detail.sourceName = plan.sourceName;
  detail.res = `${plan.width}x${plan.height}`;
  detail.refresh = plan.refresh;
  detail.hdr = !!plan.hdr;
  detail.shortcut = plan.shortcut || "";
  const avail = resolutions(detail.sourceName);
  if (avail.length && !avail.some((r) => `${r.w}x${r.h}` === detail.res)) {
    const mon = monitorOf(detail.sourceName);
    detail.res = mon
      ? `${mon.current.width}x${mon.current.height}`
      : `${avail[0].w}x${avail[0].h}`;
  }
  const list = refreshes(detail.sourceName, detail.res);
  if (list.length && !list.includes(detail.refresh)) {
    detail.refresh = preferredRefresh(detail.sourceName, detail.res);
  }
  detail.bits = bitsOf(detail.sourceName, detail.res, detail.refresh);
  savedSnap = detailSnapshot(detail);
  dirty.value = false;
}
watch(selected, (p) => loadDetail(p), { immediate: true });
watch(monitors, () => {
  if (selected.value) loadDetail(selected.value);
});

function onMonitorChange() {
  const avail = resolutions(detail.sourceName);
  if (avail.length && !avail.some((r) => `${r.w}x${r.h}` === detail.res)) {
    const mon = monitorOf(detail.sourceName);
    detail.res = mon
      ? `${mon.current.width}x${mon.current.height}`
      : `${avail[0].w}x${avail[0].h}`;
  }
  onResChange();
}
function onResChange() {
  const list = refreshes(detail.sourceName, detail.res);
  if (!list.includes(detail.refresh))
    detail.refresh = preferredRefresh(detail.sourceName, detail.res);
  detail.bits = bitsOf(detail.sourceName, detail.res, detail.refresh);
}
function onRefreshChange() {
  detail.bits = bitsOf(detail.sourceName, detail.res, detail.refresh);
}

const resOfSelected = computed(() => resolutions(detail.sourceName));
const refreshOfSelected = computed(() =>
  refreshes(detail.sourceName, detail.res),
);
const currentRes = computed(() => {
  const mon = monitorOf(detail.sourceName);
  return mon ? `${mon.current.width}x${mon.current.height}` : "";
});
const realHdr = computed(() => {
  const mon = monitorOf(detail.sourceName);
  return mon ? mon.hdr_enabled : null;
});
const hdrSupportedOfSelected = computed(
  () => monitorOf(detail.sourceName)?.hdr_supported ?? false,
);

watch(
  () => detailSnapshot(detail),
  (v) => {
    dirty.value = v !== savedSnap;
  },
);

function saveDetail() {
  const p = selected.value;
  if (!p) return;
  if (!detail.name.trim()) {
    toast(t.value.needName);
    return;
  }
  const [w, h] = detail.res.split("x").map(Number);
  p.name = detail.name.trim();
  p.sourceName = detail.sourceName;
  p.width = w;
  p.height = h;
  p.refresh = detail.refresh;
  p.hdr = detail.hdr;
  p.bits = detail.bits;
  p.shortcut = detail.shortcut;
  persistPlans();
  savedSnap = detailSnapshot(detail);
  dirty.value = false;
  toast(t.value.saved);
}

async function applyPlan(plan) {
  if (!plan) return;
  const mon = monitorOf(plan.sourceName);
  if (!mon) {
    toast(t.value.notFound);
    await refreshMonitors();
    return;
  }
  try {
    await invoke("apply_mode", {
      sourceName: plan.sourceName,
      width: plan.width,
      height: plan.height,
      refresh: plan.refresh || 0,
    });
    await invoke("set_hdr", {
      sourceName: plan.sourceName,
      enable: !!plan.hdr,
    }).catch(() => {});
    await refreshMonitors();
    toast(t.value.applied);
  } catch (e) {
    toast(String(e));
  }
}

// ---------- 新建方案弹窗 ----------
const showDialog = ref(false);
const dlg = reactive({
  name: "",
  sourceName: "",
  res: "",
  refresh: 0,
  hdr: false,
  shortcut: "",
});

function openNewDialog() {
  showSettings.value = false;
  if (monitors.value.length === 0) refreshMonitors();
  const first = monitors.value[0];
  dlg.name = "";
  dlg.sourceName = first?.source_name || "";
  const avail = resolutions(dlg.sourceName);
  dlg.res = first
    ? `${first.current.width}x${first.current.height}`
    : avail[0]
      ? `${avail[0].w}x${avail[0].h}`
      : "";
  dlg.refresh = preferredRefresh(dlg.sourceName, dlg.res);
  dlg.hdr = first?.hdr_enabled || false;
  dlg.shortcut = "";
  showDialog.value = true;
}
const dlgMonitor = computed(() => monitorOf(dlg.sourceName));
const dlgResolutions = computed(() => resolutions(dlg.sourceName));
const dlgRefreshes = computed(() => refreshes(dlg.sourceName, dlg.res));
const dlgCurrentRes = computed(() => {
  const mon = dlgMonitor.value;
  return mon ? `${mon.current.width}x${mon.current.height}` : "";
});
watch(
  () => dlg.sourceName,
  () => {
    const mon = dlgMonitor.value;
    dlg.res = mon ? `${mon.current.width}x${mon.current.height}` : "";
    dlg.refresh = preferredRefresh(dlg.sourceName, dlg.res);
    dlg.hdr = mon?.hdr_enabled || false;
  },
);
function onDlgResChange() {
  const list = refreshes(dlg.sourceName, dlg.res);
  if (!list.includes(dlg.refresh))
    dlg.refresh = preferredRefresh(dlg.sourceName, dlg.res);
}

function confirmNewPlan() {
  if (!dlg.name.trim()) {
    toast(t.value.needName);
    return;
  }
  if (!dlg.res) return;
  const [w, h] = dlg.res.split("x").map(Number);
  const plan = {
    id: Date.now().toString(36) + Math.random().toString(36).slice(2, 6),
    name: dlg.name.trim(),
    sourceName: dlg.sourceName,
    width: w,
    height: h,
    refresh: dlg.refresh,
    hdr: dlg.hdr,
    bits: bitsOf(dlg.sourceName, dlg.res, dlg.refresh),
    shortcut: dlg.shortcut,
  };
  plans.value.push(plan);
  persistPlans();
  selectedId.value = plan.id;
  showDialog.value = false;
}

function deletePlan(id) {
  if (!confirm(t.value.confirmDelete)) return;
  plans.value = plans.value.filter((p) => p.id !== id);
  persistPlans();
  if (selectedId.value === id) selectedId.value = plans.value[0]?.id ?? null;
}

// ---------- 快捷键捕获（仅生成字符串，实际注册在后端） ----------
const capturing = ref(null); // "cycle" | "plan:{id}"
const capturePreview = ref(""); // 录制过程中实时回显已按下的键位
const MOD_KEYS = new Set(["Control", "Alt", "Shift", "Meta"]);

function keyToken(e) {
  const code = e.code || "";
  if (MOD_KEYS.has(e.key)) return null; // 纯修饰键，继续等待主键
  if (/^Key[A-Z]$/.test(code)) return code.slice(3);
  if (/^Digit[0-9]$/.test(code)) return code.slice(5);
  if (/^F([1-9]|1[0-9]|2[0-4])$/.test(code)) return code;
  const map = {
    Space: "Space",
    Minus: "-",
    Equal: "=",
    BracketLeft: "[",
    BracketRight: "]",
    Backslash: "\\",
    Semicolon: ";",
    Quote: "'",
    Comma: ",",
    Period: ".",
    Slash: "/",
    Backquote: "`",
    Enter: "Enter",
    Tab: "Tab",
    Escape: "Escape",
    Delete: "Delete",
    Insert: "Insert",
    Home: "Home",
    End: "End",
    PageUp: "PageUp",
    PageDown: "PageDown",
    ArrowLeft: "Left",
    ArrowUp: "Up",
    ArrowRight: "Right",
    ArrowDown: "Down",
    Numpad0: "Numpad0",
    Numpad1: "Numpad1",
    Numpad2: "Numpad2",
    Numpad3: "Numpad3",
    Numpad4: "Numpad4",
    Numpad5: "Numpad5",
    Numpad6: "Numpad6",
    Numpad7: "Numpad7",
    Numpad8: "Numpad8",
    Numpad9: "Numpad9",
  };
  return map[code] || null;
}

function comboTaken(combo, exceptTarget) {
  const c = combo.toLowerCase();
  if (
    exceptTarget !== "cycle" &&
    (cycleShortcut.value || "").toLowerCase() === c
  )
    return true;
  if (typeof exceptTarget === "string" && exceptTarget.startsWith("plan:")) {
    const id = exceptTarget.slice(5);
    return plans.value.some(
      (p) => p.id !== id && (p.shortcut || "").toLowerCase() === c,
    );
  }
  return plans.value.some((p) => (p.shortcut || "").toLowerCase() === c);
}

function assignCombo(target, combo) {
  if (comboTaken(combo, target)) {
    toast(t.value.taken);
    return false;
  }
  if (target === "cycle") {
    cycleShortcut.value = combo;
  } else if (target.startsWith("plan:")) {
    if (target === "plan:dlg") {
      dlg.shortcut = combo;
      return true;
    }
    detail.shortcut = combo;
  }
  return true;
}

function modsOf(e) {
  const mods = [];
  if (e.ctrlKey) mods.push("Ctrl");
  if (e.altKey) mods.push("Alt");
  if (e.shiftKey) mods.push("Shift");
  if (e.metaKey) mods.push("Win");
  return mods;
}

function onCaptureKey(e) {
  e.preventDefault();
  e.stopPropagation();
  if (e.key === "Escape") {
    stopCapture();
    return;
  }
  const mods = modsOf(e);
  const main = keyToken(e);
  if (main === null) {
    // 只按了修饰键：实时回显 Ctrl + Alt …
    capturePreview.value = mods.join(" + ");
    return;
  }
  const combo = [...mods, main].join(" + ");
  capturePreview.value = combo;
  if (mods.length === 0) {
    toast(t.value.needMod);
    setTimeout(() => {
      if (capturing.value && capturePreview.value === combo)
        capturePreview.value = "";
    }, 900);
    return;
  }
  if (assignCombo(capturing.value, [...mods, main].join("+"))) stopCapture();
}

function onCaptureKeyUp(e) {
  if (!capturing.value) return;
  const mods = modsOf(e);
  capturePreview.value = mods.length ? mods.join(" + ") : "";
}

function startCapture(target) {
  stopCapture();
  capturing.value = target;
  capturePreview.value = "";
  document.addEventListener("keydown", onCaptureKey, true);
  document.addEventListener("keyup", onCaptureKeyUp, true);
}
function stopCapture() {
  if (capturing.value) {
    document.removeEventListener("keydown", onCaptureKey, true);
    document.removeEventListener("keyup", onCaptureKeyUp, true);
  }
  capturing.value = null;
  capturePreview.value = "";
}
function clearShortcut(target) {
  if (target === "cycle") cycleShortcut.value = "";
  else if (target === "plan:dlg") dlg.shortcut = "";
  else detail.shortcut = "";
}

// ---------- 循环切换由后端直接完成，前端只接收结果事件 ----------

// ---------- Toast ----------
const toastMsg = ref("");
let toastTimer = null;
function toast(msg) {
  toastMsg.value = msg;
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => (toastMsg.value = ""), 2200);
}

const unlisteners = [];
onMounted(async () => {
  applyTheme();
  autostart.value = await isEnabled().catch(() => false);
  await refreshMonitors();
  await scheduleSyncNow();
  try {
    unlisteners.push(await appWindow.onCloseRequested(onCloseRequested));
  } catch (e) {
    console.error("注册关闭拦截失败", e);
  }
  unlisteners.push(
    await listen("plan-applied", async (ev) => {
      const p = ev.payload || {};
      toast(p.ok ? `${t.value.applied} · ${p.msg}` : String(p.msg));
      if (p.planId) selectedId.value = p.planId;
      if (p.ok) await refreshMonitors();
    }),
  );
});
async function scheduleSyncNow() {
  clearTimeout(syncTimer);
  try {
    await invoke("sync_plans", {
      plans: plans.value.map(planPayload),
      cycleShortcut: cycleShortcut.value || "",
    });
  } catch (e) {
    console.error("sync_plans 失败", e);
  }
}
onUnmounted(() => {
  unlisteners.forEach((f) => f?.());
  stopCapture();
});

function planSubtitle(p) {
  const mon = monitorOf(p.sourceName);
  return `${p.width}x${p.height} @ ${p.refresh}${t.value.hzUnit}${mon ? " · " + mon.friendly_name : ""}`;
}
</script>

<template>
  <div class="app">
    <!-- 顶栏 -->
    <header class="topbar" data-tauri-drag-region>
      <div class="logo">
        <svg
          viewBox="0 0 24 24"
          width="26"
          height="26"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <rect x="2" y="4" width="14" height="10" rx="1.5" />
          <rect
            x="8"
            y="9"
            width="14"
            height="10"
            rx="1.5"
            fill="var(--accent)"
            stroke="var(--accent)"
          />
        </svg>
        <span>{{ t.appName }}</span>
      </div>
      <div class="topbar-right">
        <button class="lang-btn" @click="toggleLang">{{ t.langBtn }}</button>
        <button
          class="icon-btn"
          :class="{ active: showSettings }"
          :title="t.settings"
          :aria-label="t.settings"
          @click="toggleSettings"
        >
          <svg
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            stroke-width="1.8"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <circle cx="12" cy="12" r="3.1" />
            <path
              d="M19.4 15a1.7 1.7 0 0 0 .34 1.87l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.7 1.7 0 0 0-1.87-.34 1.7 1.7 0 0 0-1 1.55V21a2 2 0 1 1-4 0v-.09A1.7 1.7 0 0 0 8.9 19.3a1.7 1.7 0 0 0-1.87.34l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.7 1.7 0 0 0 .34-1.87 1.7 1.7 0 0 0-1.55-1H3a2 2 0 1 1 0-4h.09A1.7 1.7 0 0 0 4.7 8.9a1.7 1.7 0 0 0-.34-1.87l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.7 1.7 0 0 0 1.87.34H9a1.7 1.7 0 0 0 1-1.55V3a2 2 0 1 1 4 0v.09a1.7 1.7 0 0 0 1 1.55 1.7 1.7 0 0 0 1.87-.34l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.7 1.7 0 0 0-.34 1.87V9a1.7 1.7 0 0 0 1.55 1H21a2 2 0 1 1 0 4h-.09a1.7 1.7 0 0 0-1.55 1z"
            />
          </svg>
        </button>
      </div>
    </header>

    <div class="body">
      <!-- 左侧：方案列表 -->
      <aside class="sidebar">
        <div class="sidebar-head">
          <span class="section-title">{{ t.plans }}</span>
          <button class="btn primary sm" @click="openNewDialog">
            + {{ t.newPlan }}
          </button>
        </div>
        <div class="plan-list">
          <div
            v-for="p in plans"
            :key="p.id"
            class="plan-card"
            :class="{ active: p.id === selectedId }"
            @click="selectPlan(p)"
          >
            <div class="plan-card-top">
              <span class="plan-name">{{ p.name }}</span>
              <span class="hdr-badge" :class="{ on: p.hdr }">HDR</span>
            </div>
            <div class="plan-sub">{{ planSubtitle(p) }}</div>
            <div class="plan-actions">
              <button class="btn ghost xs" @click.stop="applyPlan(p)">
                {{ t.apply }}
              </button>
              <button
                class="btn ghost xs danger"
                @click.stop="deletePlan(p.id)"
              >
                {{ t.delete }}
              </button>
            </div>
          </div>
          <div v-if="plans.length === 0" class="empty">{{ t.noPlans }}</div>
        </div>
        <div class="shortcut-tip">{{ t.trayTip }}</div>
      </aside>

      <!-- 右侧：设置面板 -->
      <main v-if="showSettings" class="detail settings-pane">
        <div class="detail-head">
          <span class="section-title">{{ t.settings }}</span>
          <span class="spacer"></span>
          <button class="btn ghost sm" @click="showSettings = false">
            {{ t.back }}
          </button>
        </div>

        <section class="settings-group">
          <div class="settings-group-title">{{ t.appearance }}</div>
          <div class="settings-row">
            <div class="settings-text">
              <div class="settings-name">{{ t.dark }}</div>
              <div class="settings-desc">{{ t.darkDesc }}</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="dark" />
              <span class="track"><span class="thumb"></span></span>
            </label>
          </div>
        </section>

        <section class="settings-group">
          <div class="settings-group-title">{{ t.startup }}</div>
          <div class="settings-row">
            <div class="settings-text">
              <div class="settings-name">{{ t.autostart }}</div>
              <div class="settings-desc">{{ t.autostartDesc }}</div>
            </div>
            <label class="switch">
              <input
                type="checkbox"
                :checked="autostart"
                :disabled="autostartBusy"
                @change="toggleAutostart($event.target.checked)"
              />
              <span class="track"><span class="thumb"></span></span>
            </label>
          </div>
          <div class="settings-status" :class="autostartStatusKind">
            <span class="status-dot"></span>
            <span>{{ autostartStatus }}</span>
          </div>
        </section>

        <section class="settings-group">
          <div class="settings-group-title">{{ t.shortcutSection }}</div>
          <div class="settings-row">
            <div class="settings-text">
              <div class="settings-name">{{ t.cycleShortcut }}</div>
              <div class="settings-desc">{{ t.cycleDesc }}</div>
            </div>
            <div class="settings-inline">
              <button
                class="kbd-btn"
                :class="{ capturing: capturing === 'cycle' }"
                :title="capturing === 'cycle' ? t.pressKey : t.bind"
                @click="
                  capturing === 'cycle'
                    ? stopCapture()
                    : startCapture('cycle')
                "
              >
                {{
                  capturing === "cycle"
                    ? capturePreview || t.pressKey
                    : cycleShortcut || t.bind
                }}
              </button>
              <button
                v-if="cycleShortcut"
                class="btn ghost xs danger"
                @click="clearShortcut('cycle')"
              >
                {{ t.unbind }}
              </button>
            </div>
          </div>
          <div class="settings-hint">
            {{ t.shortcutDesc
            }}<template v-if="capturing"> · {{ t.escHint }}</template>
          </div>
        </section>

        <section class="settings-group">
          <div class="settings-group-title">{{ t.windowSection }}</div>
          <div class="settings-row">
            <div class="settings-text">
              <div class="settings-name">{{ t.closePrompt }}</div>
              <div class="settings-desc">{{ t.closePromptDesc }}</div>
            </div>
            <label class="switch">
              <input type="checkbox" v-model="closePrompt" />
              <span class="track"><span class="thumb"></span></span>
            </label>
          </div>
          <div class="settings-row" v-if="!closePrompt">
            <div class="settings-text">
              <div class="settings-name">{{ t.closeActionWhenOff }}</div>
            </div>
            <div class="seg">
              <button
                class="seg-item"
                :class="{ active: closeAction === 'tray' }"
                @click="closeAction = 'tray'"
              >
                {{ t.closeTray }}
              </button>
              <button
                class="seg-item"
                :class="{ active: closeAction === 'exit' }"
                @click="closeAction = 'exit'"
              >
                {{ t.closeExit }}
              </button>
            </div>
          </div>
        </section>
      </main>

      <main v-else class="detail">
        <template v-if="selected">
          <div class="detail-head">
            <span class="section-title">{{ t.edit }}</span>
            <span v-if="dirty" class="dirty-dot" :title="t.unsaved"
              >● {{ t.unsaved }}</span
            >
          </div>

          <div class="field">
            <label>{{ t.planName }}</label>
            <input v-model="detail.name" class="input" />
          </div>

          <div class="field">
            <label>{{ t.monitor }}</label>
            <select
              v-model="detail.sourceName"
              class="input"
              @change="onMonitorChange"
            >
              <option
                v-for="m in monitors"
                :key="m.source_name"
                :value="m.source_name"
              >
                {{ m.friendly_name }}
              </option>
            </select>
          </div>

          <div class="grid2">
            <div class="field">
              <label>{{ t.resolution }}</label>
              <select v-model="detail.res" class="input" @change="onResChange">
                <option
                  v-for="r in resOfSelected"
                  :key="r.w + 'x' + r.h"
                  :value="r.w + 'x' + r.h"
                >
                  {{ r.w }} × {{ r.h
                  }}<template v-if="currentRes === r.w + 'x' + r.h">
                    · {{ t.current }}</template
                  >
                </option>
              </select>
            </div>
            <div class="field">
              <label>{{ t.refresh }}</label>
              <select
                v-model.number="detail.refresh"
                class="input"
                @change="onRefreshChange"
              >
                <option v-for="r in refreshOfSelected" :key="r" :value="r">
                  {{ r }} {{ t.hzUnit }}
                </option>
              </select>
            </div>
          </div>

          <div class="grid2">
            <div class="field inline">
              <label>{{ t.hdr }}</label>
              <label class="switch">
                <input
                  type="checkbox"
                  v-model="detail.hdr"
                  :disabled="!hdrSupportedOfSelected"
                />
                <span class="track"><span class="thumb"></span></span>
                <span class="switch-label dim">
                  {{
                    hdrSupportedOfSelected
                      ? detail.hdr
                        ? t.hdrOn
                        : t.hdrOff
                      : t.hdrUnsupported
                  }}
                </span>
              </label>
              <span class="hdr-state" v-if="realHdr !== null">
                {{ t.current }}: {{ realHdr ? t.hdrOn : t.hdrOff }}
              </span>
            </div>
            <div class="field inline">
              <label>{{ t.bits }}</label>
              <span class="static-value"
                >{{ detail.bits }} {{ t.bitsUnit }}</span
              >
            </div>
          </div>

          <div class="grid2">
            <div class="field">
              <label>{{ t.shortcut }}</label>
              <div class="kbd-row">
                <button
                  class="kbd-btn"
                  :class="{ capturing: capturing === 'plan:' + selectedId }"
                  @click="
                    capturing === 'plan:' + selectedId
                      ? stopCapture()
                      : startCapture('plan:' + selectedId)
                  "
                >
                  {{
                    capturing === "plan:" + selectedId
                      ? capturePreview || t.pressKey
                      : detail.shortcut || t.bind
                  }}
                </button>
                <button
                  v-if="detail.shortcut"
                  class="btn ghost xs danger"
                  @click="clearShortcut('plan')"
                >
                  {{ t.unbind }}
                </button>
              </div>
            </div>
          </div>

          <div class="detail-actions">
            <button class="btn ghost" @click="deletePlan(selected.id)">
              {{ t.delete }}
            </button>
            <span class="spacer"></span>
            <button v-if="dirty" class="btn ghost" @click="loadDetail(selected)">
              {{ t.cancel }}
            </button>
            <button class="btn" @click="saveDetail">{{ t.save }}</button>
            <button class="btn primary" @click="applyPlan(selected)">
              {{ t.apply }}
            </button>
          </div>
        </template>
        <div v-else class="empty detail-empty">{{ t.detailEmpty }}</div>
      </main>
    </div>

    <!-- 新建方案弹窗 -->
    <div class="modal-mask" v-if="showDialog" @click.self="showDialog = false">
      <div class="modal">
        <h3>{{ t.newPlanTitle }}</h3>
        <div class="field">
          <label>{{ t.planName }}</label>
          <input
            v-model="dlg.name"
            class="input"
            :placeholder="t.namePlaceholder"
            @keyup.enter="confirmNewPlan"
          />
        </div>
        <div class="field">
          <label>{{ t.monitor }}</label>
          <select v-model="dlg.sourceName" class="input">
            <option
              v-for="m in monitors"
              :key="m.source_name"
              :value="m.source_name"
            >
              {{ m.friendly_name }}
            </option>
          </select>
        </div>
        <div class="grid2">
          <div class="field">
            <label>{{ t.resolution }}</label>
            <select v-model="dlg.res" class="input" @change="onDlgResChange">
              <option
                v-for="r in dlgResolutions"
                :key="r.w + 'x' + r.h"
                :value="r.w + 'x' + r.h"
              >
                {{ r.w }} × {{ r.h
                }}<template v-if="dlgCurrentRes === r.w + 'x' + r.h">
                  · {{ t.current }}</template
                >
              </option>
            </select>
          </div>
          <div class="field">
            <label>{{ t.refresh }}</label>
            <select v-model.number="dlg.refresh" class="input">
              <option v-for="r in dlgRefreshes" :key="r" :value="r">
                {{ r }} {{ t.hzUnit }}
              </option>
            </select>
          </div>
        </div>

        <div class="field">
          <label>{{ t.hdr }}</label>
          <label class="switch">
            <input
              type="checkbox"
              v-model="dlg.hdr"
              :disabled="!hdrSupportedOfSelected"
            />
            <span class="track"><span class="thumb"></span></span>
            <span class="switch-label dim">
              {{
                hdrSupportedOfSelected
                  ? dlg.hdr
                    ? t.hdrOn
                    : t.hdrOff
                  : t.hdrUnsupported
              }}
            </span>
          </label>
        </div>

        <div class="field">
          <label>{{ t.shortcut }}</label>
          <div class="kbd-row">
            <button
              class="kbd-btn"
              :class="{ capturing: capturing === 'plan:dlg' }"
              @click="
                capturing === 'plan:dlg'
                  ? stopCapture()
                  : startCapture('plan:dlg')
              "
            >
              {{
                capturing === "plan:dlg"
                  ? capturePreview || t.pressKey
                  : dlg.shortcut || t.bind
              }}
            </button>
            <button
              v-if="dlg.shortcut"
              class="btn ghost xs danger"
              @click="clearShortcut('plan:dlg')"
            >
              {{ t.unbind }}
            </button>
          </div>
        </div>
        <div class="modal-actions">
          <button class="btn ghost" @click="showDialog = false">
            {{ t.cancel }}
          </button>
          <button class="btn primary" @click="confirmNewPlan">
            {{ t.confirm }}
          </button>
        </div>
      </div>
    </div>

    <!-- 关闭确认弹窗 -->
    <div class="modal-mask" v-if="showCloseDialog">
      <div class="modal close-modal">
        <h3>{{ t.closeTitle }}</h3>
        <p class="modal-text">{{ t.closeText }}</p>
        <label class="check-row">
          <input type="checkbox" v-model="rememberClose" />
          <span>{{ t.remember }}</span>
        </label>
        <div class="modal-hint">{{ t.rememberHint }}</div>
        <div class="modal-actions">
          <button class="btn ghost" @click="chooseClose('exit')">
            {{ t.quitApp }}
          </button>
          <button class="btn primary" @click="chooseClose('tray')">
            {{ t.hideToTray }}
          </button>
        </div>
      </div>
    </div>

    <transition name="toast">
      <div class="toast" v-if="toastMsg">{{ toastMsg }}</div>
    </transition>
  </div>
</template>

<style>
/* ============ Fluent 风格基础 ============ */
:root {
  --bg: #f3f3f3;
  --card: #ffffff;
  --card-hover: #f9f9f9;
  --border: #e5e5e5;
  --border-strong: #d1d1d1;
  --text: #1b1b1b;
  --text-2: #616161;
  --accent: #0067c0;
  --accent-hover: #1975c5;
  --accent-text: #ffffff;
  --danger: #c42b1c;
  --shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  --radius: 8px;
  --font:
    "Segoe UI Variable Text", "Segoe UI", "Microsoft YaHei UI", system-ui,
    sans-serif;
}
:root[data-theme="dark"] {
  --bg: #202020;
  --card: #2c2c2c;
  --card-hover: #333333;
  --border: #3d3d3d;
  --border-strong: #4a4a4a;
  --text: #ffffff;
  --text-2: #a0a0a0;
  --accent: #4cc2ff;
  --accent-hover: #68cdff;
  --accent-text: #00131f;
  --danger: #ff6b6b;
  --shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}
html,
body,
#app {
  height: 100%;
}
body {
  font-family: var(--font);
  font-size: 14px;
  color: var(--text);
  background: var(--bg);
  overflow: hidden;
  -webkit-font-smoothing: antialiased;
}

.app {
  display: flex;
  flex-direction: column;
  height: 100%;
}

/* ============ 顶栏 ============ */
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 52px;
  padding: 0 16px;
  background: var(--card);
  border-bottom: 1px solid var(--border);
  -webkit-user-select: none;
  user-select: none;
  flex-shrink: 0;
}
.logo {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 600;
  font-size: 15px;
}
.logo svg {
  color: var(--accent);
}
.topbar-right {
  display: flex;
  align-items: center;
  gap: 18px;
}

/* 开关 */
.switch {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}
.switch input {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}
.switch .track {
  width: 40px;
  height: 20px;
  border-radius: 999px;
  background: transparent;
  border: 1px solid var(--text-2);
  position: relative;
  transition:
    background 0.15s,
    border-color 0.15s;
  flex-shrink: 0;
}
.switch .thumb {
  position: absolute;
  left: 3px;
  top: 50%;
  transform: translateY(-50%);
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--text-2);
  transition: all 0.15s;
}
.switch input:checked + .track {
  background: var(--accent);
  border-color: var(--accent);
}
.switch input:checked + .track .thumb {
  left: 23px;
  width: 14px;
  height: 14px;
  background: var(--accent-text);
}
.switch input:disabled + .track {
  opacity: 0.4;
}
.switch-label {
  font-size: 13px;
  color: var(--text);
}
.switch-label.dim {
  color: var(--text-2);
}

.lang-btn {
  border: 1px solid var(--border-strong);
  background: var(--card);
  color: var(--text);
  border-radius: 4px;
  padding: 5px 10px;
  font-size: 12px;
  cursor: pointer;
  font-family: var(--font);
}
.lang-btn:hover {
  background: var(--card-hover);
}

.icon-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 4px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--text);
  cursor: pointer;
  transition:
    background 0.12s,
    color 0.12s,
    border-color 0.12s;
}
.icon-btn:hover {
  background: var(--card-hover);
  border-color: var(--border);
}
.icon-btn.active {
  color: var(--accent);
  border-color: var(--accent);
}

/* ============ 主体布局 ============ */
.body {
  display: flex;
  flex: 1;
  min-height: 0;
}

/* ============ 左侧 ============ */
.sidebar {
  width: 320px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border);
  background: var(--card);
  padding: 14px;
  gap: 10px;
  min-height: 0;
}
.sidebar-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.section-title {
  font-size: 15px;
  font-weight: 600;
}
.plan-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-right: 2px;
}
.plan-card {
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--card);
  padding: 12px 14px;
  cursor: pointer;
  transition:
    border-color 0.15s,
    background 0.15s;
  box-shadow: var(--shadow);
}
.plan-card:hover {
  background: var(--card-hover);
}
.plan-card.active {
  border-color: var(--accent);
  border-width: 2px;
  padding: 11px 13px;
}
.plan-card-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}
.plan-name {
  font-weight: 600;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.hdr-badge {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  background: var(--border);
  color: var(--text-2);
  flex-shrink: 0;
}
.hdr-badge.on {
  background: var(--accent);
  color: var(--accent-text);
}
.plan-sub {
  font-size: 12px;
  color: var(--text-2);
  margin-top: 4px;
}
.plan-actions {
  display: flex;
  gap: 6px;
  margin-top: 10px;
}
.shortcut-tip {
  font-size: 11px;
  color: var(--text-2);
  text-align: center;
  padding: 4px 0;
  flex-shrink: 0;
}

/* ============ 设置面板 ============ */
.settings-pane {
  gap: 14px;
}
.settings-group {
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--card);
  box-shadow: var(--shadow);
  overflow: hidden;
  flex-shrink: 0;
}
.settings-group-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-2);
  padding: 12px 16px 8px;
  letter-spacing: 0.3px;
}
.settings-row {
  display: flex;
  align-items: center;
  gap: 18px;
  padding: 10px 16px;
  border-top: 1px solid var(--border);
}
.settings-group-title + .settings-row {
  border-top: none;
}
.settings-text {
  flex: 1;
  min-width: 0;
}
.settings-name {
  font-size: 14px;
}
.settings-desc {
  font-size: 12px;
  color: var(--text-2);
  margin-top: 3px;
}
.settings-inline {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}
.settings-hint {
  font-size: 12px;
  color: var(--text-2);
  padding: 0 16px 12px;
}
.settings-status {
  display: flex;
  align-items: center;
  gap: 7px;
  font-size: 12px;
  color: var(--text-2);
  padding: 0 16px 12px;
}
.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: currentColor;
  flex-shrink: 0;
}
.settings-status.ok {
  color: #2ea043;
}
.settings-status.error {
  color: var(--danger);
}
.settings-status.pending {
  color: #d29922;
}

/* 分段选择 */
.seg {
  display: inline-flex;
  border: 1px solid var(--border-strong);
  border-radius: 4px;
  overflow: hidden;
  flex-shrink: 0;
}
.seg-item {
  font-family: var(--font);
  font-size: 12px;
  padding: 6px 12px;
  border: none;
  background: var(--card);
  color: var(--text-2);
  cursor: pointer;
}
.seg-item + .seg-item {
  border-left: 1px solid var(--border-strong);
}
.seg-item:hover {
  background: var(--card-hover);
}
.seg-item.active {
  background: var(--accent);
  color: var(--accent-text);
}
.kbd-row {
  display: flex;
  align-items: center;
  gap: 8px;
}
.kbd-btn {
  font-family: "Consolas", "Cascadia Mono", var(--font);
  font-size: 12px;
  color: var(--text);
  background: var(--card);
  border: 1px solid var(--border-strong);
  border-bottom-width: 2px;
  border-radius: 4px;
  padding: 6px 10px;
  cursor: pointer;
  min-width: 96px;
  text-align: center;
  transition:
    border-color 0.12s,
    background 0.12s;
}
.kbd-btn:hover {
  background: var(--card-hover);
}
.kbd-btn.capturing {
  border-color: var(--accent);
  color: var(--accent);
}

/* ============ 右侧详情 ============ */
.detail {
  flex: 1;
  min-width: 0;
  padding: 22px 28px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 18px;
}
.detail-head {
  display: flex;
  align-items: center;
  gap: 12px;
}
.dirty-dot {
  font-size: 12px;
  color: #e8a33d;
}
.detail-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}
.grid2 {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 18px;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.field.inline {
  flex-direction: row;
  align-items: center;
  gap: 14px;
}
.field label {
  font-size: 13px;
  color: var(--text-2);
}
.static-value {
  font-size: 14px;
}
.hdr-state {
  font-size: 12px;
  color: var(--text-2);
}

.input {
  font-family: var(--font);
  font-size: 14px;
  color: var(--text);
  background: var(--card);
  border: 1px solid var(--border-strong);
  border-bottom-width: 2px;
  border-radius: 4px;
  padding: 7px 10px;
  outline: none;
  transition: border-color 0.12s;
  width: 100%;
}
.input:focus {
  border-color: var(--accent);
}
select.input {
  cursor: pointer;
}

.detail-actions {
  display: flex;
  gap: 10px;
  align-items: center;
  margin-top: auto;
  padding-top: 18px;
}
.spacer {
  flex: 1;
}

/* ============ 按钮 ============ */
.btn {
  font-family: var(--font);
  font-size: 14px;
  padding: 7px 18px;
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid var(--border-strong);
  background: var(--card);
  color: var(--text);
  transition:
    background 0.12s,
    border-color 0.12s;
}
.btn:hover {
  background: var(--card-hover);
}
.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: var(--accent-text);
}
.btn.primary:hover {
  background: var(--accent-hover);
  border-color: var(--accent-hover);
}
.btn.ghost {
  border-color: transparent;
  background: transparent;
}
.btn.ghost:hover {
  background: var(--card-hover);
}
.btn.danger {
  color: var(--danger);
}
.btn.sm {
  padding: 5px 12px;
  font-size: 13px;
}
.btn.xs {
  padding: 3px 10px;
  font-size: 12px;
}

/* ============ 空态 ============ */
.empty {
  color: var(--text-2);
  font-size: 13px;
  text-align: center;
  padding: 30px 10px;
}

/* ============ 弹窗 ============ */
.modal-mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.45);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 50;
}
.modal {
  width: 460px;
  max-width: calc(100vw - 40px);
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: 10px;
  padding: 22px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
}
.modal h3 {
  font-size: 16px;
}
.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}
.close-modal {
  width: 420px;
}
.modal-text {
  font-size: 13px;
  color: var(--text-2);
  line-height: 1.6;
  margin-top: -4px;
}
.modal-hint {
  font-size: 11px;
  color: var(--text-2);
  margin-top: -8px;
}
.check-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  cursor: pointer;
  user-select: none;
}
.check-row input {
  width: 15px;
  height: 15px;
  accent-color: var(--accent);
  cursor: pointer;
}

/* ============ Toast ============ */
.toast {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--text);
  color: var(--bg);
  padding: 8px 18px;
  border-radius: 6px;
  font-size: 13px;
  z-index: 60;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);
}
.toast-enter-active,
.toast-leave-active {
  transition:
    opacity 0.2s,
    transform 0.2s;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateX(-50%) translateY(8px);
}

/* 滚动条 */
::-webkit-scrollbar {
  width: 8px;
}
::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 4px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
</style>
