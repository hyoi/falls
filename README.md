Note: Japanese text only.

# 隕石ゲーム: inseki
降ってくる岩をひたすら避けるゲームです。  
画面上部に「バリヤー」というゲージが表示されるので  
それがゼロになるとゲームオーバー。  
生存秒数が右上に表示されています。  
自分では30秒くらいが精一杯でした。  
動画はtwitterに上げました。  
https://twitter.com/hyoikaz/status/1401499116627058691
## 操作方法
カーソルキーで上下左右に移動。斜め移動もします。  
Escで一時停止。  
Alt＋Enterでフルスクリーンとウインドウモード切替(縦画面なせいか使えなさそう)。  
## Rustのコンパイル
普通にcargo runで。  
```
cargo run --release
```
## お世話になりました
- [bevy 0.5](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy_prototype_lyon 0.3.1](https://github.com/Nilirad/bevy_prototype_lyon/)
  - [bevy_canvas 0.1.0](https://github.com/Nilirad/bevy_canvas)
  - [heron 0.8.0](https://github.com/jcornaz/heron/)
  - [Unofficial Bevy Cheat Book](https://github.com/bevy-cheatbook/bevy-cheatbook)
- [Google Fonts](https://fonts.google.com/)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One?subset=japanese)
## 問題・宿題
- WASMに対応したい
- 背景の星が落下物の手前に描画されることがある。それなりの頻度で。治したい
