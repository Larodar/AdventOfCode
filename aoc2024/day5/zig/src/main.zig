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
    // make this two list of u8?
    var rules = std.ArrayList(Rule).init(allocator);
    defer rules.deinit();
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        if (l.len == 0) {
            break;
        }
        const idx = std.mem.indexOfScalar(u8, l, '|').?;
        const lhs = try std.fmt.parseInt(u8, l[0..idx], 10);
        const rhs = try std.fmt.parseInt(u8, l[idx + 1 ..], 10);
        try rules.append(Rule{ .first = lhs, .second = rhs });
    }

    var update = std.ArrayList(u8).init(allocator);
    defer update.deinit();
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        update.clearRetainingCapacity();
        var entry_iter = std.mem.splitScalar(u8, l, ',');
        while (entry_iter.next()) |e| {
            const entry = try std.fmt.parseInt(u8, e, 10);
            try update.append(entry);
        }

        if (check(rules.items[0..], update.items[0..])) {
            const mid = try std.math.divFloor(usize, update.items.len, 2);
            total += @as(usize, update.items[mid]);
        }
    }

    return total;
}

fn p2(reader: anytype) !usize {
    const allocator = std.heap.page_allocator;
    var buf = [_]u8{0} ** 4096;
    var total: usize = 0;
    // make this two list of u8?
    var rules = std.ArrayList(Rule).init(allocator);
    defer rules.deinit();
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        if (l.len == 0) {
            break;
        }
        const idx = std.mem.indexOfScalar(u8, l, '|').?;
        const lhs = try std.fmt.parseInt(u8, l[0..idx], 10);
        const rhs = try std.fmt.parseInt(u8, l[idx + 1 ..], 10);
        try rules.append(Rule{ .first = lhs, .second = rhs });
    }

    var update = std.ArrayList(u8).init(allocator);
    defer update.deinit();
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        update.clearRetainingCapacity();
        var entry_iter = std.mem.splitScalar(u8, l, ',');
        while (entry_iter.next()) |e| {
            const entry = try std.fmt.parseInt(u8, e, 10);
            try update.append(entry);
        }

        const u = update.items[0..];
        if (ord(rules.items[0..], u)) {
            const mid = try std.math.divFloor(usize, update.items.len, 2);
            total += @as(usize, update.items[mid]);
        }
    }

    return total;
}

fn ord(rules: []const Rule, u: []u8) bool {
    var ret = false;
    var idx: usize = 1;
    while (idx < u.len) {
        const e = u[idx];

        var rule_iter = ApplyRuleIterator(true).new(rules, e);
        while (rule_iter.next()) |r| {
            if (std.mem.indexOfScalar(u8, u[0..idx], r.second)) |at| {
                ret = true;
                const source = u[at + 1 .. idx + 1];
                const dest = u[at..idx];
                std.mem.copyForwards(u8, dest, source);
                u[idx] = r.second;
                idx -= 1;
            }
        }

        idx += 1;
    }

    return ret;
}

fn check(rules: []const Rule, update: []const u8) bool {
    for (rules) |r| {
        if (std.mem.indexOfScalar(u8, update, r.first)) |pos| {
            if (std.mem.indexOfScalar(u8, update[0..pos], r.second)) |found_at| {
                _ = found_at;
                return false;
            }
        }
    }

    return true;
}

fn ApplyRuleIterator(comptime first: bool) type {
    return struct {
        const Self = @This();
        rules: []const Rule,
        val: u8,

        fn new(rules: []const Rule, val: u8) Self {
            return Self{
                .rules = rules,
                .val = val,
            };
        }

        fn next(self: *Self) ?Rule {
            if (self.rules.len == 0) {
                return null;
            }

            for (self.rules[0..], 0..) |value, i| {
                const v = if (first) value.first else value.second;
                if (v == self.val) {
                    self.rules = self.rules[i + 1 ..];
                    return value;
                }
            }

            return null;
        }
    };
}

const Rule = packed struct { first: u8, second: u8 };

test "test p1" {
    const input =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 143), p1(&reader));
}

test "test p2" {
    const input =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 123), p2(&reader));
}

test "test p2 simple" {
    const input =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\61,13,29
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 123), p2(&reader));
}
