# rpbcopyd

`rpbcopyd` extends macOS's pasteboard (a.k.a clipboard) to the network-wide level.
You can copy and paste text between different devices on the same network via `rpbcopy` and `rpbpaste` commands.

This is useful to share the pasteboard between your macOS and dev containers.

## Usage

Download binaries from [releases](https://github.com/tk0miya/pbcopyd/releases) and put them into your PATH.

```sh
# Server side
$ curl -L -o rpbcopyd https://github.com/tk0miya/pbcopyd/releases/download/v1.0.0/rpbcopyd-v1.0.0-x86_64-apple-darwin
$ chmod +x rpbcopyd
$ rpbcopyd -d
```

```sh
# Client side
$ curl -L -o rpbcopy https://github.com/tk0miya/pbcopyd/releases/download/v1.0.0/rpbcopy-v1.0.0-x86_64-apple-darwin
$ curl -L -o rpbpaste https://github.com/tk0miya/pbcopyd/releases/download/v1.0.0/rpbpaste-v1.0.0-x86_64-apple-darwin
$ chmod +x rpbcopy rpbpaste
$ echo "Hello, world!" | rpbcopy
$ rpbpaste
```

# License

Apache2.0 License
