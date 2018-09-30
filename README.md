This is an actix-web server Rust app that is supposed to be run on the [Mozilla Flame phone](https://wiki.postmarketos.org/wiki/Mozilla_Flame_(t2m-flame)).
# Prerequisites (Arch Linux)

## Setup `postmarketOS`

Follow [this blog post](https://sizeof.cat/post/postmarketos-on-mozilla-flame/) to install `postmarketOS` to your Flame device.

Don't select UI (set `none` when asked) and choose the following extra packages: `vim,git,nodejs,nodejs-npm,yarn,parted,gcc`.

Also see [device specific recipes](https://gitlab.com/postmarketOS/pmaports/tree/master/device/device-t2m-flame) in `postmarketOS` repo.


## Setup ARM toolchain

Install [arm-linux-gnueabihf-gcc](https://aur.archlinux.org/packages/arm-linux-gnueabihf-gcc). See pinned comment for the correct order.

Install [arm-linux-gnueabihf-musl](arm-linux-gnueabihf-musl).

## Setup Rust

Add Rust targets: `armv7-unknown-linux-gnueabihf` (likely not needed) and `armv7-unknown-linux-musleabihf`:

```bash
$ rustup target add armv7-unknown-linux-gnueabihf armv7-unknown-linux-musleabihf
```

## Build Rust binary

Point Cargo to the right linker:

```bash
$ export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc
```

If C dependencies are involved export `CC_armv7_unknown_linux_musleabihf`:

```bash
$ export CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc
```

Build the debug/release binary:

```bash
$ cargo build [--release] --target=armv7-unknown-linux-musleabihf
```

## Deploy binary to the Flame device

Secure-copy compiled binary to Flame (choose `debug` or `release` in the path below):

```bash
$ scp ./target/armv7-unknown-linux-musleabihf/[debug]|[release]/rust-app user@172.16.42.1:/home/user
```

## Run and test

SSH into the Flame:

```bash
$ ssh user@172.16.42.1
```

Run the web server app:
```bash
$ /home/user/rust-app
```

On host machine open `http://172.16.42.1:8080/postmarketos/flame/index.html` in the browser and you should see:

```
Hello from postmarketos on the flame device!
```
