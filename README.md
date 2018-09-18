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

Create a serial pair:

```
virtualserial create master slave 9600
```

See the pair you've created:

```
virtualserial show
```

Kill that pair:

```
virtualserial kill <pid>
```

_Note:_ It looks like you can't create things in `/dev/` on the latest version of macOS, so it's recommended that you create the serial ports anywhere else.
