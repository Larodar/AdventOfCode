// See https://aka.ms/new-console-template for more information

public class Program
{
    public static void Main(string[] args)
    {
        int r1 = 0;
        int r2 = 0;
        while (Console.ReadLine() is { } line)
        {
            ReadOnlySpan<char> l = line.AsSpan();
            var idx = l.IndexOf(':');
            var split = l[(idx + 1)..].Split([';', ',']);
            int red = 0;
            int blue = 0;
            int green = 0;
            while (split.TryNext(out var info))
            {
                info = info.Trim();
                var i = info.IndexOf(' ');
                int val = int.Parse(info[..i]);

                (red, blue, green) = info[(i + 1)..] switch
                {
                    "red" => (Math.Max(red, val), blue, green),
                    "green" => (red, blue, Math.Max(green, val)),
                    "blue" => (red, Math.Max(blue, val), green),
                    _ => throw new Exception("Unknown color")
                };
            }

            r2 += red * blue * green;
            if (red <= 12 && blue <= 14 && green <= 13)
            {
                // parse id
                var gameInfo = l[..idx];
                idx = gameInfo.IndexOf(' ');
                gameInfo = gameInfo[(idx + 1)..];
                var id = int.Parse(gameInfo);
                r1 += id;
            }
        }

        Console.WriteLine($"P1: {r1}; P2: {r2}");
    }
}

public ref struct Split<T>
where T : IEquatable<T>
{
    private ReadOnlySpan<T> inner;
    private T[] del;

    public Split(T del, ReadOnlySpan<T> toSplit)
    {
        this.del = [del];
        this.inner = toSplit;
    }

    public Split(T[] del, ReadOnlySpan<T> toSplit)
    {
        this.del = del;
        this.inner = toSplit;
    }

    public bool TryNext(out ReadOnlySpan<T> result)
    {
        if (this.inner.IsEmpty)
        {
            result = ReadOnlySpan<T>.Empty;
            return false;
        }

        var idx = this.inner.IndexOfAny(del.AsSpan());
        if (idx == -1)
        {
            result = this.inner;
            this.inner = ReadOnlySpan<T>.Empty;
        }
        else
        {
            result = this.inner[..idx];
            this.inner = this.inner[(idx + 1)..];
        }

        return true;
    }
}

file static class Extension
{
    public static Split<T> Split<T>(this ReadOnlySpan<T> self, T del)
        where T : IEquatable<T>
        => new Split<T>(del, self);

    public static Split<T> Split<T>(this ReadOnlySpan<T> self, T[] del)
        where T : IEquatable<T>
        => new Split<T>(del, self);
}
