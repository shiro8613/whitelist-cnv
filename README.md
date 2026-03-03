# Whitelist自動生成ツール

このツールを利用するとGoogle Form等で受け取ったシートのcsvから`whitelist.json`を生成できます。

また、フィルタースクリプトからcsvの行番号の指定やTRUE/FALSEでのフィルターもできます。

---

## 使い方

お使いのPCのバージョンにあったファイルを[リリース](https://github.com/shiro8613/whitelist-cnv/releases)からダウンロードします。

csvファイルを用意します。※GoogleFormを利用する場合はシートからcsvとしてダウンロードしてください。

フィルタースクリプトを用意します。※詳細は下

コマンドラインを開きます。

```
Mac/Linux: whitelist-cnv-macos-x64 -i <csvファイル> -f <フィルタースクリプト> -o whitelist.json 
Windows: whitelist-cnv-windows-x64 -i <csvファイル> -f <フィルタースクリプト> -o whitelist.json 
Example: whitelist-cnv-windows-x64 -i users.csv -f filter.cnv -o whitelist.json
```

※x64の部分は大抵のPCではx86です。2020年以降のmacの場合はarm64になります

このコマンドを実行します。

あとは待てば`whitelist.json`が生成されます。

---

## フィルタースクリプトについて

プログラミング言語のように利用可能です。

例は[Exampls](https://github.com/shiro8613/whitelist-cnv/tree/master/examples)にあります。

`data(列番号)` csvの指定された列のデータを取り出します。

`if <条件式> else` 条件を書くことができます。

値を返却する場合はセミコロン(`;`)を付けずに書けば返却されます。

Examplsにある

`conditional_fitler.cnv`は2列目がTRUEのデータのみ、3列目のデータを返します。

`simple_filter.cnv`は1列目のデータをそのまま返します。

※ 返却するのはマインクラフトのユーザー名です。

※ 列番号は0から始まります。

---

## その他

わからないことがあった時は、Githubにある連絡先に連絡していただければ答えます。

知り合いの場合はDMで対応可能です。

---

## ライセンス

MITライセンスにて公開します。

できれば改造・二次配布をする際は、製作者名として`shiro8613`を書いていただけるとうれしいです。