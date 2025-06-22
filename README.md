# cargo-acj (AtCoderJudge)

## 概要

AtCoderのサンプルケースを自動で実行できるRustのサブコマンドです。

## install

```
cargo install cargo-acj
```

## 使い方

```
cargo acj <contestName> <problemId> --bin <bin> --tle <tleSec>
```

| arg         | 概要                                              |
| ----------- | ------------------------------------------------- |
| contestName | コンテスト名 (abc123, typical90など)              |
| problemId   | 問題のID(A問題だったらa, Ex問題だったらhなど)     |
| bin(任意)   | 実行するbin(指定しなければmainで実行)             |
| tle(任意)   | 実行を打ち切る時間(指定しなければ2秒が指定される) |
