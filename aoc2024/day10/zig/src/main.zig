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
        for (l) |value| {
            try data.append(Position{ .val = value - 0x30 });
        }
    }

    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        for (l) |value| {
            try data.append(Position{ .val = value - 0x30 });
        }
    }

    var total: u64 = 0;
    var grid = Grid.init(data, width);
    var ptr = GridPtr.init(&grid);
    while (ptr.findNext(0)) |pos| {
        total += try follow_path(&grid, pos, 1, Trajectory.None);
    }

    return total;
}

const Trajectory = enum { None, Left, Up, Right, Down };

const GridPos = struct {
    const Self = @This();
    col: usize,
    row: usize,

    fn down(self: *const Self) ?GridPos {
        if (self.row == std.math.maxInt(usize)) {
            return null;
        }

        return GridPos{ .col = self.col, .row = self.row + 1 };
    }
    fn up(self: *const Self) ?GridPos {
        if (self.row == 0) {
            return null;
        }

        return GridPos{ .col = self.col, .row = self.row - 1 };
    }
    fn right(self: *const Self) ?GridPos {
        if (self.col == std.math.maxInt(usize)) {
            return null;
        }

        return GridPos{ .col = self.col + 1, .row = self.row };
    }
    fn left(self: *const Self) ?GridPos {
        if (self.col == 0) {
            return null;
        }

        return GridPos{ .col = self.col - 1, .row = self.row };
    }
};

fn follow_path(grid: *Grid, pos: GridPos, val: u7, trajectory: Trajectory) !u64 {
    if (grid.get_mut(pos)) |v| {
        std.debug.print("checking pos: {d}:{d} with val {d}\n", .{ pos.row, pos.col, v.val });
        if (v.val == 9) {
            return 1;
        }

        if (v.val != val) {
            return 0;
        }

        const expected = val + 1;
        var paths: u64 = 0;
        if (trajectory != Trajectory.Right) {
            if (pos.left()) |l| {
                paths += try follow_path(grid, l, expected, Trajectory.Left);
            }
        }

        if (trajectory != Trajectory.Left) {
            if (pos.right()) |r| {
                paths += try follow_path(grid, r, expected, Trajectory.Right);
            }
        }

        if (trajectory != Trajectory.Down) {
            if (pos.up()) |u| {
                paths += try follow_path(grid, u, expected, Trajectory.Up);
            }
        }

        if (trajectory != Trajectory.Up) {
            if (pos.down()) |d| {
                paths += try follow_path(grid, d, expected, Trajectory.Down);
            }
        }

        return paths;
    } else {
        return 0;
    }
}

const GridPtr = struct {
    const Self = @This();
    inner: *Grid,
    ptr: usize,

    fn init(grid: *Grid) GridPtr {
        return GridPtr{ .inner = grid, .ptr = 0 };
    }

    fn reset(self: *Self) void {
        self.ptr = 0;
    }

    fn findNext(self: *Self, needle: u8) ?GridPos {
        while (self.ptr < self.inner.inner.items.len) {
            const v = self.inner.inner.items[self.ptr];
            self.ptr += 1;
            if (v.val == needle) {
                return .{ .row = self.ptr / self.inner.width, .col = self.ptr % self.inner.width };
            }
        }

        return null;
    }
};

const Position = packed struct(u8) { val: u8 };

const Grid = struct {
    const Self = @This();
    inner: std.ArrayList(Position),
    width: usize,
    len: usize,

    fn init(data: std.ArrayList(Position), grid_width: usize) Self {
        return Self{ .inner = data, .width = grid_width, .len = data.items.len / grid_width };
    }

    fn get(self: *const Self, pos: GridPos) ?Position {
        if (pos.row * self.width + pos.col >= self.inner.items.len) {
            return null;
        }

        return self.inner.items[pos.row * self.width + pos.col];
    }

    fn get_mut(self: *Self, pos: GridPos) ?*Position {
        if (pos.row * self.width + pos.col >= self.inner.items.len) {
            return null;
        }

        return &self.inner.items[pos.row * self.width + pos.col];
    }

    fn posOf(self: *const Self, needle: u8) ?struct { row: usize, col: usize } {
        for (self.inner.items, 0..) |v, i| {
            if (v.val == needle) {
                return .{ .row = i / self.width, .col = i % self.width };
            }
        }

        return null;
    }

    fn score(self: *const Self) u64 {
        var ret: u64 = 0;
        for (self.inner) |value| {
            ret += value.score;
        }
        return ret;
    }
};

test "p1 ref" {
    const input =
        \\89010123
        \\78121874
        \\87430965
        \\96549874
        \\45678903
        \\32019012
        \\01329801
        \\10456732
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 41), p1(&reader));
}
