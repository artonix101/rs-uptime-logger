# rs-uptime-logger
An uptime logger service written in Rust that logs boots, shutdowns and uptime into a simple log file for further use. Two systemd services run at boot and shutdown.
<p align="left">
  <a href="https://github.com/artonix101/rs-uptime-logger/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/artonix101/rs-uptime-logger" />
  </a>
</p>

Build the binaries
```
cargo build --bin boot
cargo build --bin shutdown
```
Copy to user-installed bin folder
```
install -m 755 target/debug/{boot,shutdown} /usr/local/bin/
```
Create log file
```
install -o root -g root -m 644 /dev/null /var/log/rs-uptime-logger.log
```
Copy services to services Folder and reload systemd
```
install -m 644 rs-uptime-{boot,shutdown}-logger.service /etc/systemd/system/
systemctl daemon-reload
```
Enable at Boot and Shutdown (starting rs-uptime-boot-logger.service manually at any time does not write to log)
```
systemctl enable rs-uptime-boot-logger.service
systemctl enable rs-uptime-shutdown-logger.service
```
