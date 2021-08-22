# Word Counter
A program that displays word count of a file. 

# Example
## Usage
```console
$ ./word_counter poem.txt --top 4 --search lover
+---------------+-------+---+
| Search result | lover | 3 |
+---------------+-------+---+
+---------------+-----+---+
| Maximum count | the | 4 |
+---------------+-----+---+
+-------+-------+
| Word  | Count |
+-------+-------+
| the   | 4     |
+-------+-------+
| and   | 3     |
+-------+-------+
| God   | 3     |
+-------+-------+
| lover | 3     |
+-------+-------+
```

## Options

```console
$ ./word_counter --help
A program that displays word count of a file.

USAGE:
    word_counter [FLAGS] [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -r, --reverse    Reverse order
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output file
    -s, --search <search>    Search specific word
    -t, --top <top>           [default: 10]

ARGS:
    <input>    Input file
```
