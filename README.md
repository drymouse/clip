# ファイルの内容をクリップボードに貼り付ける
~~現在は標準入力のみ~~
=> ファイルを指定してコピーができるようになった。

## インストール
```
cargo install --path .
```

## 使い方
```
$ clip # input from stdin
$ clip file # input from "file"
$ cat file | clip # almost same with the second
```