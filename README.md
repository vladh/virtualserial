# virtualserial

This package allows you to create virtual serial ports on macOS. It uses socat, and is successor of sorts to [MacOSXVirtualSerialPort](https://github.com/clokey/PublicCode/tree/master/MacOSXVirtualSerialPort).

## Installation

Until it makes sense to put `virtualserial` on Homebrew, I've included a precompiled binary. You just need to install socat.

```
brew install socat
```

Then clone this repo and:

```
make install
```

## Usage

See `virtualserial help`. Here are some basics.

Create a new pair as follows:

```
$ virtualserial create master slave 9600
```

You can then see the two serial ports using `ls`:

```
$ ls -la
lrwxr-xr-x   1 vladh  staff    12B Sep 18 16:38 master -> /dev/ttys002
lrwxr-xr-x   1 vladh  staff    12B Sep 18 16:38 slave -> /dev/ttys003
```

You can create multiple pairs and keep track of them.

```
$ virtualserial show
Current instances:
[15817], master -> slave at baud 9600<Paste>
```

Kill that pair:

```
$ virtualserial kill <pid>
Instance killed: 15817
```

_Note:_ It looks like you can't create things in `/dev/` on the latest version of macOS, so it's recommended that you create the serial ports anywhere else.
