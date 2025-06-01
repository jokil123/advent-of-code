import Data.List (sort)
import Data.Map qualified as Data
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

-- calculateSimilarityScore :: [Int] -> [Int] -> Int

toOccurance :: Int -> Data.Map Int Int
toOccurance = Data.fromList [if i == n then 1 else 0 | i <- [0 .. 9]]