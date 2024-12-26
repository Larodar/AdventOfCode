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
        unreachable;
        //        total = try p2(&stdin);
    }

    std.debug.print("Result: {?}\n", .{total});
}

fn p1(reader: anytype) !usize {
    const allocator = std.heap.page_allocator;
    var buf = [_]u8{0} ** 4096;
    var data = std.ArrayList(u8).init(allocator);
    defer data.deinit();
    var width: usize = 0;
    if (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        width = l.len;
        try data.appendSlice(l);
    }

    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        try data.appendSlice(l);
    }

    const grid = Grid.init(data, width);
    _ = grid;
    return 0;
}

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
        if (col > self.width or row > self.len) {
            return null;
        }

        return self.inner.items[row * self.width + col];
    }

    fn posOf(self: *const Self, needle: u8) ?struct { row: usize, col: usize } {
        if (std.mem.indexOfScalar(u8, self.inner, needle)) |i| {
            return .{ .row = i / self.width, .col = i % self.width };
        } else {
            return null;
        }
    }

    fn findAll(self: *const Self, needle: u8) FindAllIter(Position) {
        return struct {
            inner: []const T,
            needle: T,
            const Self = @This();

            pub fn next(self: *Self) ?usize {
                if (self.inner.len == 0) {
                    return null;
                }
                if (std.mem.indexOfScalar(T, self.inner, self.needle)) |idx| {
                    self.inner = self.inner[idx + 1 ..];
                    return idx;
                } else {
                    self.inner = self.inner[self.inner.len..];
                    return null;
                }
            }
        };
        for (self.inner.items, 0..) |v, i| {
            const rhs: u7 = @truncate(needle);
            _ = rhs;
            _ = v;
            _ = i;
        }

        return null;
    }
};

fn FindAllIter(comptime T: type) type {
    return struct {
        inner: []const T,
        needle: T,
        const Self = @This();

        pub fn next(self: *Self) ?usize {
            if (self.inner.len == 0) {
                return null;
            }
            if (std.mem.indexOfScalar(T, self.inner, self.needle)) |idx| {
                self.inner = self.inner[idx + 1 ..];
                return idx;
            } else {
                self.inner = self.inner[self.inner.len..];
                return null;
            }
        }
    };
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
