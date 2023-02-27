module Pangram (isPangram) where

import Data.Char

isPangram :: String -> Bool
isPangram text = [toLower c|c <- text]
