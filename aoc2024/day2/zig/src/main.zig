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

        //std.debug.print("{s}\n", .{line});
        total += 1;
    }

    return total;
}

fn p2(reader: anytype) !usize {
    var buf = [_]u8{0} ** 1024;
    var total: u64 = 0;

    reports: while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |line| {
        var levels = std.mem.splitAny(u8, line, " ");

        var last = try std.fmt.parseInt(i32, levels.next().?, 10);
        var dampened: bool = true;
        var direction: i32 = -1;
        while (levels.next()) |l_str| {
            const l = try std.fmt.parseInt(i32, l_str, 10);
            const diff = l - last;
            const abs = @abs(diff);
            const tendency = diff >> 31;
            if (abs == 0 or abs > 3 or (direction != -1 and direction != tendency)) {
                if (!dampened) {
                    continue :reports;
                }
                dampened = false;
            }

            direction = tendency;
            last = l;
        }

        total += 1;
    }

    return total;
}
