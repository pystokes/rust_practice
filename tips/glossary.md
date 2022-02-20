# Glossary

## Crate

- いわゆるライブラリ的なもの。

## Library crate

- ルート：src/lib.rs
- 機能群が含まれる
- 他のプログラムで取り込むことに適した機能群を提供するためのクレートで、Cargo.tomlのdependenciesフィールドに追加して使用することができる。いわゆるサードパーティ製のライブラリ的なもの。

## Binary crate

- ルート：src/main.rs
- main関数が含まれる（プログラムのエントリーポイント）
- 実行可能なバイナリを提供するためのクレートで、cargo installでインストールして実行したり、cargo runでローカルでコンパイルして実行したりすることができる。
