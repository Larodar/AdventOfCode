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
        unreachable;
        //total = try p2(&stdin);
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

    var files = std.ArrayList(Position).init(allocator);
    defer files.deinit();

    var files_rev = if (layout.items.len & 1 > 0) layout.items.len - 1 else layout.items.len - 2;
    var idx: usize = 0;
    while (idx < files_rev) {
        const len = layout.items[idx];
        if (idx & 1 == 0) {
            try files.append(Position{ .len = @truncate(len), .id = @truncate(idx / 2) });
        } else {
            var empty_blocks = len;
            var file_len = layout.items[files_rev];
            while (file_len <= empty_blocks) {
                try files.append(Position{ .len = @truncate(file_len), .id = @truncate(files_rev / 2) });
                empty_blocks -= file_len;
                files_rev -= 2;
                file_len = layout.items[files_rev];
            }

            if (empty_blocks > 0) {
                try files.append(Position{ .len = @truncate(empty_blocks), .id = @truncate(files_rev / 2) });
                layout.items[files_rev] -= empty_blocks;
            }
        }
        idx += 1;
    }

    files.items[files.items.len - 1].len += @truncate(layout.items[idx]);

    // calc checksum
    var current_idx: usize = 0;
    for (files.items) |v| {
        std.debug.print("{d} - {d}\n", .{v.id, v.len});
        for (0..v.len) |value| {
            total += (value + current_idx) * v.id;
        }

        current_idx += v.len;
    }

    return total;
}

const Position = packed struct(u16) {
    len: u4,
    id: u12,
};

test "test p1" {
    const input = "2333133121414131402";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1928), p1(&reader));
}

test "test p1_simple1" {
    const input = "901";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1), p1(&reader));
}

test "test p1_simple2" {
    const input = "201";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1), p1(&reader));
}
