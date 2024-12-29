## Browser Rust
「[作って学ぶ]ブラウザの仕組み」を参考にRustで作成した自作ブラウザ。

https://github.com/d0iasm/saba

https://github.com/hikalium/wasabi

### QEMUの起動
`./run_on_wasabi.sh`実行前に以下を実行するとQEMUのウインドウが出てくるようになる。
`export DISPLAY=0`

https://github.com/d0iasm/sababook/issues/2

### テスト用サーバーの起動
`python3 -m http.server 8000`
`test.html`を返すサーバーを起動。