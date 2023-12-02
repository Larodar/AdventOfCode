open System

exception UnknownColor of string

module P1 =
    let processLine(l:string) =
        // split game info and drawing info
        let parts = l.Split(':', 2)
        // get id
        let id = Array.item 1 (parts[0].Split ' ') |> int
        let mutable red, green, blue = (0,0,0)
        for info in parts[1].Split [| ','; ';'|] do
            let infoParts = info.Trim().Split ' '
            let count = infoParts[0] |> int
            match infoParts[1] with
            | "red" -> red <- Math.Max(red, count)
            | "blue" -> blue <- Math.Max(blue, count)
            | "green" -> green <- Math.Max(green, count)
            | _ -> raise (UnknownColor("Unknown color"))
        let power = red * blue * green
        if red <= 12 && blue <= 14 && green <= 13 then (id, power) else (0, power)

    let p1 : int * int =
        let mutable p1Result, p2Result = (0,0)
        let mutable eos = false
        while not eos do
            let line = Console.ReadLine()
            if line = null then
                eos <- true
                ()
            else
                let id, power = processLine line
                p1Result <- p1Result + id
                p2Result <- p2Result + power

        (p1Result, p2Result)
        


// For more information see https://aka.ms/fsharp-console-apps
[<EntryPoint>]
let main args =
    match P1.p1 with
    | (r1, r2) -> printfn "returning result: P1 - %d; P2 - %d" r1 r2
    0
