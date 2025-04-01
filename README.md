# EZStream

## Building from Source

**Requirements**

- GTK 4.12+
- Rust (I used 1.84, anything that supports the `2021` edition of Rust should work.)

### Arch Linux

#### Install Build Dependencies

```sh
sudo pacman -Syu & sudo pacman -S gtk4
```

#### Compile and Run
```sh
./build.sh build
cd release
./EZStream
```
