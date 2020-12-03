# generate g authenticator code and svg

Example usage to generate a Google Authenticator token of length 128 and SVG, with width of 300 px by heigh 300px,
with a name "Some Service" and title "username", 

```bash
$ gauth-generator -l 128 --qr -n "Some Service" -t "username" -w 300 -h 300 --ecl Low 
```

For more information show help:

```bash
$ gauth-generator --help
```

You can also validate whether this works correctly by running:

```bash
$ gauth-generator validate KJHASKAJSHLKJASHLAKSJHALS 1111
```


This was a project to learn how to make CLI apps with Rust. Please use with caution.

https://www.rust-lang.org/what/cli
https://github.com/killercup/quicli
https://docs.rs/structopt/