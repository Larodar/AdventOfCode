const std = @import("std");
const io = std.io;

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();
    const problem_str = args.next().?;
    const problem = try std.fmt.parseInt(u8, problem_str, 10);

    const stdin_file = io.getStdIn().reader();
    var br = std.io.bufferedReader(stdin_file);
    const stdin = br.reader();

    var total: usize = undefined;
    if (problem == 1) {
        total = try p1(&stdin);
    } else if (problem == 2) {
        total = try p2(&stdin);
    }

    std.debug.print("result: {?}\n", .{total});
}

fn p1(reader: anytype) !usize {
    const allocator = std.heap.page_allocator;
    var buf = [_]u8{0} ** 4096;
    var total: usize = 0;
    var layout = std.ArrayList(u8).init(allocator);
    defer layout.deinit();
    while (true) {
        const size = try reader.read(buf[0..]);
        if (size == 0) {
            break;
        }
        const l = buf[0..size];
        for (l) |value| {
            if (value < 0x30) {
                continue;
            }
            try layout.append(value - 0x30);
        }
    }

    var files_rev = if (layout.items.len & 1 > 0) layout.items.len - 1 else layout.items.len - 2;
    //std.debug.print("len: {d}; files_rev: {d}\n", .{layout.items.len, files_rev});
    var fs_idx: usize = 0;
    var idx: usize = 0;
    while (idx < files_rev) {
        const len = layout.items[idx];
        //std.debug.print("idx: {d}; files_rev: {d}; fs_idx: {d}\n", .{ idx, files_rev, fs_idx });
        if (idx & 1 == 0) {
            if (len > 0) {
                // std.debug.print("writing file\n", .{});
                total += calc_checksum(Position{ .len = @truncate(len), .id = @truncate(idx / 2) }, &fs_idx);
            }
        } else {
            // std.debug.print("filling gap\n", .{});
            total += fill_gap(idx, &files_rev, &fs_idx, len, layout.items);
        }
        idx += 1;
    }

    //std.debug.print("layout.items[{d}] = {d}\n", .{idx, layout.items[idx]});
    if (layout.items[idx] > 0) {
        const pos = Position{ .len = @truncate(layout.items[idx]), .id = @truncate(idx / 2) };
        total += calc_checksum(pos, &fs_idx);
    }

    return total;
}

fn fill_gap(left_ptr: usize, right_ptr: *usize, file_id: *usize, empty_blocks: usize, items: []u8) usize {
    var ret: usize = 0;
    var empty = empty_blocks;
    // std.debug.print("l_ptr: {d}; r_ptr: {d}, file_id: {d}; empty_blocks: {d}\n", .{ left_ptr, right_ptr.*, file_id.*, empty_blocks });
    while (right_ptr.* > left_ptr and right_ptr.* > 1) {
        const pos = items[right_ptr.*];
        if (empty < pos) {
            const partial = Position{ .len = @truncate(empty), .id = @truncate(right_ptr.* / 2) };
            ret += calc_checksum(partial, file_id);
            items[right_ptr.*] -= @truncate(empty);
            break;
        }

        if (pos == 0) {
            right_ptr.* -= 2;
            continue;
        }

        const new_blocks = Position{ .len = @truncate(pos), .id = @truncate(right_ptr.* / 2) };
        ret += calc_checksum(new_blocks, file_id);
        items[right_ptr.*] = 0;
        right_ptr.* -= 2;
        empty -= pos;
    }

    return ret;
}

fn calc_checksum(block: Position, idx: *usize) usize {
    var ret: usize = 0;
    const start = idx.*;
    idx.* += block.len;
    for (start..idx.*) |value| {
        //std.debug.print("{d} * {d}\n", .{ value, block.id });
        ret += value * block.id;
    }

    // std.debug.print("Adding {d} to total from {d} blocks of {d}.\n", .{ ret, block.len, block.id });
    return ret;
}

const Position = packed struct(u32) {
    len: u4,
    id: u28,
};

