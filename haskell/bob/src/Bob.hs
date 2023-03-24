module Bob (responseFor) where

import Data.Char (isLetter, isLower)
import qualified Data.Text as T
import           Data.Text (Text)

responseFor :: Text -> Text
responseFor (T.strip -> xs)
    | isSilent = "Fine. Be that way!"
    | isYelled && isQuestion = "Calm down, I know what I'm doing!"
    | isYelled = "Whoa, chill out!"
    | isQuestion = "Sure."
    | otherwise = "Whatever."
    where
        isSilent = T.null xs
        isQuestion = T.last xs == '?'
        isYelled = T.any isLetter xs && not (T.any isLower xs)