# -*- coding: utf-8 -*-
"""生成 sidebar.bmp：两个叠放的显示器圆角矩形（与顶栏 logo / 应用图标同构图），
纯标准库 BMP（24bpp 位图，白底）。用法：gen_sidebar_bmp.py 输出路径 [宽=64] [高=宽]"""
import struct, math, sys

W = int(sys.argv[2]) if len(sys.argv) > 2 else 64
H = int(sys.argv[3]) if len(sys.argv) > 3 else W
ACCENT = (0, 103, 192)      # #0067c0 主题蓝
BG = (255, 255, 255)        # 白底

# 与 gen_app_icon.py 相同的几何：viewBox 24，rect1(2,4,14x10,rx1.5) 描边2，rect2(8,9,14x10,rx1.5) 实心
CX, CY = 12.0, 11.5         # logo 内容中心
SCALE = 0.9 * min(W / 20.0, H / 23.0)   # 内容占约 90% 画布，保持比例居中

def to_px(x, y):
    return (x - CX) * SCALE + W / 2.0, (y - CY) * SCALE + H / 2.0

def sdf_round_box(px, py, cx, cy, hx, hy, r):
    dx = abs(px - cx) - (hx - r)
    dy = abs(py - cy) - (hy - r)
    ax = max(dx, 0.0)
    ay = max(dy, 0.0)
    return math.hypot(ax, ay) + min(max(dx, dy), 0.0) - r

def cover(d, aa):
    v = 0.5 - d / aa
    return 0.0 if v <= 0 else (1.0 if v >= 1 else v)

r1c = to_px(9.0, 9.0); r1h = (7.0 * SCALE, 5.0 * SCALE); RR = 1.5 * SCALE
SW = 1.0 * SCALE
r2c = to_px(15.0, 14.0); r2h = (7.0 * SCALE, 5.0 * SCALE)
AA = max(0.7, 1.5 * SCALE / 47.0)

rows = bytearray()
for y in range(H - 1, -1, -1):      # BMP 自底向上
    py = y + 0.5
    row = bytearray()
    for x in range(W):
        px = x + 0.5
        d1 = abs(sdf_round_box(px, py, r1c[0], r1c[1], r1h[0], r1h[1], RR)) - SW
        a1 = cover(d1, AA)
        d2 = sdf_round_box(px, py, r2c[0], r2c[1], r2h[0], r2h[1], RR)
        a2 = cover(d2, AA)
        a1 *= (1.0 - a2)
        a = min(1.0, a1 + a2)
        r = int(BG[0] + (ACCENT[0] - BG[0]) * a)
        g = int(BG[1] + (ACCENT[1] - BG[1]) * a)
        b = int(BG[2] + (ACCENT[2] - BG[2]) * a)
        row += bytes((b, g, r))     # BGR
    pad = (4 - (len(row) % 4)) % 4
    rows += row + bytes(pad)

fsize = 54 + len(rows)
bmp = b"BM" + struct.pack("<IHHI", fsize, 0, 0, 54)
bmp += struct.pack("<IiiHHIIiiII", 40, W, H, 1, 24, 0, len(rows), 2835, 2835, 0, 0)
bmp += bytes(rows)

out = sys.argv[1]
with open(out, "wb") as f:
    f.write(bmp)
print("written", out, fsize, "bytes,", W, "x", H)
