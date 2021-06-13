Note: Japanese text only.

# 落下物よけゲー: falls
降ってくる物をひたすら避けるゲームです。  
画面上部に「バリヤー」というゲージが表示されるので  
それがゼロになるとゲームオーバー。  
生存秒数が右上に表示されています。  
自分では30秒くらいが精一杯でした。  
動画はtwitterに上げました。  
https://twitter.com/hyoikaz/status/1401499116627058691
## 操作方法
カーソルキーで上下左右に移動。斜め移動もします。  
Escで一時停止。  
縦画面なせいで使えなさそうですが、一応Alt＋Enterでフルスクリーンとウインドウモード切替。  
## WASM＜仮対応＞版
※遠くの星がなくなり背景が真っ黒になります  
https://hyoi.github.io/falls/
## Rustのコンパイル版
[bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template)をお借りしたので、cargo-makeを使います。   
```
cargo make --profile release run    
```
WASM版の場合は、
```
cargo make --profile release serve
```
※事前にRustのtargetの追加とか必要です、たぶんきっとおそらく
## お世話になりました
- [bevy 0.5](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy_prototype_lyon 0.3.1](https://github.com/Nilirad/bevy_prototype_lyon/)
  - [bevy_canvas 0.1.0](https://github.com/Nilirad/bevy_canvas)
  - [heron 0.8.0](https://github.com/jcornaz/heron/)
  - [Unofficial Bevy Cheat Book](https://github.com/bevy-cheatbook/bevy-cheatbook)
  - [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template)
- [Google Fonts](https://fonts.google.com/)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One?subset=japanese)
## 問題・宿題
- WASMに対応したい ⇒ 仮対応版を公開：背景の星が表示されない
- 背景の星が落下物の手前に描画されることがある。それなりの頻度で。治したい
