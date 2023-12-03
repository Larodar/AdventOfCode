:- module main.
:- interface.
:- import_module io.

:- pred main(io::di, io::uo) is cc_multi.
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
    io.format("P1: %d; P2: %d", [to_string(R1), to_string(R2)], !IO).
    

:- pred solve(int::in, int::in, int::out, int::out, io::di, io::uo) is det.
solve(IdTotal, PowerTotal, R1, R2, !IO) :-
    io.read_line_as_string(Result, !IO),
    (
        Result = eof
    ;
        Result = ok(Line),
        ( process_game(Line, Id, Power),
          add(IdTotal, Id, IdTotal),
          add(PowerTotal, Power, PowerTotal)
        ),
        solve(!IO, IdTotal, PowerTotal, R1, R2)
    ;
        Result = error(ErrorCode),
        io.format("%s", [s(io.error_message(errorCode))], !IO),
        io.nl
    ).

:- pred process_game(string::in, int::out, int::out) is det.
process_game(Line, Id, Power) :-
    (
        det_head_tail(split_at_char(':', Line), GameInfo, Rest),
        map(trim_space_prefix, split_at_separator(should_split, Rest), CubeInfo)
    ).

:- pred trim_space_prefix(string::in, string::out) is det.
trim_space_prefix(Input, Output) :-
    (
        det_remove_prefix(" ", Input, Output)
    ).

:- func cube_info(string::in) = data.
cube_info(S) = info(Color, to_int(Count)) :-
    (
        det_head_tail(split_at_char(' ', S), Count, Tail),
        det_head(Tail, Color)
    ).

:- pred should_split(char::in) is semidet.
should_split(';').
should_split(',').

:- func update(string, int, data) = data.
update("red", Val, C) = dist(C^red `max` Val, C^green, C^blue).
update("green", Val, C) = dist(C^red, C^green `max` Val, C^blue).
update("blue", Val, C) = dist(C^red, C^green, C^blue `max` Val).
update(_,_,_) = dist(0,0,0).
