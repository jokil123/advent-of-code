import Data.List (sort)
import System.IO

main = do
  handle <- openFile "input.txt" ReadMode
  contents <- hGetContents handle
  print $ calculateSum (parseInput contents)
  hClose handle

parseInput :: String -> ([Int], [Int])
parseInput input =
  let w = map words (lines input)
   in (map (read . head) w, map (read . last) w)

calculateSum :: ([Int], [Int]) -> Int
calculateSum lists =
  let (a, b) = lists
   in sum [abs (x - y) | (x, y) <- zip (sort a) (sort b)]
