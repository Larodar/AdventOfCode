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
    var buf = [_]u8{0} ** 128;

    var total: usize = 0;
    var dial: i64 = 50;
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        if (l.len == 0) {
            break;
        }

        const slice = buf[0..l.len];
        const dir = buf[0];
        var count = try std.fmt.parseInt(i64, slice[1..], 10);
        if (dir == 'L') {
            count = count * -1;
        }

        dial = @mod(dial + count, 100);
        if (dial == 0) {
            total += 1;
        }
    }

    return total;
}

fn p2(reader: anytype) !usize {
    var buf = [_]u8{0} ** 128;

    var total: usize = 0;
    var dial: i64 = 50;
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        if (l.len == 0) {
            break;
        }

        const slice = buf[0..l.len];
        const dir = buf[0];
        var count = try std.fmt.parseInt(i64, slice[1..], 10);
        total += count / @abs(count);
        if (dir == 'L') {
            count = count * -1;
        }

        dial = @mod(@abs(dial + count), 100);
        if (dial == 0) {
            total += 1;
        }
    }

    return total;
}

test "test p1 ref" {
    const input =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 3), p1(&reader));
}

test "test p2 ref" {
    const input =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 6), p1(&reader));
}
