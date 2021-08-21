# Word Counter
A program that displays word count of a file. 

# Example
## Usage
```console
$ ./word_counter poem.txt --top 6
'the' is counted maximum 4 times
+-------+-------+
| Word  | Count |
+-------+-------+
| the   | 4     |
+-------+-------+
| and   | 3     |
+-------+-------+
| God   | 3     |
+-------+-------+
| to    | 3     |
+-------+-------+
| is    | 3     |
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
    -r, --reverse    
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Output file
    -t, --top <top>           [default: 10]

ARGS:
    <input>    Input file

```
