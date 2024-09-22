# mysqlの起動
```
# docker-compose.ymlに記載しているmysqlをバックグラウンドで起動
docker compose up -d

# mysqlを終了（volumeの設定をしていないのでデータごと消えるので注意）
docker compose down
```
