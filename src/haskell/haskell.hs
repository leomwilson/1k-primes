sieve (x:xs) = x : sieve (filter (\y -> y `mod` x /= 0) xs)

main = mapM_ print (take 1000 $ sieve [2..])