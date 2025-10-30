# rswc
Rust solution to [Coding Challenges](https://codingchallenges.fyi/) [build your own wc](https://codingchallenges.fyi/challenges/challenge-wc) project.

## Step 0

Get the test data file.

```bash
curl -L "https://www.dropbox.com/scl/fi/d4zs6aoq6hr3oew2b6a9v/test.txt?rlkey=20c9d257pxd5emjjzd1gcbn03&dl=0" -o test.txt
```

## Step 1
Number of bytes in the file.
```bash
cargo run -- -c test.txt
     Running `target/debug/rswc -c test.txt`
  342190 test.txt
```

## Step 2
Number of lines in a file.
```bash
cargo run -- -l test.txt   
     Running `target/debug/rswc -l test.txt`
    7145 test.txt
```

## Step 3
Number of words in a file.
```bash
cargo run -- -w test.txt
     Running `target/debug/rswc -w test.txt`
   58164 test.txt
```

## Step 4
Number of characters in a file.
```bash
cargo run -- -m test.txt
     Running `target/debug/rswc -m test.txt`
  339292 test.txt
```

## Step 5
The default option - i.e. no options are provided, which is the equivalent to the -c, -l and -w options.
```bash
cargo run -- test.txt
    7145    58164   342190 test.txt
```

## Step 6
Read from standard input if no filename is specified.
```bash
cat test.txt | cargo run -- -l
     Running `target/debug/rswc -l`
    7145
```

### Testing on Big Files (Over 100 GB)
```bash
seq 1 300000 | xargs -Inone cat test.txt | wc
 2143500000 17449200000 102657000000
seq 1 300000 | xargs -Inone cat test.txt | cargo run -r
     Running `target/release/rswc`
   2143500000   17449200000  102657000000
```
