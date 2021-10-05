## todo app
vueとactix-webの勉強として作成したtodo_appです．todoデータの保存にpostgresqlをコメントデータの保存にmongodbを利用しています．ロングポーリングを実装しています．

<img src="https://dl.dropboxusercontent.com/s/rpjjyr17jf642ec/ui%E7%94%BB%E5%83%8F.png">

### フロントエンド
```
cd ui
npm install
npm run build
```

```
cd ..
```

### サーバー
1. 環境変数`DATABASE_URL`,`MONGODB_URL`,`INDEX_DIR_PATH`を設定する．
1. postgresql, mongodbのサービスを起動する．
1. diesel cliをインストール
```
cd server
diesel migration run
cargo run --bin build_server
```

そして`http://localhost:8080/public/index.html`にアクセスする．

