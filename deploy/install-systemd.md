# Deploy systemd

Esempio minimale su VPS Linux.

```bash
sudo useradd --system --create-home --home-dir /home/swadebot swadebot
sudo mkdir -p /opt/swadedsbot
sudo cp target/release/swadedsbot /opt/swadedsbot/
sudo cp .env.example /opt/swadedsbot/.env
sudo editor /opt/swadedsbot/.env
sudo chown -R swadebot:swadebot /opt/swadedsbot /home/swadebot
sudo cp deploy/swadedsbot.service /etc/systemd/system/swadedsbot.service
sudo systemctl daemon-reload
sudo systemctl enable --now swadedsbot
sudo journalctl -u swadedsbot -f
```
