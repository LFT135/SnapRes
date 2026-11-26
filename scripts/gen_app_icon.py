# -*- coding: utf-8 -*-
"""生成应用图标源图：两个叠放的显示器圆角矩形（与 App.vue 顶栏 logo 同构图）。
纯标准库实现：圆角矩形 SDF + 解析式抗锯齿，输出 RGBA PNG（透明背景）。"""
import struct, zlib, math, sys

SIZE = 1024
S = 47.0          # 24 viewBox 单位 -> 像素
CX, CY = 12.0, 11.5  # logo 整体包围盒中心，映射到画布中心
ACCENT = (0, 103, 192)  # #0067c0

def to_px(x, y):
    return (x - CX) * S + SIZE / 2.0, (y - CY) * S + SIZE / 2.0

def sdf_round_box(px, py, cx, cy, hx, hy, r):
    dx = abs(px - cx) - (hx - r)
    dy = abs(py - cy) - (hy - r)
    ax = max(dx, 0.0)
    ay = max(dy, 0.0)
    return math.hypot(ax, ay) + min(max(dx, dy), 0.0) - r

def cover_from_d(d, aa=1.0):
    """d<0 在形状内；返回 0..1 覆盖率"""
    v = 0.5 - d / aa
    return 0.0 if v <= 0 else (1.0 if v >= 1 else v)

# rect1: x=2..16, y=4..14, rx=1.5  -> 描边
r1c = to_px(9.0, 9.0); r1h = (7.0 * S, 5.0 * S); RR = 1.5 * S
SW = 1.0 * S  # stroke-width 2 -> 半宽 1 单位
# rect2: x=8..22, y=9..19, rx=1.5 -> 实心
r2c = to_px(15.0, 14.0); r2h = (7.0 * S, 5.0 * S)

rows = []
for y in range(SIZE):
    py = y + 0.5
    row = bytearray()
    row.append(0)  # filter: None
    for x in range(SIZE):
        px = x + 0.5
        # 后层描边
        d1 = abs(sdf_round_box(px, py, r1c[0], r1c[1], r1h[0], r1h[1], RR)) - SW
        a1 = cover_from_d(d1)
        # 前层实心
        d2 = sdf_round_box(px, py, r2c[0], r2c[1], r2h[0], r2h[1], RR)
        a2 = cover_from_d(d2)
        a1 *= (1.0 - a2)  # 前层盖住后层
        r, g, b = ACCENT
        row += bytes((
            r if a1 + a2 > 0 else 0,
            g if a1 + a2 > 0 else 0,
            b if a1 + a2 > 0 else 0,
            int(max(a1, a2) * 255),
        ))
    rows.append(bytes(row))

raw = b"".join(rows)

def chunk(tag, data):
    c = struct.pack(">I", len(data)) + tag + data
    return c + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)

png = b"\x89PNG\r\n\x1a\n"
png += chunk(b"IHDR", struct.pack(">IIBBBBB", SIZE, SIZE, 8, 6, 0, 0, 0))
png += chunk(b"IDAT", zlib.compress(raw, 9))
png += chunk(b"IEND", b"")

out = sys.argv[1] if len(sys.argv) > 1 else "app-icon-1024.png"
with open(out, "wb") as f:
    f.write(png)
print("written", out, len(png), "bytes")
