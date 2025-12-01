const std = @import("std");
const io = std.io;

pub fn StdinLinesIterator(comptime T: type, comptime del_type: type) type {
    return struct {
        buffer: []const T,
        reader: *std.io.Reader,
        delimiter: switch (del_type) {
            .sequence, .any => []const T,
            .scalar => T,
        },
        const Self = @This();

        pub fn next(self: *Self) []const u8 {
            if (try self.reader.readUntilDelimiterOrEof(self.buffer[0..], self.delimiter)) |line| {
                return line;
            }
        }
    };
}
