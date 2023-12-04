:- module main.
:- interface.
:- import_module io.

:- pred main(io::di, io::uo) is det.
:- implementation.
:- import_module char, list, string, bool, integer, int.

:- type data
    --->    dist(
                red :: int,
                green :: int,
                blue :: int)
    ;       info(
                color :: string,
                count :: int).

main(!IO) :-
    solve(0, 0, R1, R2, !IO),
    io.format("P1: %i; P2: %i", [i(R1), i(R2)], !IO).
    

:- pred solve(int::in, int::in, int::out, int::out, io::di, io::uo) is det.
solve(IdTotal, PowerTotal, R1, R2, !IO) :-
    io.read_line_as_string(Result, !IO),
    (
        Result = eof,
        ( R1 = IdTotal,
          R2 = PowerTotal
        )
    ;
        Result = ok(Line),
        ( process_game(Line, Id, Power),
          IdTotal = IdTotal + Id,
          PowerTotal = PowerTotal + Id
        ),
        solve(IdTotal, PowerTotal, R1, R2, !IO)
    ;
        Result = error(ErrorCode),
        io.format("%s\n", [s(io.error_message(ErrorCode))], !IO)
    ).

:- pred process_game(string::in, int::out, int::out) is det.
process_game(Line, Id, Power) :-
    (
        det_head_tail(split_at_char(':', Line), GameInfo, Rest),
        map(trim_space_prefix, split_at_separator(should_split, head(Rest)), _)
    ).

:- pred trim_space_prefix(string::in, string::out) is det.
trim_space_prefix(Input, Output) :-
    (
        det_remove_prefix(" ", Input, Output)
    ).

:- func cube_info(string) = data.
cube_info(S) = info(Color, Count) :-
    (
        det_head_tail(split_at_char(' ', S), CountStr, Tail),
        Count = to_int(CountStr)
    ).

:- pred should_split(char::in) is semidet.
should_split(';').
should_split(',').

:- pred update(string::in, int::in, data::in, data::out) is semidet.
update(Color, Val, Dist, Ret) :-
    (
        Color = "red",
        (
            Ret = dist(Dist^red `max` Val, Dist^green, Dist^blue)
        )
    ;
        Color = "green",
        (
            Ret = dist(Dist^red, Dist^green `max` Val, Dist^blue)
        )
    ;
        Color = "blue",
        (
            Ret = dist(Dist^red, Dist^green, Dist^blue `max` Val)
        )
    ).
