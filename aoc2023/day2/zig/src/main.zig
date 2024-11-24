const std = @import("std");
const mem = std.mem;

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdin_file = std.io.getStdIn().reader();
    var br = std.io.bufferedReader(stdin_file);
    var buf = [_]u8{0} ** 1024;
    const stdin = br.reader();
    var total: usize = 0;
    lines: while (try stdin.readUntilDelimiterOrEof(buf[0..], '\n')) |line| {
        var l = line;
        const prefix = try read_prefix(l);
        var pos = prefix.read_count;
        while (pos < l.len) {
            // 0: green
            // 0: blue
            // 0: red
            var colors = [_]u16{0} ** 3;
            var draw_end = false;
            while (!draw_end) {
                l = l[pos + 2 ..];
                const result = try read_one(l);
                pos = result.read_count;
                draw_end = result.draw_end;
                colors[@intFromEnum(result.color)] = result.count;
            }

            if (colors[@intFromEnum(Color.Green)] > 13 or colors[@intFromEnum(Color.Blue)] > 14 or colors[@intFromEnum(Color.Red)] > 12) {
                continue :lines;
            }
        }

        total += prefix.id;
    }

    std.debug.print("Result: {?}\n", .{total});
}

const Color = enum { Green, Blue, Red };

const ColorResult = struct {
    color: Color,
    count: u16,
    draw_end: bool,
    read_count: usize,

    pub fn red(count: u16, read_count: usize, draw_end: bool) ColorResult {
        return ColorResult{ .color = Color.Red, .count = count, .draw_end = draw_end, .read_count = read_count };
    }
    pub fn green(count: u16, read_count: usize, draw_end: bool) ColorResult {
        return ColorResult{ .color = Color.Green, .count = count, .draw_end = draw_end, .read_count = read_count };
    }
    pub fn blue(count: u16, read_count: usize, draw_end: bool) ColorResult {
        return ColorResult{ .color = Color.Blue, .count = count, .draw_end = draw_end, .read_count = read_count };
    }
};

const PrefixResult = struct {
    id: usize,
    read_count: usize,

    pub fn init(id: usize, read_count: usize) PrefixResult {
        return PrefixResult{ .id = id, .read_count = read_count };
    }
};

fn read_prefix(slice: []const u8) std.fmt.ParseIntError!PrefixResult {
    // remove 'Game '
    const l = slice[5..];
    const end = mem.indexOfScalar(u8, l, ':').?;
    const id = try std.fmt.parseInt(u16, l[0..end], 10);
    // +2 for the ': '
    return PrefixResult.init(id, end + 5);
}

fn read_one(slice: []const u8) std.fmt.ParseIntError!ColorResult {
    var s = slice;
    const val_idx = mem.indexOfScalar(u8, s, ' ').?;
    const val = s[0..val_idx];
    const count = try std.fmt.parseInt(u16, val, 10);
    s = s[val_idx + 1 ..];
    const color_idx = if (mem.indexOfAny(u8, s, &[_]u8{ ',', ';' })) |idx| idx else s.len;
    const draw_end = s.len == color_idx or s[color_idx] == ';';
    const col = s[0..color_idx];
    const read_count = val_idx + 1 + color_idx;
    if (mem.eql(u8, col, "red")) {
        return ColorResult.red(count, read_count, draw_end);
    } else if (mem.eql(u8, col, "blue")) {
        return ColorResult.blue(count, read_count, draw_end);
    } else {
        return ColorResult.green(count, read_count, draw_end);
    }
}

test "simple test" {}
