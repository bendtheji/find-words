# find_words
A rust library for finding words that can be constructed from a count-sensitive list of letters.

## Description
Given a list of English words (`words.txt`) and a list of letters (e.g. "yxmiasdaegwyxmiasdaegwyxmiasdaegw"), display all the words that can be constructed from any subset (or all) of those letters in any order.

Note: This list of letters is count-sensitive, i.e. if your input list is 'bde' you can form the word 'bed' but you cannot form the word 'deed' as you only have one 'e'

Assume the goal is to minimize the time between receiving the letters and displaying the possible words.

## Example: bin/find_words.rs

To see an example of the `find_words` program, run the command below:
```sh
cargo run
```
This programs generates a random string of 10 letters, and finds words found in `words.txt` that are constructable from the random string.

## Benchmarks

Ran benchmarks for the word comparison function used to find words that can be constructed from the random string of letters. 

Ran different lengths of the random string of letters (**i.e. 4, 8, 12, 50, 100 , 200**), against different text files containing different number of words (**i.e. 100, 1000, 10000**).

### 100 words
| Length of random string | Lower Bound | Estimate     | Upper Bound |
|-------------------------|-------------|--------------|-------------|
| 4                       | 118.33µs    | **123.56µs** | 129.00µs    |
| 8                       | 99.881µs    | **104.48µs** | 109.92µs    |
| 12                      | 87.258µs    | **87.883µs** | 88.605µs    |
| 50                      | 95.355µs    | **96.014µs** | 96.670µs    |
| 100                     | 94.226µs    | **94.671µs** | 95.137µs    |
| 200                     | 97.001µs    | **98.988µs** | 100.96µs    |

### 1000 words
| Length of random string | Lower Bound | Estimate     | Upper Bound |
|-------------------------|-------------|--------------|-------------|
| 4                       | 277.53µs    | **281.84µs** | 286.66µs    |
| 8                       | 288.03µs    | **295.05µs** | 302.68µs    |
| 12                      | 267.20µs    | **270.95µs** | 275.41µs    |
| 50                      | 219.15µs    | **222.77µs** | 226.67µs    |
| 100                     | 189.71µs    | **192.01µs** | 194.39µs    |
| 200                     | 189.35µs    | **195.02µs** | 204.00µs    |

### 10000 words
| Length of random string | Lower Bound | Estimate     | Upper Bound |
|-------------------------|-------------|--------------|-------------|
| 4                       | 460.32µs    | **469.56µs** | 479.23µs    |
| 8                       | 476.60µs    | **488.60µs** | 500.75µs    |
| 12                      | 506.97µs    | **517.11µs** | 527.34µs    |
| 50                      | 559.80µs    | **577.28µs** | 596.16µs    |
| 100                     | 499.88µs    | **515.65µs** | 534.22µs    |
| 200                     | 498.49µs    | **511.46µs** | 523.73µs    |

To run the benchmarks, simply run the following command:
```sh
cargo bench --bench find_words_benchmark
```

To get more detailed metrics:
```sh
cargo bench --bench find_words_benchmark -- --verbose
```