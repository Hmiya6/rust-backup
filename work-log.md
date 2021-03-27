
## architecture
```
* App
    * run()
    * DebugInterface
        * output result/progress to log file and stderr
    * Config
        * src
        * dst
        * .zip or .tar
        * backup interval
        * new or replace ?
        * name: ("{}_{}.zip", name, backup_time)
    * Compressor
        * src
        * name
    * FileManipurator
        * src
        * dst
        * move_file()
    * API
        * diff the file system
        * compress the file system
        * transfer file
    * snapshot (hashed value)
    * 
```

## Config ファイルの読み込み
`.toml` ファイルを読み込ませて設定を行う.  
toml クレートと serde クレートを用いた.  

## ファイル分割

#### 1

このファイル構造のとき、foo.rs から bar.rs の構造体を読み込みたい
```
src/
├─ foo.rs
├─ bar.rs
├─ main.rs
```

main.rs か lib.rs に以下を書く
```rust
// main.rs or lib.rs

mod foo; // foo.rs を読み込む
pub use foo::Foo; // 構造体のみを公開

mod bar; // bar.rs を読み込む
pub use bar::Bar; // 構造体のみを公開
```
ソースコード全体で読み込むことができるようになる

foo.rs に以下を書く.
```rust
// main.rs (or lib.rs) で読み込んだ構造体を読み込むことが可能.
use crate::Bar;

```

#### 2
モジュール自体を公開することも可能
```rust
// main.rs (lib.rs)
pub mod bar; // モジュールを公開

```

```rust
// foo.rs
use crate::bar::Bar; // 構造体をインポート.

```

## テストを書く

```rust
// cargo test のときだけコンパイルが行われる
#[cfg(test)] 
mod tests {

    // ファイル内の機能を test モジュールにインポート
    use super::*; 

    // test 関数を示す注釈 
    // tests モジュールの中にはテスト関数以外の関数をいれることができるため、識別のために必要
    #[test] 
    fn test_something() {
        // do something
    }
}

```


## ディレクトリの差分をとる
ディレクトリのハッシュをとりたい  
なので、ファイルのハッシュをつなげてディレクトリハッシュの入力にする.  
ファイルのハッシュは、その内容だけでなくそのパスも入力としてハッシュとする (名前の変更に対応)

## zip 圧縮

zip クレートを使う.
* [docs.rs](https://docs.rs/zip/0.5.11/zip/index.html)
* [github.com](https://github.com/zip-rs/zip)



#### 実装
dirwalk をしながらディレクトリとファイルを zip へ書き込んでいく
```rust
// src/backup.rs

    fn archive(&self) -> Result<()> {
        
        // set file name
        let now = Utc::now();
        let file_name = format!("{}/backup_{}.zip", self.dst, now);
        
        // create zip file
        let zip_file = fs::File::create(Path::new(&file_name))?;
        
        // write directory into the zip file
        let mut writer = ZipWriter::new(zip_file);
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);
        // dirwalk
        for entry in WalkDir::new(&self.src) {
            if let Ok(file) = entry {
                // relative path from src
                let path = format!("{}", file.path().display());
                if file.file_type().is_dir() {
                    // add directory into the zip file
                    writer.add_directory(&path, options)?;
                } else {
                    // add file into the zip file
                    writer.start_file(&path, options)?;
                    writer.write_all(&fs::read(file.path())?)?;
                }
            } 
        }
        writer.finish()?;

        // return 
        Ok(())
    }
```

#### コンパイルできない...
以下のエラーコードが出てコンパイルできなかった. 
```
error: linking with `cc` failed: exit code: 1
```
以下を実行し、再ビルドするとコンパイルできた. 
```sh
$ cargo clean
# $ cargo build
```
すると解決した.


## 現在時間の取得
chrono クレートを使う.  
* [docs.rs](https://docs.rs/chrono/0.4.19/chrono/)
* [github.com](https://github.com/chronotope/chrono)



## ディレクトリ内のすべてのファイルを探索する.
#### 1. walkdir クレートを使う
* [docs.rs](https://docs.rs/walkdir/2.3.2/walkdir/index.html)
* [github.com](https://github.com/BurntSushi/walkdir)

```rust
use walkdir::WalkDir;

for entry in WalkDir::new("root") {
    let file = entry.unwrap();
    if file.file_type().is_dir() {
        continue;
    }
    // do something
}
```

#### 2. std::fs::read_dir を使う.
read_dir を再帰的に使うことでファイルの全探索が可能.
* [docs.rs](https://doc.rust-lang.org/std/fs/fn.read_dir.html)


## 
