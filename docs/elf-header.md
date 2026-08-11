# ELF Header

Nanahoshi で ELF Loader を実装するために必要な、
ELF64 Header の構造と現在の実装内容をまとめる。

> この文書では **ELF64 / Little Endian / x86-64** を対象とする。

## 1. ELF Header の役割

ELF ファイルの先頭には ELF Header があり、ファイル全体を解釈するための情報が格納されている。

今回の Loader では特に次の情報が重要になる。

- ELF64 か
- Little Endian か
- 対象 architecture は何か
- Entry Point はどこか
- Program Header Table はどこにあるか
- Program Header は何個あるか

```text
ELF file
+-------------------------+ 0x00
| ELF Header              |
+-------------------------+
| Program Header Table    |
+-------------------------+
| ...                     |
+-------------------------+
```

## 2. ELF Magic

ELF ファイルの先頭 4 bytes は Magic Number になっている。

```text
7f 45 4c 46
```

文字として表すと、

```text
\x7fELF
```

内訳:

| Byte   | 意味 |
| ------ | ---- |
| `0x7f` | DEL  |
| `0x45` | `E`  |
| `0x4c` | `L`  |
| `0x46` | `F`  |

Parser では最初にこの値を検証する。

```rust
if data[EI_MAG0] != 0x7f
    || data[EI_MAG1] != b'E'
    || data[EI_MAG2] != b'L'
    || data[EI_MAG3] != b'F'
{
    panic!("not an ELF file");
}
```

## 3. ELF64 Header の構造

ELF64 Header は以下の構造を持つ。

```c
typedef struct {
        unsigned char   e_ident[EI_NIDENT];
        Elf64_Half      e_type;
        Elf64_Half      e_machine;
        Elf64_Word      e_version;
        Elf64_Addr      e_entry;
        Elf64_Off       e_phoff;
        Elf64_Off       e_shoff;
        Elf64_Word      e_flags;
        Elf64_Half      e_ehsize;
        Elf64_Half      e_phentsize;
        Elf64_Half      e_phnum;
        Elf64_Half      e_shentsize;
        Elf64_Half      e_shnum;
        Elf64_Half      e_shstrndx;
} Elf64_Ehdr;
```

### 型のサイズ

| ELF type        |    Size |
| --------------- | ------: |
| `unsigned char` |  1 byte |
| `Elf64_Half`    | 2 bytes |
| `Elf64_Word`    | 4 bytes |
| `Elf64_Addr`    | 8 bytes |
| `Elf64_Off`     | 8 bytes |

ELF 仕様では field の順序と型が定義されているため、
各 field の offset は型のサイズを足して求められる。

例:

```text
e_ident = 16 bytes
             |
             v
e_type starts at 0x10

e_type = 2 bytes
             |
             v
e_machine starts at 0x12
```

## 4. ELF64 Header の offset

| Offset | Size | Field         | Type                |
| -----: | ---: | ------------- | ------------------- |
| `0x00` |   16 | `e_ident`     | `unsigned char[16]` |
| `0x10` |    2 | `e_type`      | `Elf64_Half`        |
| `0x12` |    2 | `e_machine`   | `Elf64_Half`        |
| `0x14` |    4 | `e_version`   | `Elf64_Word`        |
| `0x18` |    8 | `e_entry`     | `Elf64_Addr`        |
| `0x20` |    8 | `e_phoff`     | `Elf64_Off`         |
| `0x28` |    8 | `e_shoff`     | `Elf64_Off`         |
| `0x30` |    4 | `e_flags`     | `Elf64_Word`        |
| `0x34` |    2 | `e_ehsize`    | `Elf64_Half`        |
| `0x36` |    2 | `e_phentsize` | `Elf64_Half`        |
| `0x38` |    2 | `e_phnum`     | `Elf64_Half`        |
| `0x3a` |    2 | `e_shentsize` | `Elf64_Half`        |
| `0x3c` |    2 | `e_shnum`     | `Elf64_Half`        |
| `0x3e` |    2 | `e_shstrndx`  | `Elf64_Half`        |

ELF64 Header 全体は `0x40 = 64 bytes`。

## 5. `e_ident`

先頭 16 bytes の `e_ident` は、残りの ELF をどのように解釈するかを示す。

```text
Offset  Field
0x00    EI_MAG0
0x01    EI_MAG1
0x02    EI_MAG2
0x03    EI_MAG3
0x04    EI_CLASS
0x05    EI_DATA
0x06    EI_VERSION
0x07    EI_OSABI
0x08    EI_ABIVERSION
0x09-
0x0f    padding
```

今回の ELF:

```text
7f 45 4c 46 02 01 01 00 00 ...
```

主な値:

### `EI_CLASS`

```text
1 = ELF32
2 = ELF64
```

今回は `2` なので ELF64。

### `EI_DATA`

```text
1 = Little Endian
2 = Big Endian
```

今回は `1` なので Little Endian。

## 6. Little Endian の読み取り

今回の ELF は Little Endian なので、

```text
03 00
```

は、

```text
0x0003
```

として読む。

Rust では `from_le_bytes` を使用する。

```rust
fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([
        data[offset],
        data[offset + 1],
    ])
}

fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
    ])
}

fn read_u64_le(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}
```

## 7. 現在の Parser の構造

ELF Header の field offset は定数として保持する。