fn p2(reader: anytype) !usize {
    const allocator = std.heap.page_allocator;
    var buf = [_]u8{0} ** 4096;
    var total: usize = 0;
    var layout = std.ArrayList(u8).init(allocator);
    defer layout.deinit();
    while (true) {
        const size = try reader.read(buf[0..]);
        if (size == 0) {
            break;
        }
        const l = buf[0..size];
        for (l) |value| {
            if (value < 0x30) {
                continue;
            }
            try layout.append(value - 0x30);
        }
    }

    const rev_idx = if (layout.items.len & 1 > 0) layout.items.len - 1 else layout.items.len - 2;
    //std.debug.print("len: {d}; files_rev: {d}\n", .{layout.items.len, files_rev});
    var fs_idx: usize = 0;
    var idx: usize = 0;
    while (idx < layout.items.len) {
        var len = layout.items[idx];
        if (idx & 1 == 0) {
            if (len > 128) {
                // this is now a gap of size len - 128
                len -= 128;
            } else {
                const pos = Position{ .len = @truncate(len), .id = @truncate(idx / 2) };
                total += calc_checksum(pos, &fs_idx);
                idx += 1;
                continue;
            }
        }

        std.debug.assert(len < 128);
        var rev_i = rev_idx;
        while (len > 0) {
            if (rev_i <= idx) {
                break;
            }

            const file_len = layout.items[rev_i];
            if (file_len == 0 or len < file_len) {
                rev_i -= 2;
                continue;
            }

            const pos = Position{ .len = @truncate(file_len), .id = @truncate(rev_i / 2) };
            total += calc_checksum(pos, &fs_idx);
            len -= file_len;
            // Flip the first bit to mark it as 'used'.
            layout.items[rev_i] += 128;
            rev_i -= 2;
        }

        if (len > 0) {
            const pos = Position{ .len = @truncate(len), .id = 0 };
            total += calc_checksum(pos, &fs_idx);
        }

        idx += 1;
    }

    return total;
}

test "test p1 ref" {
    const input = "2333133121414131402";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1928), p1(&reader));
}

test "test p1 single" {
    const input = "2";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 0), p1(&reader));
}

test "test p1 single2" {
    const input = "9";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 0), p1(&reader));
}

test "test p1 simple" {
    const input = "302";
    // result: 00011
    // 0 + 1 * 3 + 1 * 4
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 7), p1(&reader));
}

test "test p1 simple with small gap" {
    const input = "312";
    // result: 00011
    // 0 + 1 * 3 + 1 * 4
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 7), p1(&reader));
}

test "test p1 simple with 2blocks gap" {
    const input = "322";
    // result: 00011
    // 0 + 1 * 3 + 1 * 4
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 7), p1(&reader));
}

test "test p1 trailing gap" {
    const input = "23331331214141314020";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1928), p1(&reader));
}

test "test p1 trailing zeros" {
    const input = "233313312141413140200";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1928), p1(&reader));
}

//test "test p1 zeros" {
//    const input = "000000000000000000000";
//    var stream = std.io.fixedBufferStream(input);
//    const reader = stream.reader();
//    try std.testing.expectEqual(@as(usize, 0), p1(&reader));
//}

test "test p1 ignore 0 sized file1" {
    const input = "32002";
    // result: 00011
    // 0 + 2 * 3 + 2 * 4
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 14), p1(&reader));
}

test "test p1 ignore 0 sized file2" {
    const input = "32022";
    // result: 00011
    // 0 + 2 * 3 + 2 * 4
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 14), p1(&reader));
}

test "test fill_gap simple" {
    const allocator = std.heap.page_allocator;
    var input = std.ArrayList(u8).init(allocator);
    defer input.deinit();
    try input.append(3);
    try input.append(2);
    try input.append(2);
    var r_ptr: usize = 2;
    var f_id: usize = 3;

    try std.testing.expectEqual(7, fill_gap(0, &r_ptr, &f_id, 2, input.items));
    try std.testing.expectEqual(5, f_id);
    try std.testing.expectEqual(0, r_ptr);
}

test "test fill_gap leftover" {
    const allocator = std.heap.page_allocator;
    var input = std.ArrayList(u8).init(allocator);
    defer input.deinit();
    try input.append(3);
    try input.append(3);
    try input.append(2);
    var r_ptr: usize = 2;
    var f_id: usize = 3;

    try std.testing.expectEqual(7, fill_gap(0, &r_ptr, &f_id, 2, input.items));
    try std.testing.expectEqual(5, f_id);
    try std.testing.expectEqual(0, r_ptr);
}

test "test fill_gap small" {
    const allocator = std.heap.page_allocator;
    var input = std.ArrayList(u8).init(allocator);
    defer input.deinit();
    try input.append(3);
    try input.append(1);
    try input.append(2);
    var r_ptr: usize = 2;
    var f_id: usize = 3;

    try std.testing.expectEqual(3, fill_gap(0, &r_ptr, &f_id, 1, input.items));
    try std.testing.expectEqual(4, f_id);
    try std.testing.expectEqual(2, r_ptr);
    try std.testing.expectEqual(1, input.items[2]);
}

test "test p2 ref" {
    const input = "2333133121414131402";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 2858), p2(&reader));
}
