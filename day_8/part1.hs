module Main where

import Data.Map (Map)
import qualified Data.Map as Map
import Debug.Trace

data Node = Node { left :: String, right :: String, element :: String } deriving (Show)

findNextElement 'L' = take 3 . drop 7
findNextElement 'R' = take 3 . drop 12

parseLines :: [String] -> [(String, Node)]
parseLines = map (\x -> (take 3 x, Node (findNextElement 'L' x) (findNextElement 'R' x) (take 3 x)))

parseNode :: Map String Node -> [Char] -> Node -> Int -> Int
parseNode _ _ (Node _ _ "ZZZ") count = count
parseNode nodes ('L':directions) currentNode count = parseNode nodes directions (nodes Map.! left currentNode) (count + 1)
parseNode nodes ('R':directions) currentNode count = parseNode nodes directions (nodes Map.! right currentNode) (count + 1)

main = do
  fileInput <- readFile "input.txt"
  let fileLines = lines fileInput
  let repeatedInstructions = cycle (head fileLines)
  let nodeLines = drop 2 fileLines
  let nodes = parseLines nodeLines
  let mapping = Map.fromList nodes
  let initialNode = mapping Map.! "AAA"
  let count = parseNode mapping repeatedInstructions initialNode 0
  putStrLn $ "The count is " ++ show count
