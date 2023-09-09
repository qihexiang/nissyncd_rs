# NISSYNCD: Automatically rebuild NIS database (in Rust)

The program is designed for computer clusters which share users and groups by NIS, which could watch for `/etc/passwd`, `/etc/shadow`, `/etc/group` and `/etc/gshadow` changing and run `make` in NIS directory automatically.

This program requires you to install `make` before use it.

## Environment variables

You can set a environment variables named `YP_DIRECTORY` to specify the NIS database directory. Default value is `/var/yp`.

## Build & Release

It's recommended to build the source code to `x86_64-unknown-linux-musl` target, that will make the program statically linked, and you could run it everywhere. The file provided in release page is compile to this target.

## systemd intergration

```ini
# /etc/systemd/system/nissyncd.service
[Unit]
Description=Rebuild NIS database automatically.

[Service]
Type=simple
# assumes that put the binary file in /usr/local/bin
ExecStart=/usr/local/bin/nissyncd

[Install]
WantedBy=multi-user.target
```
