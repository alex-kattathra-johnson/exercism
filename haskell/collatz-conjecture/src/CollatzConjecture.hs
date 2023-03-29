module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz n
    | n == 1 = Just 0
    | even n && n > 0 = succ <$> collatz (n `div` 2)
    | odd n && n > 0 = succ <$> collatz (3*n+1)
    | otherwise = Nothing

