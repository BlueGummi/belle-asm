import System.IO
import System.Environment (getArgs, getProgName)
import System.Directory (renameFile, doesFileExist, removeFile)
import Data.List (isPrefixOf, isSuffixOf, isInfixOf)
import System.FilePath (takeBaseName, takeDirectory)
import Control.Exception (catch, SomeException)

formatLine :: String -> String
formatLine line
  | ";" `isInfixOf` line = ""
  | isSuffixOf ":" line || isPrefixOf "." line = line
  | otherwise = "    " ++ line

main :: IO ()
main = do
  commandLineArgs <- getArgs
  if null commandLineArgs
    then do
      progName <- getProgName
      putStrLn $ "Usage: " ++ progName ++ " <input>"
      return ()
    else mapM_ processInputFile commandLineArgs

processInputFile :: FilePath -> IO ()
processInputFile inputFilePath = do
  fileExists <- doesFileExist inputFilePath
  if not fileExists
    then return ()
    else do
      contents <- readFileSafe inputFilePath
      case contents of
        Left _ -> return ()
        Right fileContents -> do
          let formattedLines = map formatLine (lines fileContents)
          let tempFilePath = takeDirectory inputFilePath ++ "/" ++ takeBaseName inputFilePath ++ ".tmp"
          writeFile tempFilePath (unlines formattedLines)
          renameResult <- tryRename tempFilePath inputFilePath
          case renameResult of
            Left _ -> removeFileSafe tempFilePath
            Right _ -> return ()

readFileSafe :: FilePath -> IO (Either String String)
readFileSafe filePath = catch (Right <$> readFile filePath) handleError
  where
    handleError :: SomeException -> IO (Either String String)
    handleError _ = return (Left "Error reading file")

tryRename :: FilePath -> FilePath -> IO (Either String ())
tryRename tempFilePath originalFilePath = catch (Right <$> renameFile tempFilePath originalFilePath) handleError
  where
    handleError :: SomeException -> IO (Either String ())
    handleError _ = return (Left "Error renaming file")

removeFileSafe :: FilePath -> IO ()
removeFileSafe filePath = catch (removeFile filePath) handleError
  where
    handleError :: SomeException -> IO ()
    handleError _ = return ()
