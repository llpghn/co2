# Setup Connection with certificates

* https://serverfault.com/questions/241588/how-to-automate-ssh-login-with-password


# Cross-Compile Rust

Setze eine Ubuntu Maschine auf. (15 GB Disk, 2 GB RAM, 1 CPU)
Folge im wesentlichen diesen Schritten: https://disconnected.systems/blog/rust-powered-rover/#cross-compile-hello-world

Anstelle von arm-unknown-linux-gnueabihf nutze arm-unknown-linux-gnueabi
```
apt-get install -qq gcc-arm-linux-gnueabi libc6-armhf-cross libc6-dev-armhf-cross
curl https://sh.rustup.rs -sSf | sh # Or install from your package manager
rustup default stable
rustup target add arm-unknown-linux-gnueabi
```
in der .cargo/config.toml (Ja ich musste das mit .toml noch erst rausbekommen)
```
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```
compile mit: 
```
cargo build --target=arm-unknown-linux-gnueabi
```

und dann übertragen mit 
```
scp /target/arm-unknown-linux-gnueabi/debug/<MODULENAME>
```

-----

cd ~/co2/handler/i2c
git pull

