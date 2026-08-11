# Nanahoshi

Nanahoshi is a small x86-64 ELF loader written in Rust.

ELF64 の構造を理解しながら、ELF Header / Program Header を自前で解析し、
最終的に `PT_LOAD` segment をメモリへ配置して Entry Point へ制御を移すことを目標とする。

## Goal

```text
ELF file
   |
   v
ELF Header
   |
   v
Program Header Table
   |
   v
PT_LOAD
   |
   v
Load segments into memory
   |
   v
Entry Point
```

最終的には、次のような情報を ELF から取得・利用できる Loader を実装する。

```text
$ cargo run -- sample/kernel.elf

ELF64
Machine: x86-64
Entry point: 0x...
Program Header Offset: 0x...
Program Header Count: ...
```

## Current Status

- [x] ELF ファイルを `Vec<u8>` として読み込む
- [x] ELF Magic (`\x7fELF`) を検証する
- [x] ELF64 Header の各 field を解析する
- [x] Little Endian の `u16` / `u32` / `u64` を読み取る
- [x] `e_type`, `e_machine`, `e_entry`, `e_phoff` などを構造体に保持する
- [ ] ELF64 / Little Endian / Version の validation を完成させる
- [ ] Program Header Table を解析する
- [ ] `PT_LOAD` segment を抽出する
- [ ] segment を疑似メモリへ配置する
- [ ] `p_memsz > p_filesz` の領域を zero-fill する
- [ ] 実メモリへロードする
- [ ] ELF Entry Point へ制御を移す

## Supported Format

現時点では、対象を意図的に絞っている。

- ELF64
- Little Endian
- x86-64
- static / PIE ELF を学習対象として使用

ELF32、Big Endian、Dynamic Linker、GOT / PLT、shared library などは現時点では対象外。

## Project Structure

```text
nanahoshi/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── docs/
│   └── elf-header.md
├── sample/
├── src/
└── target/
```

今後は以下のようにドキュメントを追加する予定。

```text
docs/
├── elf-header.md
├── program-header.md
└── loading.md
```

## Documents

- [ELF Header](docs/elf-header.md)

## Sample ELF

macOS で通常の Rust executable を生成すると Mach-O になるため、
ELF の学習用サンプルには `x86_64-unknown-none` を使用する。

```bash
rustup target add x86_64-unknown-none
```

Object File:

```bash
rustc \
    --target x86_64-unknown-none \
    --crate-type=lib \
    --emit=obj \
    sample/sample.rs \
    -o sample/sample.o
```

Executable ELF:

```bash
rustc \
    --target x86_64-unknown-none \
    -C panic=abort \
    sample/kernel.rs \
    -o sample/kernel.elf
```

確認:

```bash
file sample/sample.o
file sample/kernel.elf
xxd -l 64 sample/kernel.elf
```

## Reference

- System V ABI: ELF Header  
  https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html