```rust
// ELF Identification
const EI_MAG0: usize = 0;
const EI_MAG1: usize = 1;
const EI_MAG2: usize = 2;
const EI_MAG3: usize = 3;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_OSABI: usize = 7;
const EI_ABIVERSION: usize = 8;

// ELF Header
const E_TYPE: usize = 0x10;
const E_MACHINE: usize = 0x12;
const E_VERSION: usize = 0x14;
const E_ENTRY: usize = 0x18;
const E_PHOFF: usize = 0x20;
const E_SHOFF: usize = 0x28;
const E_FLAGS: usize = 0x30;
const E_EHSIZE: usize = 0x34;
const E_PHENTSIZE: usize = 0x36;
const E_PHNUM: usize = 0x38;
const E_SHENTSIZE: usize = 0x3a;
const E_SHNUM: usize = 0x3c;
const E_SHSTRNDX: usize = 0x3e;
```

解析結果は Rust の型へ変換して保持する。

```rust
#[derive(Debug)]
pub struct ElfIdent {
    class: u8,
    data: u8,
    version: u8,
    osabi: u8,
    abi_version: u8,
}

#[derive(Debug)]
enum ElfType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Unknown(u16),
}

#[derive(Debug)]
enum ElfMachine {
    X86_64,
    Unknown(u16),
}

#[derive(Debug)]
enum ElfVersion {
    None,
    Current,
    Unknown(u32),
}

#[derive(Debug)]
pub struct Elf64Ehdr {
    e_ident: ElfIdent,
    e_type: ElfType,
    e_machine: ElfMachine,
    e_version: ElfVersion,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}
```

### `e_type`

```text
0 = ET_NONE
1 = ET_REL
2 = ET_EXEC
3 = ET_DYN
4 = ET_CORE
```

```rust
let raw_type = read_u16_le(data, E_TYPE);

let e_type = match raw_type {
    0 => ElfType::None,
    1 => ElfType::Rel,
    2 => ElfType::Exec,
    3 => ElfType::Dyn,
    4 => ElfType::Core,
    value => ElfType::Unknown(value),
};
```

未知の値は `Unknown(value)` として元の値を残す。

## 8. `kernel.elf` を実際に読む

生成した ELF:

```text
00000000: 7f45 4c46 0201 0100 0000 0000 0000 0000
00000010: 0300 3e00 0100 0000 8012 0000 0000 0000
00000020: 4000 0000 0000 0000 2005 0000 0000 0000
00000030: 0000 0000 4000 3800 0800 4000 0e00 0c00
```

主な field:

| Field         | Raw                       | Value         |
| ------------- | ------------------------- | ------------- |
| `e_type`      | `03 00`                   | `ET_DYN`      |
| `e_machine`   | `3e 00`                   | `62 = x86-64` |
| `e_version`   | `01 00 00 00`             | `1`           |
| `e_entry`     | `80 12 00 00 00 00 00 00` | `0x1280`      |
| `e_phoff`     | `40 00 00 00 00 00 00 00` | `0x40`        |
| `e_shoff`     | `20 05 00 00 00 00 00 00` | `0x520`       |
| `e_ehsize`    | `40 00`                   | `64`          |
| `e_phentsize` | `38 00`                   | `56`          |
| `e_phnum`     | `08 00`                   | `8`           |
| `e_shentsize` | `40 00`                   | `64`          |
| `e_shnum`     | `0e 00`                   | `14`          |
| `e_shstrndx`  | `0c 00`                   | `12`          |

重要なのは以下。

```text
e_entry     = 0x1280
e_phoff     = 0x40
e_phentsize = 56
e_phnum     = 8
```

Program Header Table は offset `0x40` から始まり、
56 bytes の Program Header が 8 個存在する。

## 9. Object File と Executable ELF

### `sample.o`

```text
e_type  = ET_REL
e_entry = 0
e_phoff = 0
e_phnum = 0
```

Object File はリンク前のため、今回のサンプルでは実行時のロードに使う
Program Header Table を持っていない。

### `kernel.elf`

```text
e_type      = ET_DYN
e_entry     = 0x1280
e_phoff     = 0x40
e_phentsize = 56
e_phnum     = 8
```

`file` では `pie executable` と表示されるが、
PIE のため ELF Header の `e_type` は `ET_DYN` になっている。

## 10. 実装時に間違えた点

### `e_type` を 1 byte として読んでいた

最初は、

```rust
data[E_TYPE]
```

としていた。

しかし `e_type` は `Elf64_Half` なので 2 bytes。

```rust
read_u16_le(data, E_TYPE)
```

と読む必要がある。

### Header offset を field 番号のように考えていた

各 field の開始位置は、前の field のサイズを考慮して決まる。

```text
e_type
offset = 0x10
size   = 2

        |
        v

e_machine
offset = 0x12
```

### `e_version` は 4 bytes

`e_version` は `Elf64_Word` なので `u32`。

```rust
read_u32_le(data, E_VERSION)
```

として読む。

### `EI_VERSION` と `e_version` は別

```text
e_ident[EI_VERSION]
```

は 1 byte。

```text
e_version
```

は `Elf64_Word` なので 4 bytes。

### File Offset と Virtual Address は別

Program Header で、

```text
p_offset = 0x1000
p_vaddr  = 0x400000
```

なら、

```text
ELF file offset 0x1000
        |
        | load
        v
virtual address 0x400000
```

という意味になる。

この違いは `PT_LOAD` を実装するときに重要。

## 11. 次の Step

ELF Header の解析はここで一旦区切る。

次は、

```text
e_phoff
e_phentsize
e_phnum
```

を使い Program Header Table を走査する。

今回の ELF では、

```text
e_phoff     = 0x40
e_phentsize = 56
e_phnum     = 8
```

なので、

```text
Program Header #i offset
    = e_phoff + e_phentsize * i
```

となる。

次に解析する field:

```text
p_type
p_flags
p_offset
p_vaddr
p_paddr
p_filesz
p_memsz
p_align
```

ここから `PT_LOAD` を探し、実際の ELF Loader の処理へ進む。

## Reference

- System V ABI: ELF Header  
  https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html
