# rs-uptime-logger
An uptime logger service written in Rust that logs boots, shutdowns and uptime into a simple log file for further use. Command and user that initiated shutdown are also logged. Does not rely on *last* or */var/log/wtmp*, only */proc* and can be used with very minmal linux systems. (Tested on Manjaro, further testing planned)

**⚠️ Warning: This project is in active development and currently in a pre-alpha state. It is not yet stable or ready for production use. Expect rapid changes and broken functionality. Use at your own risk.**

<p align="left">
  <a href="https://github.com/artonix101/rs-uptime-logger/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/artonix101/rs-uptime-logger" />
  </a>
</p>

Build the binaries
```
cargo build --bin boot
cargo build --bin closedown
cargo build --bin wrappers
```
Copy to user-installed bin folder
```
install -m 755 target/debug/{boot,closedown,wrappers} /usr/local/bin/
```
Run install script (copies wrapper binaries to /sbin and renames reboot, shutdown and poweroff to .real) - To undo use restore_originals.sh
```
/scripts/install_wrappers.sh
```
Create log file
```
install -o root -g root -m 644 /dev/null /var/log/rs-uptime-logger.log
```
Copy services to services Folder and reload systemd
```
install -m 644 services/rs-uptime-{boot,closedown}-logger.service /etc/systemd/system/
systemctl daemon-reload
```
Enable at Boot and Closedown (starting rs-uptime-boot-logger.service manually at any time does not write to log)
```
systemctl enable rs-uptime-boot-logger.service
systemctl enable rs-uptime-closedown-logger.service
```
