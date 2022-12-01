module Main where

main :: IO ()
main = do
    s <- readFile "/home/larodar/source/AdventOfCode/day1/input.txt"
    let list = (toInt . lines) s
    (print . tupleFirst . compareWithNext) (0, 0, list)

tupleFirst :: (a, b) -> a
tupleFirst (x, _) = x

toInt :: [String] -> [Int]
toInt = map read

compareWithNext :: (Int, Int, [Int]) -> (Int, [Int])
compareWithNext (0, 0, list) = compareWithNext (0, head list, tail list)
compareWithNext (count, _, []) = (count, [])
compareWithNext (count, last, list) = do
    let current = head list
    if last < current then 
      compareWithNext (count + 1, head list, tail list) 
    else 
      compareWithNext(count, current, tail list)
