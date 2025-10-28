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
  327900 test.txt
```

## Step 2
Number of lines in a file.
```bash
cargo run -- -l test.txt   
     Running `target/debug/rswc -l test.txt`
    7145 test.txt
```
