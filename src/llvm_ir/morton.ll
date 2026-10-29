; This Source Code Form is subject to the terms of the Mozilla Public
; License, v. 2.0. If a copy of the MPL was not distributed with this
; file, You can obtain one at https://mozilla.org/MPL/2.0/.
;
; Copyright (c) 2026 c1ph3rc4t

define internal i64 @split_by_3(i21 %a21) alwaysinline {
    %a64 = zext i21 %a21 to i64
    %w0 = shl i64 %a64, 32
    %w1 = or i64 %a64, %w0
    %w2 = and i64 %w1, u0x001f00000000ffff
    %w3 = shl i64 %w2, 16
    %w4 = or i64 %w2, %w3
    %w5 = and i64 %w4, u0x001f0000ff0000ff
    %w6 = shl i64 %w5, 8
    %w7 = or i64 %w5, %w6
    %w8 = and i64 %w7, u0x100f00f00f00f00f
    %w9 = shl i64 %w8, 4
    %w10 = or i64 %w8, %w9
    %w11 = and i64 %w10, u0x10c30c30c30c30c3
    %w12 = shl i64 %w11, 2
    %w13 = or i64 %w11, %w12
    %r = and i64 %w13, u0x1249249249249249
    ret i64 %r
}

define i64 @morton_3d(i21 %x21, i21 %y21, i21 %z21) {
    %x64 = call i64 @split_by_3(i21 %x21)
    %y64 = call i64 @split_by_3(i21 %y21)
    %w0 = shl i64 %y64, 1
    %z64 = call i64 @split_by_3(i21 %z21)
    %w1 = shl i64 %z64, 2
    %w2 = or i64 %x64, %w0
    %r = or i64 %w2, %w1
    ret i64 %r
}
