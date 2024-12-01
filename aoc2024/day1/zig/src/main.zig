const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    var args = std.process.args();
    _ = args.skip();
    const problem_str = args.next().?;
    const problem = try std.fmt.parseInt(u8, problem_str, 10);
    var total: usize = undefined;
    if (problem == 1) {
        total = try p1(allocator);
    } else if (problem == 2) {
        total = try p2(allocator);
    }

    std.debug.print("Result: {?}\n", .{total});
}

fn p1(alloc: std.mem.Allocator) !usize {
    const stdin_file = std.io.getStdIn().reader();
    var br = std.io.bufferedReader(stdin_file);
    var buf = [_]u8{0} ** 1024;
    const stdin = br.reader();

    var rhs = std.ArrayList(i32).init(alloc);
    defer rhs.deinit();
    var lhs = std.ArrayList(i32).init(alloc);
    defer lhs.deinit();

    while (try stdin.readUntilDelimiterOrEof(buf[0..], '\n')) |line| {
        var it = std.mem.splitAny(u8, line, " ");
        const lhs_raw = it.next().?;
        const rhs_raw = std.mem.trimLeft(u8, it.rest(), " ");
        const new_lhs = try std.fmt.parseInt(i32, lhs_raw, 10);
        const new_rhs = try std.fmt.parseInt(i32, rhs_raw, 10);
        try lhs.append(new_lhs);
        try rhs.append(new_rhs);
    }

    std.mem.sort(i32, lhs.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, rhs.items, {}, comptime std.sort.asc(i32));

    var total: usize = 0;
    for (lhs.items, 0..) |lhs_item, i| {
        const rhs_item = rhs.items[i];
        total += @abs(lhs_item - rhs_item);
    }

    return total;
}

fn p2(alloc: std.mem.Allocator) !usize {
    const stdin_file = std.io.getStdIn().reader();
    var br = std.io.bufferedReader(stdin_file);
    var buf = [_]u8{0} ** 1024;
    const stdin = br.reader();

    var rhs = std.ArrayList(i32).init(alloc);
    defer rhs.deinit();
    var lhs = std.ArrayList(i32).init(alloc);
    defer lhs.deinit();

    while (try stdin.readUntilDelimiterOrEof(buf[0..], '\n')) |line| {
        var it = std.mem.splitAny(u8, line, " ");
        const lhs_raw = it.next().?;
        const rhs_raw = std.mem.trimLeft(u8, it.rest(), " ");
        const new_lhs = try std.fmt.parseInt(i32, lhs_raw, 10);
        const new_rhs = try std.fmt.parseInt(i32, rhs_raw, 10);
        try lhs.append(new_lhs);
        try rhs.append(new_rhs);
    }

    var total: usize = 0;
    for (lhs.items) |item| {
        for (rhs.items) |v| {
            if (item == v) {
                total += @intCast(item);
            }
        }
    }

    return total;
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
