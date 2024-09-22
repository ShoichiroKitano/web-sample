# migrationの適用
```sh
# プロジェクトのローカルにsqlx-clientをインストール
$ cargo install sqlx-cli --no-default-features --features mysql --root ./tools

# .envに記載されている環境変数を読み込む
$ . .env

# 環境変数DATABASE_URLに設定されている内容に基づいてdatabaseを作成(DATABASE_URLは.envに記載)
$ ./tools/bin/sqlx database create

# migrations直下のsqlをdatabaseに適用
$ ./tools/bin/sqlx migrate run

# 確認(パスワードはpassword)
$ mysql -uroot -h 127.0.0.1 -P 3306 -p
mysql> show databases;
+--------------------+
| Database           |
+--------------------+
| information_schema |
| mysql              |
| performance_schema |
| sample_web         |
| sys                |
+--------------------+
5 rows in set (0.01 sec)

# 利用するDBをsample_webに切り替え
mysql> use sample_web;

# sample_webに登録されているテーブルの一覧を確認
mysql> show tables;
+----------------------+
| Tables_in_sample_web |
+----------------------+
| _sqlx_migrations     | <- sqlxのマイグレーションの適用履歴を管理するテーブル
| samples              | <- migratesディレクトリ内のファイルで定義したテーブル
+----------------------+
2 rows in set (0.01 sec)

# sample_web.samplesのテーブル定義を確認
mysql> show create table sample_web;

# sample_web._sqlx_migrationsのレコードを確認
mysql> select * from _sqlx_migrations;
```

# migratoinの操作
```sh
# .envに記載されている環境変数を読み込む
$ . .env

# 現在適用されているmigrationのバージョンを1つ戻す
$ ./tools/bin/sqlx migrate revert

# 新しいmigraitionファイルの生成
$ ./tools/bin/sqlx migrate add add_hoge_to_fuga
```

# 動作確認
```sh
# .envに記載されている環境変数を読み込む
$ . .env

# 起動
$ cargo run

# POST /samplesを実行 samplesテーブルにname = test1、status = 1なレコードを追加する
$ curl -XPOST localhost:8080 -H "Content-Type: application/json" -d '{"name": "test1", "status": 1 }'
# mysqlで実行結果を確認
mysql > select * from samples;

# GET /samplesを実行
$ curl localhost:8080/samples

# jsonはjqコマンドを使うと見やすい
$ curl localhost:8080/samples | jq .

# HTTPのstatus codeなどをみたい場合は-iオプションをつける
$ curl -i localhost:8080/samples
```

# ソースコードの整形(フォーマット)
```
$ cargo fmt
```

# debug出力
```
# infoを設定するとリクエスト等も出力できる。他にもdebugやerrorなども設定できる。
RUST_LOG=info cargo run
```
