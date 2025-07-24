# rs-uptime-logger
An uptime logger service written in Rust
<p align="left">
  <a href="https://github.com/artonix101/rs-uptime-logger/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/artonix101/rs-uptime-logger" />
  </a>
</p>

```
cargo build --bin boot
cargo build --bin shutdown

cp target/debug/rs-uptime-boot-logger /usr/local/bin/rs-uptime-boot-logger
cp target/debug/rs-uptime-shutdown-logger /usr/local/bin/rs-uptime-shutdown-logger

touch /var/log/rs-uptime-logger.log
# chown root:root /var/log/rs-uptime-logger.log
# chmod 644 /var/log/rs-uptime-logger.log

cp rs-uptime-boot-logger.service /etc/systemd/system/
cp rs-uptime-shutdown-logger.service /etc/systemd/system/
systemctl enable rs-uptime-boot-logger
systemctl enable rs-uptime-boot-logger
```
