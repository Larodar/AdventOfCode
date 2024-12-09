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

    reports: while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |line| {
        var levels = std.mem.splitAny(u8, line, " ");

        const first = try std.fmt.parseInt(i32, levels.next().?, 10);
        const second = try std.fmt.parseInt(i32, levels.next().?, 10);
        var current = second - first;
        var diff = @abs(current);
        const tendency = current >> 31;
        if (first == second or diff > 3) {
            continue :reports;
        }

        // extract sign bit
        var last = second;
        while (levels.next()) |l_str| {
            const l = try std.fmt.parseInt(i32, l_str, 10);
            current = l - last;
            diff = @abs(current);
            if (current >> 31 != tendency or diff > 3 or diff == 0) {
                continue :reports;
            }
            last = l;
        }

        total += 1;
    }

    return total;
}

fn p2(reader: anytype) !usize {
    var buf = [_]u8{0} ** 1024;
    var total: u64 = 0;

    var report_buf = [_]i32{0} ** 128;
    var derivative = [_]i32{0} ** 128;
    var scnd_der = [_]i32{0} ** 128;
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |line| {
        var levels = std.mem.splitAny(u8, line, " ");
        var idx: usize = 0;
        while (levels.next()) |l_str| {
            report_buf[0] = try std.fmt.parseInt(i32, l_str, 10);
            idx += 1;
        }

        var i: usize = 0;
        var wndw1 = std.mem.window(i32, report_buf[0..idx], 2, 1);
        while (wndw1.next()) |w| {
            derivative[i] = w[1] - w[0];
            i += 1;
        }

        const slice = derivative[0..i];
        if (std.mem.count(i32, slice, &[_]i32{0}) > 1) {
            // more than one duplicate
            continue;
        }

        if (std.mem.max(i32, slice) < 0) {
            if (count_diff_larger_three(slice) < 2) {
                // the entire array is descending with atmost one big leap
                total += 1;
            }
            continue;
        }

        if (std.mem.min(i32, slice) > 0) {
            if (count_diff_larger_three(slice) < 2) {
                // the entire array is ascending with atmost one big leap
                total += 1;
            }
            total += 1;
            continue;
        }

        var ii: usize = 0;
        var wndw2 = std.mem.window(i32, slice, 2, 1);
        while (wndw2.next()) |w| {
            const one = w[0];
            const two = w[1];
            scnd_der[ii] = if (one < 0 and two < 0 or one > 0 and two > 0) 0 else 1;
            ii += 1;
        }

        var last_was_1 = false;
        var changes: u8 = 0;
        for (scnd_der[0..ii]) |value| {
            if (value == 0) {
                last_was_1 = false;
                continue;
            }

            if (value == 1) {
                if (last_was_1) {
                    if (changes == 1) {
                        break;
                    }
                    changes += 1;
                }
                last_was_1 = true;
            }
        }

        if (changes < 2) {
            total += 1;
        }
    }

    return total;
}

fn count_diff_larger_three(slice: []const i32) usize {
    var count: usize = 0;
    for (slice) |b| {
        if (@abs(b) > 3) {
            count += 1;
        }
    }
    return count;
}

test "test p2 descending" {
    const input = "5 6 4 3 2 1";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1), p2(&reader));
}

test "test p2 ascending" {
    const input = "5 6 4 7 8 9";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1), p2(&reader));
}

test "test p2 descending with duplicate" {
    const input = "8 6 4 4 1";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1), p2(&reader));
}

test "test p2 unsafe" {
    const input = "9 7 6 2 1";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 0), p2(&reader));
}

test "test p2 unsafe2" {
    const input = "1 2 7 8 9";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 0), p2(&reader));
}

test "test p2 unsafe3" {
    const input = "16 18 16 16 12";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 0), p2(&reader));
}

test "test p2 unsafe4" {
    const input = "6 9 8 8 7 4 6";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 0), p2(&reader));
}

test "test p2 safe2" {
    const input = "1 3 2 4 5";
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 1), p2(&reader));
}

test "test p2 reference" {
    const input =
        \\7 6 4 2 1
        \\1 3 2 4 5
        \\8 6 4 4 1
        \\1 3 6 7 9
        \\1 2 7 8 9
        \\9 7 6 2 1
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 4), p2(&reader));
}
