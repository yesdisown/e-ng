# e-ng

The successor to [BrutBurger](https://github.com/brutburger)'s [Project E](https://github.com/brutburger/e). A C++ obfuscator written in Rust.
Use it to ragebait people on help forums.

## Installation

From crates.io:

```bash
cargo install e-ng
```

From source:

```bash
git clone https://github.com/yesdisown/e-ng
cd e-ng
cargo install --path .
```

## Usage

Obfuscate a file using the default charset (`ae`):

```bash
e-ng input.cpp output.cpp
```

Pipe via `stdin` and `stdout`:

```bash
cat input.cpp | e-ng --no-shuffle > output.cpp
```

Use a custom phrase dictionary file (one phrase per line):

```bash
e-ng input.cpp output.cpp --phraseset phrases.txt
```

## Examples

### Example 1 (Hello world)
```c++
// in.cpp
#include <iostream>

int main() {
  std::cout << "Hello, world!";
}
```
upon e-ng'ing with the default settings ( `e-ng in.cpp out.cpp`), will produce the following code:
```c++
// out.cpp
#include <iostream>
#define ea "Hello, world!"
#define a int
#define ae cout
#define e main
#define aa std

a e() {
  aa :: ae << ea;
}
```
### Example 2 (codeforces 4A - [Watermelon](https://codeforces.com/problemset/problem/4/A))
```c++
// in.cpp
#include <bits/stdc++.h>
using namespace std;
 
void solve() {
  int w;
  cin >> w;
  if (w % 2 == 0 && w > 2) cout << "YES";
  else cout << "NO";
}
 
int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(0x0);
  solve();
  return 0;
}
```
lets experiment a bit. This time we'll use a custom phraseset
```bash
echo zap\nzip\nzop > dict.txt
e-ng in.cpp out.cpp -p dict.txt
```
```c++
// out.cpp
#include <bits/stdc++.h>
#define zap using
#define zopzap 2
#define zapzipzip ios_base
#define zipzip cin
#define zapzipzap main
#define zapzap void
#define zip namespace
#define zipzop if
#define zapzapzop "NO"
#define zapzop int
#define zapzipzop sync_with_stdio
#define zipzapzap return
#define zapzip solve
#define zopzip 0
#define zapzapzap "YES"
#define zapzapzip else
#define zapzopzap false
#define zapzopzip tie
#define zipzap w
#define zapzopzop 0x0
#define zopzop cout
#define zop std

zap zip zop;
zapzap zapzip() {
  zapzop zipzap;
  zipzip >> zipzap;
  zipzop(zipzap % zopzap == zopzip && zipzap > zopzap) zopzop << zapzapzap;
  zapzapzip zopzop << zapzapzop;
}
zapzop zapzipzap() {
  zapzipzip :: zapzipzop(zapzopzap);
  zipzip . zapzopzip(zapzopzop);
  zapzip();
  zipzapzap zopzip;
}
```
an [absolute masterpiece](https://open.spotify.com/album/59J51uy6r6QcYe7cX0Fzz6?autoplay=true).

## Options

* `-c`, `--charset <CHARSET>` – character set used to generate macro names (default: `ae`).
* `-p`, `--phraseset <PHRASESET>` – path to a dictionary file with phrases separated by newlines.
* `--no-shuffle` – keep the generated `#define` lines in order.
* `-v`, `--verbose` – print status info to `stderr`.
* `-h`, `--help` – show help.
* `-V`, `--version` – show version.

## License

MIT
