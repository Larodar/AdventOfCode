:- module main.
:- interface.
:- import_module io.

:- pred main(io::di, IO::uo) is cc_multi.
:- implementation.
:- import_module char, list, string.

main(!IO) :-
    solve(!IO, 0, 0, R1, R2),
    io.format("P1: %d; P2: %d", R1, R2).
    

:- pred solve(!IO, int::in, int::in, int::out, int::out) is semidet.
solve(!IO, IdTotal, PowerTotal, R1, R2) :-
    io.read_line_as_string(Resut, !IO),
    (
        Result = eof
    ;
        Result = ok(Line),
        ( process_game(Line, Id, Power),
          add(IdTotal,Id,IdTotal),
          add(PowerTotal,Power,PowerTotal),
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
        det_head_tail(split_at_char(':',Line), GameInfo, Rest),
        map(trim_space_prefix, split_at_char(should_split, Rest), CubeInfo),
    ).

:- pred trim_space_prefix(string::in, string::out) is det.
trim_space_prefix(Input, Output) :-
    (
        det_remove_prefix(" ", Input, Output)
    ).

:- func cube_info(string) = (string, int).
cube_info(S) = split_at_char(' '

:- func should_split(char) = bool.
should_split(';') = true.
should_split(',') = true.
should_split(_) = false.

:- func update(string, int, int,int,int) = (int, int, int).
update("red", val, Red, Green, Blue) = (Red `max` val, Green, Blue).
update("green", val, Red, Green, Blue) = (Red, Green `max` val, Blue).
update("blue", val, Red, Green, Blue) = (Red, Green, Blue `max` val).
update(_,_,_,_,_) = (0,0,0).
