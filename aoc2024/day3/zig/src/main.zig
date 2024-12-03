const std = @import("std");
const io = std.io;

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();
    const problem_str = args.next().?;
    const problem = try std.fmt.parseInt(u8, problem_str, 10);

    const stdin_file = std.io.getStdIn().reader();
    var br = std.io.bufferedReader(stdin_file);
    const stdin = br.reader();

    var total: usize = undefined;
    if (problem == 1) {
        total = try p1(&stdin);
    } else if (problem == 2) {
        total = try p2(&stdin);
    }

    std.debug.print("Result: {?}\n", .{total});
}

fn p1(reader: anytype) !usize {
    var buf = [_]u8{0} ** 1024;
    var total: u64 = 0;

    while (try reader.readUntilDelimiterOrEof(buf[0..], 'm')) |segment| {
        if (segment.len < 7) {
            continue;
        }

        if (!std.mem.startsWith(u8, segment, "ul(")) {
            continue;
        }

        if (try read_factors(segment[3..])) |result| {
            total += @as(u64, result);
        }
    }
    return total;
}

fn read_factors(segment: []const u8) !?u64 {
    if (segment.len == 0 or segment[0] < 0x30 or segment[0] > 0x39) {
        return null;
    }

    var factor_1: u64 = 0;
    var factor_2: u64 = 0;
    var pos: usize = 0;
    if (std.mem.indexOfNone(u8, segment, "0123456789")) |idx| {
        if (idx == segment.len - 1) {
            return null;
        }

        factor_1 = try std.fmt.parseInt(u64, segment[0..idx], 10);
        if (segment[idx] != ',') {
            return null;
        }
        pos = idx + 1;
    } else {
        return null;
    }
    const s = segment[pos..];
    if (std.mem.indexOfNone(u8, s, "0123456789")) |idx| {
        if (idx == segment.len - 1) {
            return null;
        }

        factor_2 = try std.fmt.parseInt(u64, s[0..idx], 10);
        if (s[idx] != ')') {
            return null;
        }

        pos = idx + 1;
    } else {
        return null;
    }

    return factor_1 * factor_2;
}

fn p2(reader: anytype) !usize {
    var buf = [_]u8{0} ** 4096;
    var total: u64 = 0;

    var enabled = true;
    while (try reader.readUntilDelimiterOrEof(buf[0..], 'd')) |segment| {
        if (segment.len < 3) {
            continue;
        }

        var s = segment;
        if (std.mem.startsWith(u8, segment, "on't()")) {
            enabled = false;
            continue;
        }

        if (std.mem.startsWith(u8, segment, "o()")) {
            enabled = true;
            s = s[3..];
        }

        if (enabled) {
            total += try consume_mul(s);
        }
    }

    return total;
}

fn consume_mul(segment: []const u8) !usize {
    var total: usize = 0;
    var iter = std.mem.splitSequence(u8, segment, "mul(");
    while (iter.next()) |s| {
        if (try read_factors(s)) |result| {
            total += @as(u64, result);
        }
    }

    return total;
}

test "test p1" {
    const input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 161), p1(&reader));
}

test "test p2" {
    const input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 48), p2(&reader));
}
