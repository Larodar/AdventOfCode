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
        //        total = try p2(&stdin);
    }

    std.debug.print("Result: {?}\n", .{total});
}

fn p1(reader: anytype) !usize {
    const allocator = std.heap.page_allocator;
    var buf = [_]u8{0} ** 4096;
    var data = std.ArrayList(Position).init(allocator);
    defer data.deinit();

    var width: usize = 0;
    if (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        width = l.len;
        for (l) |v| {
            try data.append(Position{ .val = @truncate(v), .marked = 0 });
        }
    }

    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        for (l) |v| {
            try data.append(Position{ .val = @truncate(v), .marked = 0 });
        }
    }

    const grid = Grid.init(data, width);
    if (grid.posOf('^')) |pos| {
        var col = pos.col;
        var row = pos.row;
        grid.mark(row, col);
        var dir = Direction.Up;
        walk: while (row < grid.len and col < grid.width) {
            switch (dir) {
                Direction.Up => {
                    while (grid.get(row - 1, col)) |val| {
                        if (val.val == '#') {
                            break;
                        }

                        row -= 1;
                        grid.mark(row, col);
                    } else {
                        break :walk;
                    }

                    dir = Direction.Right;
                },
                Direction.Right => {
                    while (grid.get(row, col + 1)) |val| {
                        if (val.val == '#') {
                            break;
                        }

                        col += 1;
                        grid.mark(row, col);
                    } else {
                        break :walk;
                    }

                    dir = Direction.Down;
                },
                Direction.Down => {
                    while (grid.get(row + 1, col)) |val| {
                        if (val.val == '#') {
                            break;
                        }

                        row += 1;
                        grid.mark(row, col);
                    } else {
                        break :walk;
                    }

                    dir = Direction.Left;
                },
                Direction.Left => {
                    while (grid.get(row, col - 1)) |val| {
                        if (val.val == '#') {
                            break;
                        }

                        col -= 1;
                        grid.mark(row, col);
                    } else {
                        break :walk;
                    }

                    dir = Direction.Up;
                },
            }
        }
    } else {
        @panic("No starting position.");
    }

    return grid.count_marked();
}

const Direction = enum { Up, Right, Down, Left };

const Position = packed struct(u8) {
    val: u7,
    marked: u1,
};

const Grid = struct {
    const Self = @This();
    inner: std.ArrayList(Position),
    width: usize,
    len: usize,

    fn init(data: std.ArrayList(Position), grid_width: usize) Self {
        return Self{ .inner = data, .width = grid_width, .len = data.items.len / grid_width };
    }

    fn get(self: *const Self, row: usize, col: usize) ?Position {
        if (row * self.width + col > self.inner.items.len) {
            return null;
        }

        return self.inner.items[row * self.width + col];
    }

    fn mark(self: *const Self, row: usize, col: usize) void {
        if (row * self.width + col > self.inner.items.len) {
            return;
        }

        self.inner.items[row * self.width + col].marked = 1;
    }

    fn count_marked(self: *const Self) usize {
        var ret: usize = 0;
        for (self.inner.items) |v| {
            ret += v.marked;
        }

        return ret;
    }

    fn posOf(self: *const Self, needle: u8) ?struct { row: usize, col: usize } {
        for (self.inner.items, 0..) |v, i| {
            const rhs: u7 = @truncate(needle);
            if (v.val == rhs) {
                return .{ .row = i / self.width, .col = i % self.width };
            }
        }

        return null;
    }
};
test "Position_mark_test1" {
    var input = Position{ .val = 0b111_1111, .marked = 0 };
    input.marked = 1;
    try std.testing.expect(input.val == 0b111_1111);
    try std.testing.expect(@as(u8, @bitCast(input)) == 0b1111_1111);
}

test "Position_mark_test2" {
    var input = Position{ .val = 0b011_1110, .marked = 0 };
    input.marked = 1;
    try std.testing.expect(input.val == 0b011_1110);
    std.debug.print("{b}\n", .{@as(u8, @bitCast(input))});
    try std.testing.expect(@as(u8, @bitCast(input)) == 0b1011_1110);
}

test "posOf_test" {
    const input = "000000100000";

    const allocator = std.heap.page_allocator;
    var data = std.ArrayList(Position).init(allocator);
    defer data.deinit();
    for (input) |v| {
        try data.append(Position{ .val = @truncate(v), .marked = 0 });
    }

    const grid = Grid.init(data, 4);
    const pos = grid.posOf('1').?;
    try std.testing.expect(pos.row == 1);
    try std.testing.expect(pos.col == 2);
}

test "p1_test" {
    const input =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 41), p1(&reader));
}
