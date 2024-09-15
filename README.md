# Dirtree

A tool for generating and updating directory structure representations in README files.

## Installation

### macOS (Intel and ARM) and Linux

You can install dirtree with the following command:

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/dirtree/main/install.sh | bash
```

This will download the appropriate binary for your system (including ARM-based Macs) and install it in `/usr/local/bin`.

### Manual Installation

If you prefer to install manually:

1. Download the appropriate binary for your system from the [latest release](https://github.com/yourusername/dirtree/releases/latest):
   - macOS Intel: `dirtree-macos-intel`
   - macOS ARM (M1, M2, etc.): `dirtree-macos-arm`
   - Linux (x86_64): `dirtree-linux`
2. Make the binary executable: `chmod +x dirtree-*`
3. Move the binary to a directory in your PATH and rename it to `dirtree`, e.g., `sudo mv dirtree-* /usr/local/bin/dirtree`

## Usage

```bash
dirtree [OPTIONS] [DIR]
```

Options:
- `-d, --depth <DEPTH>`: Number of subdirectory levels to expand (0 means no limit)

Example:
```bash
dirtree -d 2 ~/projects/my-awesome-project
```

This will generate a directory tree for `~/projects/my-awesome-project` with a depth of 2 levels and update the README.md file in that directory.## Directory Structure

```
📁 directory_structure_generator
   📁 .git
      📁 hooks
         📄 applypatch-msg.sample
         📄 commit-msg.sample
         📄 fsmonitor-watchman.sample
         📄 post-update.sample
         📄 pre-applypatch.sample
         📄 pre-commit.sample
         📄 pre-merge-commit.sample
         📄 pre-push.sample
         📄 pre-rebase.sample
         📄 pre-receive.sample
         📄 prepare-commit-msg.sample
         📄 push-to-checkout.sample
         📄 update.sample
      📁 info
         📄 exclude
      📁 logs
         📁 refs
            📁 heads
               📄 main
            📁 remotes
               📁 personal
                  📄 main
         📄 HEAD
      📁 objects
         📁 09
            📄 7aa125e90cb2bd1494369d8ec7c6d480d68dcf
         📁 0c
            📄 eb9480e57610d226b5a80aa61d397891f9383b
         📁 10
            📄 f364106a3e201d9678f652b656d7a152fd8216
         📁 13
            📄 0b0346a6397d3460abda1e6a3306f1de8cdb2f
         📁 18
            📄 887e11962a74f27196d2fd0485d4ae29786c88
         📁 20
            📄 6357f86760f1e11da3c013974f34342ea8e28f
         📁 23
            📄 c7a34938d22cacfcb0336d7a21d72c7779035e
         📁 24
            📄 1f860ed058ebbd0d063dbbe32e884a3f30e83b
            📄 e1c1d959abb9c2b1c2edb0a3c5455aa6c128ab
         📁 28
            📄 83f4d27b20dbbf74e1b5ac35b3831fc20239db
         📁 2a
            📄 610d07efbc8b19e6df3867c1ca1312ff259093
         📁 2e
            📄 5bb9e60f3d60b71ea26b432c9735714e08fe6e
         📁 32
            📄 f8abcaa176edc6d6f69e9001bc7ba15b3fa601
         📁 33
            📄 6b75db2efbeb57490f73425eeace83ce495097
         📁 3e
            📄 f1f1c7d4efd32c7b777d15a03add9fc691a66f
         📁 45
            📄 1360c7aad1c8d00186948ebd6f3a40b022fb7b
         📁 49
            📄 f1e9358b6e11958b5c70d40497693d46a7ce86
         📁 4f
            📄 b5d02db588d28c822ca05315a9d86020388896
         📁 59
            📄 1349d743c56eea52e468a6995feeb5e322ae49
            📄 7003450b61d02d0c07c9cbb28c291280091387
         📁 5b
            📄 0a594454f6fbd0432e67b1c698bb791bf73395
         📁 6c
            📄 56de7b9a7c681ed48971e4d6b6cdf8729e5716
         📁 70
            📄 6da3f1c85a0b09d1b3deaa8850913850789634
         📁 7a
            📄 12d6b30c38a1d5b1cd45e4377a77bb8d712772
            📄 94f429cf925f64d99ccf757678a23e1801c4a2
         📁 8f
            📄 3532438b05dba1ee46610f6d4e499c312b3482
            📄 e1d9dea73f13c474fe6bad0a12f50d8e443b0e
         📁 92
            📄 1e4b4af036a20e636346a9d1938a9ae1d7a436
         📁 93
            📄 b880ac5a0a80f573341433fa5ee7496c5494f8
         📁 96
            📄 58701ccfcc552ea872069c9de2d0f251403b3b
         📁 97
            📄 c87210617aac992c8ac462b40035c0fb52f744
         📁 98
            📄 23b92f7109c9f0183e11d857af7c8f7fdb9bec
            📄 5b136cf301db154a33da453b0cd3ce5e816a27
         📁 9e
            📄 c06183b074463d382317a83970aea37486cde0
         📁 a0
            📄 b97989ec4635684d9a39f988bd1f05bcd09d36
         📁 a1
            📄 44742180e230d100b4fe8e291265923d0876ab
            📄 fb773475de22dfaf2e3201d7735f1c9534b739
         📁 a5
            📄 56f7f414dd7e5fda4bb3371aea69aa000af4b7
            📄 a52a9fc3f9292ea9f9e21d00b041cff48124a8
         📁 a6
            📄 ca4d0ec528bb6400c2fb6f3077f22385f05c17
         📁 a7
            📄 86765ccd33532cd9151f59c1a25a32ae4eec22
         📁 af
            📄 e7a040ccc1114321a1a4d624b5a0bdc1512a27
         📁 b5
            📄 ade3a59c0e80e544072211858d91c911f2c05f
         📁 c1
            📄 4368af1c317e5a79b268f6698fce6ab7d8cd6b
         📁 c3
            📄 aba5f58e491a7bb1162c1a09ee0a6f562f15d9
         📁 c4
            📄 0e108162ae9329e3f5561fed5192ce6cab4db3
         📁 ca
            📄 b59ffb25f156086fd5c4be6aa4ea72d525d3d1
         📁 ce
            📄 d71449aeb59282375cc4288f7ac82a8a229d8d
         📁 d1
            📄 fb75c19df7a767fa5f66f6887f298170bb2894
         📁 dc
            📄 db3b9f9767a3579b4e035b7246e897a5b4b56e
         📁 de
            📄 6d7f073f61976a81a4bcd07902d6749cba9e6b
         📁 df
            📄 dadcdac581d93db9c6c301e88a4619ab224b1f
         📁 e6
            📄 6cde8e89398dda56149fdd2aa672e98bef222c
            📄 9de29bb2d1d6434b8b29ae775ad8c2e48c5391
            📄 d3b12ba5b9a9077adf5151c225bac5d2c4e07a
         📁 e8
            📄 d8a548540fb361cedce01460b93aa5027b7f68
         📁 ea
            📄 dbebce921e46c78152917cd948de691eb169e2
         📁 eb
            📄 82044b8dd7ca54c477e5c47b056f4ab345f538
         📁 f2
            📄 28f277c0037ebd8f6989728d0e9ee55a3a009e
         📁 f4
            📄 52e8d61bd291ca5c29c53fa32d9c291a371622
            📄 a88e2f108d7dfbbab3a099ab8836d4dae78e85
         📁 ff
            📄 591d005b238a07bc0a4f3302734d2d20e0edf9
         📁 info
         📁 pack
      📁 refs
         📁 heads
            📄 main
         📁 remotes
            📁 personal
               📄 main
         📁 tags
            📄 1.0.0
      📄 COMMIT_EDITMSG
      📄 FETCH_HEAD
      📄 HEAD
      📄 ORIG_HEAD
      📄 config
      📄 description
      📄 index
   📁 .github
      📁 workflows
         🔧 release.yml
   📁 src
      📄 main.rs
      📄 tests.rs
   📄 .gitignore
   📄 Cargo.toml
   📝 README.md
   📜 install.sh

```
