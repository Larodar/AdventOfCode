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
    var buf = [_]u8{0} ** 4096;
    var ops = [_]u64{0} ** 16;
    var total: u64 = 0;
    // operands
    var op_idx: usize = 0;
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        const result_idx = std.mem.indexOfScalar(u8, l, ':').?;
        const result = try std.fmt.parseInt(u64, l[0..result_idx], 10);
        op_idx = 0;
        var op_iter = std.mem.splitScalar(u8, l[result_idx + 2 ..], ' ');
        while (op_iter.next()) |op_str| {
            const op = try std.fmt.parseInt(u64, op_str, 10);
            ops[op_idx] = op;
            op_idx += 1;
        }

        // each bit is an operator
        // 0 -> +
        // 1 -> *
        var permutation: u16 = 0;
        const mask: u16 = 1;
        while (permutation < std.math.pow(u16, 2, @truncate(op_idx - 1))) {
            var local = ops[0];
            for (ops[1..op_idx], 0..) |value, i| {
                if ((permutation & mask << @truncate(i)) > 0) {
                    // mul
                    local *= value;
                } else {
                    // add
                    local += value;
                }
            }

            if (local == result) {
                total += local;
                break;
            }

            permutation += 1;
        }
    }

    return total;
}

fn p2(reader: anytype) !usize {
    var buf = [_]u8{0} ** 4096;
    var ops = [_]u64{0} ** 16;
    var concat_buf = [_]u8{0} ** 128;
    var total: u64 = 0;
    // operands
    var op_idx: usize = 0;
    while (try reader.readUntilDelimiterOrEof(buf[0..], '\n')) |l| {
        const result_idx = std.mem.indexOfScalar(u8, l, ':').?;
        const result = try std.fmt.parseInt(u64, l[0..result_idx], 10);
        op_idx = 0;
        var op_iter = std.mem.splitScalar(u8, l[result_idx + 2 ..], ' ');
        while (op_iter.next()) |op_str| {
            const op = try std.fmt.parseInt(u64, op_str, 10);
            ops[op_idx] = op;
            op_idx += 1;
        }

        // each bit is an operator
        // 0 -> +
        // 1 -> *
        // 2 -> ||
        // 00000000_00000000_00000000_00000000_00000000_0000000_0000000_00000000
        var permutation: u64 = 0;
        permutations: while (permutation < std.math.pow(u64, 4, op_idx - 1)) {
            var local = ops[0];
            for (ops[1..op_idx], 0..) |value, i| {
                const idx: u6 = @truncate(i);
                const state = (permutation >> (idx * 2)) & 0x0000_0000_0000_0003;
                switch (state) {
                    0 => {
                        // mul
                        local *= value;
                    },
                    1 => {
                        // add
                        local += value;
                    },
                    2 => {
                        // concat
                        const slice = try std.fmt.bufPrint(concat_buf[0..], "{d}", .{local});
                        const val_slice = try std.fmt.bufPrint(concat_buf[slice.len..], "{d}", .{value});
                        local = try std.fmt.parseInt(u64, concat_buf[0 .. slice.len + val_slice.len], 10);
                    },
                    else => {
                        permutation += 1;
                        continue :permutations;
                    },
                }
            }

            if (local == result) {
                //std.debug.print("{s}\n", .{l});
                total += local;
                break;
            }

            permutation += 1;
        }
    }

    return total;
}

const Operator = enum { Add, Mul };

test "p1_test" {
    const input =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
        \\156: 15 6
        \\7290: 6 8 6 15
        \\161011: 16 10 13
        \\192: 17 8 14
        \\21037: 9 7 18 13
        \\292: 11 6 16 20
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 3749), p1(&reader));
}

test "p2_test" {
    const input =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
        \\156: 15 6
        \\7290: 6 8 6 15
        \\161011: 16 10 13
        \\192: 17 8 14
        \\21037: 9 7 18 13
        \\292: 11 6 16 20
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 11387), p2(&reader));
}

test "p2_single_test" {
    const input =
        \\156: 15 6
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 156), p2(&reader));
}

test "p2_single_concat_test" {
    const input =
        \\123456789: 1 2 3 4 5 6 7 8 9
    ;
    var stream = std.io.fixedBufferStream(input);
    const reader = stream.reader();
    try std.testing.expectEqual(@as(usize, 123456789), p2(&reader));
}
