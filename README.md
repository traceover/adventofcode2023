# adventofcode2023
My solutions to https://adventofcode.com/ for 2023.

## Quick Start

To run my solutions to each day of the advent calendar,
make sure you have [Rust](https://www.rust-lang.org/learn/get-started)
installed on your machine and available in path.

Then run:

```console
$ cargo run --release
   Compiling adventofcode2023 v0.1.0 (C:\Users\YourName\Documents\Github\adventofcode2023)
    Finished release [optimized] target in 2.18s
     Running 'target\release\adventofcode2023.exe'
Please enter your session cookie:
```

### How do I get my session cookie?

The program looks for a folder called `inputs` by default (but can be overrided with command-line switch)
which stores a list of `.txt` files containing the inputs for each day (such as `1.txt`, `2.txt`, ...).
You can manually download each individual day's input, or, you can log into [adventofcode](https://adventofcode.com/),
inspect the browser (Right-Click on the page > Inspect > Application/Storage > Cookies) and copy the cookie
named `session` (should be a long, random hexadecimal number) and paste that into the prompt, and the program
will download your inputs automatically.
