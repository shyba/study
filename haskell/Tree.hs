data Tree a = Node a (Tree a) (Tree a) | Empty
    deriving Show

example :: Tree Int
example = Node 0 (Node 1 (Node 2 Empty Empty) (Node 3 Empty Empty)) (Node 4 Empty Empty)

treeHeight :: Tree a -> Int
treeHeight Empty = 0
treeHeight (Node _ left right) = 1 + max (treeHeight left) (treeHeight right)

treeLookup :: Ord a => a -> Tree a -> Bool
treeLookup _ Empty = False
treeLookup a (Node b left right) = if a == b then True else (treeLookup a left) || (treeLookup a right)

treeInsert :: Ord a => a -> Tree a -> Tree a
treeInsert a Empty = Node a Empty Empty
treeInsert a (Node b left right) | a > b  = Node b (left) (treeInsert a right)
                                 | a < b  = Node b (treeInsert a left) (right)
                                 | a == b = Node b left right

treeTraverse :: Ord a => Tree a -> [a]
treeTraverse Empty = []
treeTraverse (Node a left right) = a:((treeTraverse left) ++ (treeTraverse right))

main = do
    putStrLn (show example)
    putStrLn (show $ treeLookup 1 example)
    putStrLn (show $ treeLookup 100 example)
    let withHundred = treeInsert 100 example
    putStrLn (show $ treeLookup 100 withHundred)
    putStrLn (show $ treeInsert (0-1) example)
    let values = treeTraverse withHundred
    putStrLn (show $ values)
    let otherTree = foldr treeInsert Empty (reverse values)
    putStrLn (show otherTree)
    putStr (show $ treeHeight example)
