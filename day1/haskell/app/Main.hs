module Main where

main :: IO ()
main = do
    s <- readFile "/home/larodar/source/AdventOfCode/day1/input.txt"
    (print . toInt . lines) s

toInt :: [String] -> [Int]
toInt = map read
