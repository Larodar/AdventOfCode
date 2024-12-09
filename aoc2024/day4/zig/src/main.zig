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
    var data = std.ArrayList(u8).init(allocator);
    defer data.deinit();
    var width: usize = 0;
    if (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        width = l.len + 6;
        try data.appendNTimes(0, l.len + 6);
        try data.appendNTimes(0, l.len + 6);
        try data.appendNTimes(0, l.len + 6);
        try data.appendNTimes(0, 3);
        try data.appendSlice(l);
        try data.appendNTimes(0, 3);
    }

    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        try data.appendNTimes(0, 3);
        try data.appendSlice(l);
        try data.appendNTimes(0, 3);
    }

    try data.appendNTimes(0, width);
    try data.appendNTimes(0, width);
    try data.appendNTimes(0, width);

    const grid = Grid.init(data, width);

    var total: usize = 0;
    // init at 3 since we added padding
    var row: usize = 2;
    var col: usize = 2;
    // forward
    while (row < grid.len - 2) {
        col = 3;
        while (col < grid.width) {
            const letter = grid.get(row, col) orelse {
                col += 1;
                continue;
            };

            if (letter != 'X') {
                col += 1;
                continue;
            }

            //forward
            if (grid.get(row, col + 1) == 'M' and grid.get(row, col + 2) == 'A' and grid.get(row, col + 3) == 'S') {
                total += 1;
            }

            // backward
            if (grid.get(row, col - 1) == 'M' and grid.get(row, col - 2) == 'A' and grid.get(row, col - 3) == 'S') {
                total += 1;
            }

            // downward
            if (grid.get(row + 1, col) == 'M' and grid.get(row + 2, col) == 'A' and grid.get(row + 3, col) == 'S') {
                total += 1;
            }

            // upward
            if (grid.get(row - 1, col) == 'M' and grid.get(row - 2, col) == 'A' and grid.get(row - 3, col) == 'S') {
                total += 1;
            }

            // diag to top left
            if (grid.get(row - 1, col - 1) == 'M' and grid.get(row - 2, col - 2) == 'A' and grid.get(row - 3, col - 3) == 'S') {
                total += 1;
            }

            // diag to top right
            if (grid.get(row - 1, col + 1) == 'M' and grid.get(row - 2, col + 2) == 'A' and grid.get(row - 3, col + 3) == 'S') {
                total += 1;
            }

            // diag to bottom right
            if (grid.get(row + 1, col + 1) == 'M' and grid.get(row + 2, col + 2) == 'A' and grid.get(row + 3, col + 3) == 'S') {
                total += 1;
            }

            // diag to bottom right
            if (grid.get(row + 1, col - 1) == 'M' and grid.get(row + 2, col - 2) == 'A' and grid.get(row + 3, col - 3) == 'S') {
                total += 1;
            }

            col += 1;
        }

        row += 1;
    }

    return total;
}
fn p2(reader: anytype) !usize {
    const allocator = std.heap.page_allocator;
    var buf = [_]u8{0} ** 4096;
    var data = std.ArrayList(u8).init(allocator);
    defer data.deinit();
    var width: usize = 0;
    if (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        width = l.len + 2;
        try data.appendNTimes(0, l.len + 2);
        try data.appendNTimes(0, 1);
        try data.appendSlice(l);
        try data.appendNTimes(0, 1);
    }

    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        try data.appendNTimes(0, 1);
        try data.appendSlice(l);
        try data.appendNTimes(0, 1);
    }

    try data.appendNTimes(0, width);

    const grid = Grid.init(data, width);

    var total: usize = 0;
    // init at 3 since we added padding
    var row: usize = 1;
    var col: usize = 1;
    // forward
    while (row < grid.len - 1) {
        col = 1;
        while (col < grid.width) {
            const letter = grid.get(row, col) orelse {
                col += 1;
                continue;
            };

            if (letter != 'A') {
                col += 1;
                continue;
            }

            var current: usize = 0;
            // top right to bottom left
            if (grid.get(row - 1, col + 1) == 'M' and grid.get(row + 1, col - 1) == 'S') {
                current += 1;
            }

            // top to bottom
            //if (grid.get(row - 1, col) == 'M' and grid.get(row + 1, col) == 'S') {
            //    current += 1;
            //}

            // top left to bottom right
            if (grid.get(row - 1, col - 1) == 'M' and grid.get(row + 1, col + 1) == 'S') {
                current += 1;
            }

            // left to right
            //if (grid.get(row, col - 1) == 'M' and grid.get(row, col + 1) == 'S') {
            //    current += 1;
            //}

            // bottom left to top right
            if (grid.get(row + 1, col - 1) == 'M' and grid.get(row - 1, col + 1) == 'S') {
                current += 1;
            }

            // bottom to top
            //if (grid.get(row + 1, col) == 'M' and grid.get(row - 1, col) == 'S') {
            //    current += 1;
            //}

            // bottom right to top left
            if (grid.get(row + 1, col + 1) == 'M' and grid.get(row - 1, col - 1) == 'S') {
                current += 1;
            }

            // right to left
            //if (grid.get(row, col + 1) == 'M' and grid.get(row, col - 1) == 'S') {
            //    current += 1;
            //}

            if (current > 1) {
                //std.debug.print("row|col: {d}|{d}\n", .{row, col});
                total += 1;
            }

            col += 1;
        }

        row += 1;
    }

    return total;
}

const Grid = struct {
    const Self = @This();
    inner: std.ArrayList(u8),
    width: usize,
    len: usize,

    fn init(data: std.ArrayList(u8), grid_width: usize) Self {
        return Self{ .inner = data, .width = grid_width, .len = data.items.len / grid_width };
    }

    fn get(self: *const Self, row: usize, col: usize) ?u8 {
        if (col > self.width or row > self.len) {
            return null;
        }

        return self.inner.items[row * self.width + col];
    }
};

test "p1_test" {
    const input =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 18), p1(&reader));
}

test "p1_double" {
    const input =
        \\.S..S..S..
        \\..A.A.A...
        \\...MMM....
        \\.SAMXMAS..
        \\...MMM....
        \\..A.A.A...
        \\.S..S..S..
        \\..........
        \\..........
        \\..........
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 8), p1(&reader));
}


test "p2_test" {
    const input =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 9), p2(&reader));
}

test "p2_double" {
    const input =
        \\..........
        \\..........
        \\..MMM.....
        \\..MAS.....
        \\..SSS.....
        \\..........
        \\..SSS.....
        \\..SAM.....
        \\..MMM.....
        \\..........
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 2), p2(&reader));
}
