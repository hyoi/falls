Note: Japanese text only.

# 落下物よけゲー: falls
降ってくる物をひたすら避けるゲームです。  
画面上部にバリヤーゲージが表示されるので  
それがゼロになるとゲームオーバー。  
生存秒数が右上に表示されています。  
Windowsアプリだと自分では30秒くらいが精一杯でした。  
WASMだとフレーム落ちがすごくて  
しばらく死なずに遊んでられます (^_^;) 。  
## WASM版
https://hyoi.github.io/falls/  
※えらくフレーム落ちて、自分の環境だと10FPSくらいだった。  
※障害物も20～30個くらいに減って緊張感ありません。
## 操作方法
`⇧` `⇩` `⇦` `⇨` キーで上下左右に移動。同時押しで斜め移動も。  
`Esc`キーで一時停止(Pause)。  
`Space`キーでゲーム開始など。  
縦画面なので使えなさそうですが`Alt`＋`Enter`でフルスクリーンとウインドウモード切替。  
## コンパイル方法
デスクトップアプリにするなら `cargo run`でOK。
```
cargo run -r    
```
WASMの場合は、bevy 0.6 から bevy_webgl2 に頼らなくても良くなりました。
```
cargo build -r --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target --target web --no-typescript ./target/wasm32-unknown-unknown/release/falls.wasm
```
※`wasm-bindgen`コマンドの各ディレクトリーは作業環境に合わせてください。   
※WASMのコンパイルには事前にRustのtargetの追加とwasm-bindgenのインストールが必要です。  
※wasm-bindgenを実行するとバージョン違いで警告が出ることがあります。その時は素直にバージョン上げましょう。  
```
rustup target install wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```
　[Unofficial Bevy Cheat Book - 13.5. Browser (WebAssembly)](https://bevy-cheatbook.github.io/platforms/wasm.html)をご参考に。
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [heron](https://github.com/jcornaz/heron/)
  - [bevy-web-resizer](https://github.com/frewsxcv/bevy-web-resizer)
  - [Unofficial Bevy Cheat Book](https://github.com/bevy-cheatbook/bevy-cheatbook)
- [Google Fonts](https://fonts.google.com/)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One?subset=japanese)
## 問題・宿題
- ~~WASMに対応したい ⇒ 仮対応版を公開：背景の星が表示されない~~
- ~~背景の星が落下物の手前に描画されることがある。それなりの頻度で。治したい~~
- WASMのフレーム落ち、もちょっと何とかならないかしら
- 音を鳴らしたい。beep音でいいから
- 全部 なおしたい なおしたい 病（リファクタリングにいたる病）