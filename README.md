# rs-uptime-logger
An uptime logger service written in Rust

```
cargo build --release

cp target/release/rs-uptime-logger /usr/local/bin/rs-uptime-logger
# chmod +x /usr/local/bin/rs-uptime-logger.service

touch /var/log/rs-uptime-logger.log
# chown root:root /var/log/rs-uptime-logger.log
# chmod 644 /var/log/rs-uptime-logger.log

cp rs-uptime-logger.service /etc/systemd/system/
systemctl daemon-reexec
systemctl enable rs-uptime-logger
systemctl start rs-uptime-logger
```