module Main where

import System.Environment (getArgs)
import Control.Monad (foldM)

import Flow


readInstruction :: Char -> Int
readInstruction '(' =  1
readInstruction ')' = -1
readInstruction _   =  0


part1 :: String -> String
part1 input =
    let parsed = map readInstruction input
        s = foldl (+) 0 parsed
    in  show s



part2 :: String -> String
part2 input =
    input
    |> map readInstruction
    |> scanl (+) 0
    |> zip [1..]
    |> dropWhile ((0 <) . snd)
    |> head
    |> fst
    |> \i -> i - 1
    |> show


main :: IO ()
main = do
  [partName, inputName] <- getArgs
  content <- readFile inputName
  if partName == "1"
      then putStrLn $ part1 content
      else putStrLn $ part2 content

