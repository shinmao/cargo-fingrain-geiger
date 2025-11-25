# Unified Unsafe Code Analysis Report

**Generated:** 2025-11-25 00:39:01

This report contains unsafe code analysis results for all crates in `/home/crates/source`.

## Summary Table

| Crate | Functions | Expressions | Impls | Traits | Methods | Ptr Derefs | Unsafe Calls | Core | Alloc | Std | Other | Status |
|-------|-----------|-------------|-------|--------|---------|------------|--------------|------|-------|-----|-------|--------|
| aho-corasick-1.1.4 | 44/45 | 2951/3026 | 5/5 | 1/1 | 107/107 | 57/60 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| anyhow-1.0.100 | 16/19 | 464/470 | 3/3 | 0/0 | 12/12 | 5/6 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| autocfg-1.5.0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| base64-0.22.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| bitflags-2.10.0 | 0/18 | 0/562 | 0/144 | 0/9 | 0/6 | 0/27 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| block-buffer-0.11.0 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| bytes-1.11.0 | 40/40 | 805/869 | 12/14 | 1/1 | 16/20 | 34/34 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| cc-1.2.47 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| cfg-if-1.0.4 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| chrono-0.4.42 | 0/0 | 4/1050 | 1/22 | 0/0 | 0/6 | 0/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| clap-4.5.53 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| clap_builder-4.5.53 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| clap_derive-4.5.49 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| clap_lex-0.7.6 | 1/1 | 15/15 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| crossbeam-utils-0.8.21 | 4/4 | 75/75 | 16/16 | 0/0 | 3/3 | 10/10 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| digest-0.11.0-rc.4 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| either-1.15.0 | 0/0 | 2/7 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| fastrand-2.3.0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| form_urlencoded-1.2.2 | 0/0 | 10/10 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| futures-0.3.31 | 43/156 | 2844/5671 | 56/88 | 1/2 | 141/333 | 110/299 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| futures-channel-0.3.31 | 0/0 | 117/117 | 10/10 | 0/0 | 3/3 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| futures-core-0.3.31 | 0/0 | 36/36 | 2/2 | 0/0 | 0/0 | 4/4 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| futures-io-0.3.31 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| futures-sink-0.3.31 | 0/0 | 2/2 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| futures-task-0.3.31 | 9/9 | 79/79 | 12/12 | 1/1 | 11/11 | 2/2 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| futures-util-0.3.31 | 15/115 | 703/3043 | 43/76 | 1/2 | 27/180 | 93/283 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| getrandom-0.3.4 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| h2-0.4.12 | 50/181 | 3675/9563 | 111/244 | 4/28 | 137/430 | 197/380 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| hashbrown-0.16.1 | 5/26 | 2408/4921 | 33/80 | 2/2 | 111/199 | 38/160 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| heck-0.5.0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| http-1.3.1 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| hyper-1.8.1 | 49/229 | 2089/9593 | 76/228 | 2/6 | 60/442 | 139/396 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| idna-1.1.0 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| indexmap-2.12.1 | 1/97 | 1410/2713 | 18/29 | 1/1 | 83/196 | 22/102 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| itertools-0.14.0 | 1/1 | 36/167 | 0/3 | 0/0 | 0/5 | 3/21 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| itoa-1.0.15 | 0/0 | 8/8 | 0/0 | 0/0 | 0/0 | 1/1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| lazy_static-1.5.0 | 0/0 | 7/7 | 1/1 | 0/0 | 0/0 | 2/2 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| libc-1.0.0-alpha.1 | 0/92 | 34/696 | 0/2 | 0/0 | 8/92 | 3/59 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| linux-raw-sys-0.12.0 | 10/10 | 292/15785 | 0/0 | 0/0 | 20/2058 | 12/120 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| lock_api-0.4.14 | 0/0 | 685/685 | 33/33 | 14/14 | 24/24 | 59/59 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| log-0.4.28 | 2/2 | 18/179 | 1/6 | 0/0 | 0/5 | 0/22 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| memchr-2.7.6 | 27/41 | 1973/2421 | 2/2 | 0/0 | 109/147 | 4/4 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| miniz_oxide-0.8.9 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| mio-1.1.0 | 2/94 | 68/1376 | 1/18 | 0/0 | 8/115 | 4/73 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| nix-0.30.1 | 0/106 | 80/2717 | 0/8 | 0/1 | 8/125 | 5/86 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| nom-8.0.0 | 36/97 | 1990/2567 | 0/2 | 0/0 | 21/71 | 33/83 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| num-traits-0.2.19 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| once_cell-1.21.3 | 0/2 | 81/160 | 5/11 | 0/1 | 3/8 | 12/17 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| parking_lot-0.12.5 | 18/108 | 2464/3560 | 57/59 | 15/15 | 79/213 | 165/232 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| parking_lot_core-0.9.12 | 17/107 | 1500/2586 | 7/9 | 1/1 | 30/164 | 103/170 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| percent-encoding-2.3.2 | 0/0 | 8/8 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| pin-project-lite-0.2.16 | 0/0 | 11/191 | 0/0 | 0/0 | 2/2 | 1/6 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| proc-macro2-1.0.103 | 0/0 | 18/18 | 0/0 | 0/0 | 3/3 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| quote-1.0.42 | 0/0 | 18/18 | 0/0 | 0/0 | 3/3 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| rand-0.10.0-rc.5 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| rand_chacha-0.10.0-rc.1 | 7/7 | 1035/1083 | 40/40 | 23/23 | 56/64 | 3/3 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| rand_core-0.10.0-rc-2 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| regex-1.12.2 | 35/51 | 3053/4190 | 11/19 | 1/2 | 196/249 | 39/55 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| regex-automata-0.4.13 | 35/51 | 3608/4190 | 14/18 | 2/2 | 206/249 | 42/55 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| regex-syntax-0.8.8 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| reqwest-0.12.24 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| rustix-1.1.2 | 47/536 | 532/24109 | 5/27 | 1/2 | 41/2181 | 12/242 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| rustls-0.23.35 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| ryu-1.0.20 | 7/9 | 572/702 | 0/0 | 0/0 | 2/2 | 34/42 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| scopeguard-1.2.0 | 0/0 | 16/16 | 1/1 | 0/0 | 0/0 | 3/3 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| semver-1.0.27 | 5/5 | 120/120 | 2/2 | 0/0 | 1/1 | 2/2 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| serde-1.0.228 | 0/0 | 5/10 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| serde_derive-1.0.228 | 0/0 | 106/106 | 3/3 | 0/0 | 5/5 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| serde_json-1.0.145 | 34/50 | 2633/3214 | 2/2 | 0/0 | 111/149 | 39/47 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| sha2-0.11.0-rc.3 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| slab-0.4.11 | 0/0 | 29/140 | 0/3 | 0/0 | 3/8 | 5/18 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| smallvec-2.0.0-alpha.12 | 0/0 | 737/742 | 6/6 | 0/0 | 20/20 | 6/6 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| socket2-0.6.1 | 3/96 | 690/1485 | 2/6 | 0/0 | 13/98 | 17/74 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| strsim-0.11.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| syn-2.0.111 | 0/0 | 106/106 | 3/3 | 0/0 | 5/5 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tempfile-3.23.0 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| thiserror-2.0.17 | 0/0 | 106/106 | 3/3 | 0/0 | 5/5 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| thiserror-impl-2.0.17 | 0/0 | 106/106 | 3/3 | 0/0 | 5/5 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| time-0.3.44 | 4/107 | 386/2119 | 0/6 | 0/0 | 7/103 | 3/80 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tokio-1.48.0 | 7/166 | 124/6101 | 8/151 | 0/4 | 9/265 | 9/283 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tokio-macros-2.6.0 | 0/0 | 106/106 | 3/3 | 0/0 | 5/5 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tokio-util-0.7.17 | 48/243 | 1912/12217 | 69/264 | 2/20 | 55/561 | 124/559 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| toml-0.9.8 | 0/44 | 145/2747 | 0/6 | 0/0 | 24/178 | 6/24 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| toml_edit-0.23.7 | 1/45 | 1462/4343 | 17/28 | 1/1 | 96/263 | 27/46 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tracing-0.1.41 | 0/3 | 289/561 | 10/19 | 0/0 | 12/14 | 37/49 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tracing-attributes-0.1.30 | 0/1 | 106/106 | 3/3 | 0/0 | 5/5 | 13/13 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| tracing-core-0.1.34 | 0/0 | 158/222 | 6/14 | 0/0 | 5/7 | 19/26 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| unicode-ident-1.0.22 | 0/0 | 4/4 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |
| url-2.5.7 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| windows-sys-0.61.2 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| windows-targets-0.53.5 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| windows_aarch64_gnullvm-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| windows_aarch64_msvc-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| windows_i686_gnu-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| windows_i686_msvc-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| windows_x86_64_gnu-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| windows_x86_64_gnullvm-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| windows_x86_64_msvc-0.53.1 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | 🔒 Safe |
| winnow-0.7.13 | - | - | - | - | - | - | - | - | - | - | - | ❌ Failed |
| zerocopy-0.9.0-alpha.0 | 4/5 | 251/363 | 25/28 | 15/15 | 25/30 | 3/16 | 0/0 | 0/0 | 0/0 | 0/0 | 0/0 | ☢️ Unsafe |

## Statistics

- **Total Directories:** 100
- **Successfully Analyzed:** 82
- **Failed:** 18
- **Skipped (no Cargo.toml):** 0

## Detailed Reports


### aho-corasick-1.1.4

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

8/8        997/997      5/5    1/1     86/86   24/24       0/0           0/0    0/0    0/0    0/0     !  aho-corasick 1.1.4
36/37      1954/2029    0/0    0/0     21/21   33/36       0/0           0/0    0/0    0/0    0/0     !  `-- memchr 2.5.0

44/45      2951/3026    5/5    1/1     107/107 57/60       0/0           0/0    0/0    0/0    0/0

```

### anyhow-1.0.100

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

16/19      464/470      3/3    0/0     12/12   5/6         0/0           0/0    0/0    0/0    0/0     !  anyhow 1.0.100

16/19      464/470      3/3    0/0     12/12   5/6         0/0           0/0    0/0    0/0    0/0

```

### autocfg-1.5.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  autocfg 1.5.0

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### base64-0.22.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) base64 0.22.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### bitflags-2.10.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  bitflags 2.10.0
0/0        0/17         0/0    0/0     0/0     0/1         0/0           0/0    0/0    0/0    0/0     ?  |-- arbitrary 1.4.2
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- derive_arbitrary 1.4.2
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- quote 1.0.41
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |       `-- syn 2.0.107
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.41
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           `-- unicode-ident 1.0.19
0/18       0/434        0/140  0/9     0/1     0/13        0/0           0/0    0/0    0/0    0/0     ?  |-- bytemuck 1.24.0
0/0        0/0          0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- bytemuck_derive 1.10.2
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- quote 1.0.41
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |       `-- syn 2.0.107
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde_core 1.0.228

0/18       0/562        0/144  0/9     0/6     0/27        0/0           0/0    0/0    0/0    0/0

```

### bytes-1.11.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

40/40      805/859      12/14  1/1     16/20   34/34       0/0           0/0    0/0    0/0    0/0     !  bytes 1.11.0
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- serde_core 1.0.228

40/40      805/869      12/14  1/1     16/20   34/34       0/0           0/0    0/0    0/0    0/0

```

### cfg-if-1.0.4

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  cfg-if 1.0.4

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### chrono-0.4.42

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        3/17         1/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  chrono 0.4.42
0/0        1/922        0/18   0/0     0/1     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- iana-time-zone 0.1.61
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- num-traits 0.2.19
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde 1.0.218
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- serde_derive 1.0.218
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- proc-macro2 1.0.93
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |   `-- unicode-ident 1.0.17
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- quote 1.0.38
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?          |   `-- proc-macro2 1.0.93
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?          `-- syn 2.0.98
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?              |-- proc-macro2 1.0.93
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?              |-- quote 1.0.38
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?              `-- unicode-ident 1.0.17

0/0        4/1050       1/22   0/0     0/6     0/13        0/0           0/0    0/0    0/0    0/0

```

### clap_lex-0.7.6

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

1/1        15/15        0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  clap_lex 0.7.6

1/1        15/15        0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### crossbeam-utils-0.8.21

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

4/4        75/75        16/16  0/0     3/3     10/10       0/0           0/0    0/0    0/0    0/0     !  crossbeam-utils 0.8.21

4/4        75/75        16/16  0/0     3/3     10/10       0/0           0/0    0/0    0/0    0/0

```

### either-1.15.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  either 1.15.0
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde 1.0.218

0/0        2/7          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### fastrand-2.3.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) fastrand 2.3.0

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### form_urlencoded-1.2.2

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  form_urlencoded 1.2.2
0/0        8/8          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- percent-encoding 2.3.2

0/0        10/10        0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### futures-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  futures 0.3.31
0/0        81/81        8/8    0/0     3/3     9/9         0/0           0/0    0/0    0/0    0/0     !  |-- futures-channel 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-core 0.3.31
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- futures-sink 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |-- futures-core 0.3.31
0/0        0/51         0/2    0/0     0/3     0/3         0/0           0/0    0/0    0/0    0/0     ?  |-- futures-executor 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-core 0.3.31
9/9        79/79        12/12  1/1     11/11   2/2         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-task 0.3.31
7/11       527/550      29/30  0/0     8/10    72/73       0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-util 0.3.31
0/0        81/81        8/8    0/0     3/3     9/9         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- futures-channel 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- futures-core 0.3.31
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- futures-io 0.3.31
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- futures-macro 0.3.31
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   |   |-- proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   |   |   `-- unicode-ident 1.0.22
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |-- quote 1.0.42
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   |   |   `-- proc-macro2 1.0.103
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  |   |   |   `-- syn 2.0.111
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   |       |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |       |-- quote 1.0.42
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   |       `-- unicode-ident 1.0.22
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- futures-sink 0.3.31
9/9        79/79        12/12  1/1     11/11   2/2         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- futures-task 0.3.31
0/2        0/558        0/24   0/1     0/14    0/69        0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- futures 0.1.31
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- memchr 2.7.6
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- log 0.4.28
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- pin-project-lite 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- pin-utils 0.1.0
0/0        29/29        0/0    0/0     3/3     5/5         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- slab 0.4.11
0/0        0/33         0/0    0/0     0/12    0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- tokio-io 0.1.13
0/0        0/496        0/2    0/0     0/28    0/32        0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- bytes 0.4.12
0/1        0/179        0/0    0/0     0/0     0/16        0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |-- byteorder 1.5.0
0/0        0/81         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   `-- iovec 0.1.4
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |       |       `-- libc 0.2.177
0/2        0/558        0/24   0/1     0/14    0/69        0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- futures 0.1.31
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- log 0.4.28
0/0        0/73         0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- num_cpus 1.17.0
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |       `-- libc 0.2.177
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- futures-io 0.3.31
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- futures-sink 0.3.31
9/9        79/79        12/12  1/1     11/11   2/2         0/0           0/0    0/0    0/0    0/0     !  |-- futures-task 0.3.31
7/11       527/550      29/30  0/0     8/10    72/73       0/0           0/0    0/0    0/0    0/0     !  `-- futures-util 0.3.31

43/156     2844/5671    56/88  1/2     141/333 110/299     0/0           0/0    0/0    0/0    0/0

```

### futures-channel-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        81/81        8/8    0/0     3/3     9/9         0/0           0/0    0/0    0/0    0/0     !  futures-channel 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  `-- futures-core 0.3.31

0/0        117/117      10/10  0/0     3/3     13/13       0/0           0/0    0/0    0/0    0/0

```

### futures-core-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  futures-core 0.3.31

0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0

```

### futures-io-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  futures-io 0.3.31

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### futures-sink-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  futures-sink 0.3.31

0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### futures-task-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

9/9        79/79        12/12  1/1     11/11   2/2         0/0           0/0    0/0    0/0    0/0     !  futures-task 0.3.31

9/9        79/79        12/12  1/1     11/11   2/2         0/0           0/0    0/0    0/0    0/0

```

### futures-util-0.3.31

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

6/11       442/550      26/30  0/0     6/10    68/73       0/0           0/0    0/0    0/0    0/0     !  futures-util 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |-- futures-core 0.3.31
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- futures-macro 0.3.31
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |-- proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   `-- unicode-ident 1.0.22
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- quote 1.0.42
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   `-- proc-macro2 1.0.103
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  |   `-- syn 2.0.111
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |       |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- quote 1.0.42
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |       `-- unicode-ident 1.0.22
9/9        79/79        12/12  1/1     11/11   2/2         0/0           0/0    0/0    0/0    0/0     !  |-- futures-task 0.3.31
0/2        0/558        0/24   0/1     0/14    0/69        0/0           0/0    0/0    0/0    0/0     ?  |-- futures 0.1.31
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |-- pin-project-lite 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- pin-utils 0.1.0
0/0        29/29        0/0    0/0     3/3     5/5         0/0           0/0    0/0    0/0    0/0     !  |-- slab 0.4.11
0/0        0/33         0/0    0/0     0/12    0/4         0/0           0/0    0/0    0/0    0/0     ?  `-- tokio-io 0.1.13
0/0        0/496        0/2    0/0     0/28    0/32        0/0           0/0    0/0    0/0    0/0     ?      |-- bytes 0.4.12
0/1        0/179        0/0    0/0     0/0     0/16        0/0           0/0    0/0    0/0    0/0     ?      |   |-- byteorder 1.5.0
0/0        0/81         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?      |   `-- iovec 0.1.4
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      |       `-- libc 0.2.177
0/2        0/558        0/24   0/1     0/14    0/69        0/0           0/0    0/0    0/0    0/0     ?      |-- futures 0.1.31
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- log 0.4.28

15/115     703/3043     43/76  1/2     27/180  93/283      0/0           0/0    0/0    0/0    0/0

```

### h2-0.4.12

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        3/3          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  h2 0.4.12
0/0        32/32        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |-- atomic-waker 1.1.2
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |-- bytes 1.10.1
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.219
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- fnv 1.0.7
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |-- futures-core 0.3.31
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- futures-sink 0.3.31
1/1        162/162      10/10  0/0     2/2     16/16       0/0           0/0    0/0    0/0    0/0     !  |-- http 1.3.1
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |   |-- bytes 1.10.1
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- fnv 1.0.7
0/0        8/8          0/0    0/0     0/0     1/1         0/0           0/0    0/0    0/0    0/0     !  |   `-- itoa 1.0.15
0/0        78/83        0/0    0/0     0/0     5/6         0/0           0/0    0/0    0/0    0/0     !  |-- indexmap 2.10.0
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- equivalent 1.0.2
1/1        1244/1520    17/22  1/1     72/85   16/16       0/0           0/0    0/0    0/0    0/0     !  |   |-- hashbrown 0.15.4
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- equivalent 1.0.2
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.219
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- quickcheck 1.0.3
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- log 0.4.27
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- serde 1.0.219
0/0        0/32         0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- rand 0.8.5
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- libc 0.2.174
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- log 0.4.27
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- rand_chacha 0.3.1
0/2        0/680        0/0    0/0     0/25    0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |-- ppv-lite86 0.2.21
0/5        0/403        0/40   0/23    0/39    0/3         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |   `-- zerocopy 0.8.26
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |-- rand_core 0.6.4
0/6        0/192        0/1    0/0     0/3     0/3         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |   |-- getrandom 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |   |   |-- cfg-if 1.0.1
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |   |   `-- libc 0.2.174
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   |   `-- serde 1.0.219
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   `-- serde 1.0.219
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- rand_core 0.6.4
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- serde 1.0.219
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.219
0/0        29/29        0/0    0/0     3/3     5/5         0/0           0/0    0/0    0/0    0/0     !  |-- slab 0.4.10
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.219
0/0        36/50        1/1    0/0     1/1     2/3         0/0           0/0    0/0    0/0    0/0     !  |-- tokio-util 0.7.15
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |   |-- bytes 1.10.1
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-core 0.3.31
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-sink 0.3.31
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |   |-- pin-project-lite 0.2.16
0/0        29/29        0/0    0/0     3/3     5/5         0/0           0/0    0/0    0/0    0/0     !  |   |-- slab 0.4.10
7/25       1049/2753    60/114 2/3     36/117  85/143      0/0           0/0    0/0    0/0    0/0     !  |   |-- tokio 1.47.0
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |   |   |-- bytes 1.10.1
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- libc 0.2.174
0/2        0/686        0/13   0/0     0/23    0/14        0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- mio 1.0.4
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |-- libc 0.2.174
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- log 0.4.27
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- pin-project-lite 0.2.16
0/6        0/788        0/4    0/0     0/6     0/15        0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- socket2 0.6.0
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- libc 0.2.174
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- tokio-macros 2.5.0
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- proc-macro2 1.0.95
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   `-- unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |   `-- proc-macro2 1.0.95
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- syn 2.0.104
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- proc-macro2 1.0.95
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           `-- unicode-ident 1.0.18
0/0        14/14        1/1    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   `-- tracing 0.1.41
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- log 0.4.27
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |       |-- pin-project-lite 0.2.16
0/0        77/98        1/5    0/0     2/2     7/11        0/0           0/0    0/0    0/0    0/0     !  |       `-- tracing-core 0.1.34
0/0        81/124       5/9    0/0     3/5     12/15       0/0           0/0    0/0    0/0    0/0     !  |           `-- once_cell 1.21.3
7/25       1049/2753    60/114 2/3     36/117  85/143      0/0           0/0    0/0    0/0    0/0     !  |-- tokio 1.47.0
0/0        14/14        1/1    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  `-- tracing 0.1.41

50/181     3675/9563    111/244 4/28    137/430 197/380     0/0           0/0    0/0    0/0    0/0

```

### hashbrown-0.16.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

1/1        1387/1601    20/23  1/1     83/96   17/17       0/0           0/0    0/0    0/0    0/0     !  hashbrown 0.16.1
2/3        819/903      12/13  1/1     28/32   19/19       0/0           0/0    0/0    0/0    0/0     !  |-- allocator-api2 0.2.21
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- equivalent 1.0.2
2/2        202/202      1/1    0/0     0/0     2/2         0/0           0/0    0/0    0/0    0/0     !  |-- foldhash 0.2.0
0/6        0/667        0/5    0/0     0/3     0/33        0/0           0/0    0/0    0/0    0/0     ?  |-- rayon 1.11.0
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- either 1.15.0
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.228
0/7        0/566        0/5    0/0     0/30    0/36        0/0           0/0    0/0    0/0    0/0     ?  |   `-- rayon-core 1.13.0
0/0        0/457        0/6    0/0     0/6     0/21        0/0           0/0    0/0    0/0    0/0     ?  |       |-- crossbeam-deque 0.8.6
0/3        0/438        0/11   0/0     0/29    0/22        0/0           0/0    0/0    0/0    0/0     ?  |       |   |-- crossbeam-epoch 0.9.18
0/4        0/75         0/16   0/0     0/3     0/10        0/0           0/0    0/0    0/0    0/0     ?  |       |   |   `-- crossbeam-utils 0.8.21
0/4        0/75         0/16   0/0     0/3     0/10        0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- crossbeam-utils 0.8.21
0/4        0/75         0/16   0/0     0/3     0/10        0/0           0/0    0/0    0/0    0/0     ?  |       `-- crossbeam-utils 0.8.21
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde_core 1.0.228

5/26       2408/4921    33/80  2/2     111/199 38/160      0/0           0/0    0/0    0/0    0/0

```

### heck-0.5.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) heck 0.5.0

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### hyper-1.8.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/1        71/125       0/11   0/1     4/4     4/4         0/0           0/0    0/0    0/0    0/0     !  hyper 1.8.1
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |-- bytes 1.10.1
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde_derive 1.0.228
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- proc-macro2 1.0.103
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |   `-- unicode-ident 1.0.20
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.41
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |   `-- proc-macro2 1.0.103
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |           `-- syn 2.0.108
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- quote 1.0.41
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               `-- unicode-ident 1.0.20
0/0        0/81         0/8    0/0     0/3     0/9         0/0           0/0    0/0    0/0    0/0     ?  |-- futures-channel 0.3.31
0/0        0/36         0/2    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-core 0.3.31
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- futures-sink 0.3.31
0/0        0/36         0/2    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |-- futures-core 0.3.31
0/11       0/550        0/30   0/0     0/10    0/73        0/0           0/0    0/0    0/0    0/0     ?  |-- futures-util 0.3.31
0/0        0/81         0/8    0/0     0/3     0/9         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-channel 0.3.31
0/0        0/36         0/2    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-core 0.3.31
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-sink 0.3.31
0/9        0/79         0/12   0/1     0/11    0/2         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-task 0.3.31
0/41       0/2421       0/2    0/0     0/147   0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- memchr 2.7.6
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- log 0.4.28
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- serde 1.0.228
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |   |-- pin-project-lite 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- pin-utils 0.1.0
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- http-body-util 0.1.3
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |   |-- bytes 1.10.1
0/0        0/36         0/2    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-core 0.3.31
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |-- http-body 1.0.1
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |   |   |-- bytes 1.10.1
1/1        162/162      10/10  0/0     2/2     16/16       0/0           0/0    0/0    0/0    0/0     !  |   |   `-- http 1.3.1
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |   |       |-- bytes 1.10.1
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- fnv 1.0.7
0/0        8/8          0/0    0/0     0/0     1/1         0/0           0/0    0/0    0/0    0/0     !  |   |       `-- itoa 1.0.15
1/1        162/162      10/10  0/0     2/2     16/16       0/0           0/0    0/0    0/0    0/0     !  |   |-- http 1.3.1
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |   |-- pin-project-lite 0.2.16
7/25       1020/2761    54/114 1/3     36/117  82/141      0/0           0/0    0/0    0/0    0/0     !  |   `-- tokio 1.48.0
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |       |-- bytes 1.10.1
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |       |-- libc 0.2.177
0/2        0/676        0/15   0/0     0/23    0/14        0/0           0/0    0/0    0/0    0/0     ?  |       |-- mio 1.1.0
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |       |   |-- libc 0.2.177
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- log 0.4.28
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |       |-- pin-project-lite 0.2.16
0/6        0/803        0/4    0/0     0/6     0/15        0/0           0/0    0/0    0/0    0/0     ?  |       |-- socket2 0.6.1
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- libc 0.2.177
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- tokio-macros 2.6.0
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.41
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |           `-- syn 2.0.108
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- http-body 1.0.1
1/1        162/162      10/10  0/0     2/2     16/16       0/0           0/0    0/0    0/0    0/0     !  |-- http 1.3.1
0/0        8/8          0/0    0/0     0/0     1/1         0/0           0/0    0/0    0/0    0/0     !  |-- itoa 1.0.15
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |-- pin-project-lite 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- pin-utils 0.1.0
7/25       1020/2761    54/114 1/3     36/117  82/141      0/0           0/0    0/0    0/0    0/0     !  `-- tokio 1.48.0

49/229     2089/9593    76/228 2/6     60/442  139/396     0/0           0/0    0/0    0/0    0/0

```

### indexmap-2.12.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        78/83        0/0    0/0     0/0     5/6         0/0           0/0    0/0    0/0    0/0     !  indexmap 2.12.1
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- equivalent 1.0.2
1/1        1332/1601    18/23  1/1     83/96   17/17       0/0           0/0    0/0    0/0    0/0     !  |-- hashbrown 0.16.1
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- equivalent 1.0.2
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- quickcheck 1.0.3
0/0        0/32         0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   `-- rand 0.8.5
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |       |-- libc 0.2.177
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- rand_core 0.6.4
0/6        0/192        0/1    0/0     0/3     0/3         0/0           0/0    0/0    0/0    0/0     ?  |       |   |-- getrandom 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   |   |-- cfg-if 1.0.3
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |       |   |   `-- libc 0.2.177
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |       |-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |       `-- serde_derive 1.0.228
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |           |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |           |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |           |-- quote 1.0.41
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |           |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |       |           `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |               |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |               |-- quote 1.0.41
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |               `-- unicode-ident 1.0.19
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde_core 1.0.228

1/97       1410/2713    18/29  1/1     83/196  22/102      0/0           0/0    0/0    0/0    0/0

```

### itertools-0.14.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

1/1        22/49        0/0    0/0     0/0     1/10        0/0           0/0    0/0    0/0    0/0     !  itertools 0.14.0
0/0        14/14        0/0    0/0     0/0     2/2         0/0           0/0    0/0    0/0    0/0     !  `-- either 1.11.0
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- serde 1.0.202
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          `-- serde_derive 1.0.202
0/0        0/15         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?              |-- proc-macro2 1.0.82
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?              |   `-- unicode-ident 1.0.12
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?              |-- quote 1.0.36
0/0        0/15         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?              |   `-- proc-macro2 1.0.82
0/0        0/80         0/3    0/0     0/2     0/9         0/0           0/0    0/0    0/0    0/0     ?              `-- syn 2.0.63
0/0        0/15         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?                  |-- proc-macro2 1.0.82
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?                  |-- quote 1.0.36
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?                  `-- unicode-ident 1.0.12

1/1        36/167       0/3    0/0     0/5     3/21        0/0           0/0    0/0    0/0    0/0

```

### itoa-1.0.15

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        8/8          0/0    0/0     0/0     1/1         0/0           0/0    0/0    0/0    0/0     !  itoa 1.0.15

0/0        8/8          0/0    0/0     0/0     1/1         0/0           0/0    0/0    0/0    0/0

```

### lazy_static-1.5.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        7/7          1/1    0/0     0/0     2/2         0/0           0/0    0/0    0/0    0/0     !  lazy_static 1.5.0

0/0        7/7          1/1    0/0     0/0     2/2         0/0           0/0    0/0    0/0    0/0

```

### libc-1.0.0-alpha.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/92       34/696       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0     !  libc 1.0.0-alpha.1

0/92       34/696       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0

```

### linux-raw-sys-0.12.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

10/10      292/15785    0/0    0/0     20/2058 12/120      0/0           0/0    0/0    0/0    0/0     !  linux-raw-sys 0.12.0

10/10      292/15785    0/0    0/0     20/2058 12/120      0/0           0/0    0/0    0/0    0/0

```

### lock_api-0.4.14

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        669/669      32/32  14/14   24/24   56/56       0/0           0/0    0/0    0/0    0/0     !  lock_api 0.4.14
0/0        16/16        1/1    0/0     0/0     3/3         0/0           0/0    0/0    0/0    0/0     !  `-- scopeguard 1.2.0

0/0        685/685      33/33  14/14   24/24   59/59       0/0           0/0    0/0    0/0    0/0

```

### log-0.4.28

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

2/2        18/18        1/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  log 0.4.28
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- serde 1.0.219
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde_derive 1.0.219
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |       `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           `-- unicode-ident 1.0.18
0/0        0/31         0/2    0/0     0/0     0/7         0/0           0/0    0/0    0/0    0/0     ?  |-- sval 2.14.1
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- sval_derive_macros 2.14.1
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- quote 1.0.40
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |       `-- syn 2.0.106
0/0        0/19         0/0    0/0     0/0     0/2         0/0           0/0    0/0    0/0    0/0     ?  `-- value-bag 1.11.1

2/2        18/179       1/6    0/0     0/5     0/22        0/0           0/0    0/0    0/0    0/0

```

### memchr-2.7.6

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  memchr 2.7.6

27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0

```

### miniz_oxide-0.8.9

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) miniz_oxide 0.8.9
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) `-- adler2 2.0.0

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### mio-1.1.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/2        16/676       0/15   0/0     0/23    1/14        0/0           0/0    0/0    0/0    0/0     !  mio 1.1.0
0/90       34/682       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0     !  |-- libc 0.2.171
2/2        18/18        1/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- log 0.4.27

2/94       68/1376      1/18   0/0     8/115   4/73        0/0           0/0    0/0    0/0    0/0

```

### nix-0.30.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/16       46/2035      0/6    0/1     0/33    2/27        0/0           0/0    0/0    0/0    0/0     !  nix 0.30.1
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- bitflags 2.5.0
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- cfg-if 1.0.0
0/90       34/682       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0     !  `-- libc 0.2.172

0/106      80/2717      0/8    0/1     8/125   5/86        0/0           0/0    0/0    0/0    0/0

```

### nom-8.0.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        36/36        0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  nom 8.0.0
36/37      1954/2029    0/0    0/0     21/21   33/36       0/0           0/0    0/0    0/0    0/0     !  `-- memchr 2.5.0
0/60       0/502        0/2    0/0     0/50    0/47        0/0           0/0    0/0    0/0    0/0     ?      `-- libc 0.2.146

36/97      1990/2567    0/2    0/0     21/71   33/83       0/0           0/0    0/0    0/0    0/0

```

### num-traits-0.2.19

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  num-traits 0.2.19

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### once_cell-1.21.3

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        81/124       5/9    0/0     3/5     12/15       0/0           0/0    0/0    0/0    0/0     !  once_cell 1.21.3
0/2        0/36         0/2    0/1     0/3     0/2         0/0           0/0    0/0    0/0    0/0     ?  `-- critical-section 1.1.3

0/2        81/160       5/11   0/1     3/8     12/17       0/0           0/0    0/0    0/0    0/0

```

### parking_lot-0.12.5

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

1/1        279/279      17/17  0/0     25/25   3/3         0/0           0/0    0/0    0/0    0/0     !  parking_lot 0.12.5
0/0        669/669      32/32  14/14   24/24   56/56       0/0           0/0    0/0    0/0    0/0     !  |-- lock_api 0.4.14
0/0        16/16        1/1    0/0     0/0     3/3         0/0           0/0    0/0    0/0    0/0     !  |   |-- scopeguard 1.2.0
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde_core 1.0.228
16/16      912/1343     0/0    0/0     8/58    78/89       0/0           0/0    0/0    0/0    0/0     !  `-- parking_lot_core 0.9.12
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- cfg-if 1.0.3
0/90       34/687       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0     !      |-- libc 0.2.176
1/1        554/556      7/7    1/1     14/14   22/22       0/0           0/0    0/0    0/0    0/0     !      `-- smallvec 1.15.1
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          `-- serde 1.0.228

18/108     2464/3560    57/59  15/15   79/213  165/232     0/0           0/0    0/0    0/0    0/0

```

### parking_lot_core-0.9.12

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

16/16      912/1343     0/0    0/0     8/58    78/89       0/0           0/0    0/0    0/0    0/0     !  parking_lot_core 0.9.12
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- cfg-if 1.0.3
0/90       34/687       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0     !  |-- libc 0.2.176
1/1        554/556      7/7    1/1     14/14   22/22       0/0           0/0    0/0    0/0    0/0     !  `-- smallvec 1.15.1

17/107     1500/2586    7/9    1/1     30/164  103/170     0/0           0/0    0/0    0/0    0/0

```

### percent-encoding-2.3.2

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        8/8          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  percent-encoding 2.3.2

0/0        8/8          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### pin-project-lite-0.2.16

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  pin-project-lite 0.2.16

0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0

```

### proc-macro2-1.0.103

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- unicode-ident 1.0.20

0/0        18/18        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0

```

### quote-1.0.42

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  quote 1.0.42
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !      `-- unicode-ident 1.0.22

0/0        18/18        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0

```

### rand_chacha-0.10.0-rc.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) rand_chacha 0.10.0-rc.1
2/2        632/680      0/0    0/0     17/25   0/0         0/0           0/0    0/0    0/0    0/0     !  |-- ppv-lite86 0.2.21
5/5        403/403      40/40  23/23   39/39   3/3         0/0           0/0    0/0    0/0    0/0     !  |   `-- zerocopy 0.8.27
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- rand_core 0.10.0-rc-2

7/7        1035/1083    40/40  23/23   56/64   3/3         0/0           0/0    0/0    0/0    0/0

```

### rand_core-0.10.0-rc-2

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  rand_core 0.10.0-rc-2

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### regex-1.12.2

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  regex 1.12.2
8/8        997/997      5/5    1/1     86/86   24/24       0/0           0/0    0/0    0/0    0/0     !  |-- aho-corasick 1.1.3
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- log 0.4.28
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- serde_derive 1.0.228
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- quote 1.0.41
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |   |           `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               |-- quote 1.0.41
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               `-- unicode-ident 1.0.19
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |   `-- memchr 2.7.6
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- log 0.4.28
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |-- memchr 2.7.6
0/0        83/638       4/7    0/1     1/11    11/14       0/0           0/0    0/0    0/0    0/0     !  |-- regex-automata 0.4.13
8/8        997/997      5/5    1/1     86/86   24/24       0/0           0/0    0/0    0/0    0/0     !  |   |-- aho-corasick 1.1.3
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- log 0.4.28
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |   |-- memchr 2.7.6
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) |   `-- regex-syntax 0.8.8
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) `-- regex-syntax 0.8.8

35/51      3053/4190    11/19  1/2     196/249 39/55       0/0           0/0    0/0    0/0    0/0

```

### regex-automata-0.4.13

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        638/638      7/7    1/1     11/11   14/14       0/0           0/0    0/0    0/0    0/0     !  regex-automata 0.4.13
8/8        997/997      5/5    1/1     86/86   24/24       0/0           0/0    0/0    0/0    0/0     !  |-- aho-corasick 1.1.3
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- log 0.4.28
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- serde_derive 1.0.228
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- quote 1.0.41
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |   |           `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               |-- quote 1.0.41
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               `-- unicode-ident 1.0.19
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |   `-- memchr 2.7.6
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- log 0.4.28
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- log 0.4.28
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |-- memchr 2.7.6
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) `-- regex-syntax 0.8.8

35/51      3608/4190    14/18  2/2     206/249 42/55       0/0           0/0    0/0    0/0    0/0

```

### regex-syntax-0.8.8

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) regex-syntax 0.8.8

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### rustix-1.1.2

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

37/436     240/7503     5/22   1/2     21/62   0/46        0/0           0/0    0/0    0/0    0/0     !  rustix 1.1.2
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- bitflags 2.9.1
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.219
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde_derive 1.0.219
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |   `-- unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |           `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               `-- unicode-ident 1.0.18
0/0        0/103        0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |-- errno 0.3.13
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   `-- libc 0.2.175
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |-- libc 0.2.175
10/10      292/15713    0/0    0/0     20/2022 12/120      0/0           0/0    0/0    0/0    0/0     !  `-- linux-raw-sys 0.11.0

47/536     532/24109    5/27   1/2     41/2181 12/242      0/0           0/0    0/0    0/0    0/0

```

### ryu-1.0.20

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

7/9        572/702      0/0    0/0     2/2     34/42       0/0           0/0    0/0    0/0    0/0     !  ryu 1.0.20

7/9        572/702      0/0    0/0     2/2     34/42       0/0           0/0    0/0    0/0    0/0

```

### scopeguard-1.2.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        16/16        1/1    0/0     0/0     3/3         0/0           0/0    0/0    0/0    0/0     !  scopeguard 1.2.0

0/0        16/16        1/1    0/0     0/0     3/3         0/0           0/0    0/0    0/0    0/0

```

### semver-1.0.27

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

5/5        120/120      2/2    0/0     1/1     2/2         0/0           0/0    0/0    0/0    0/0     !  semver 1.0.27

5/5        120/120      2/2    0/0     1/1     2/2         0/0           0/0    0/0    0/0    0/0

```

### serde-1.0.228

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  serde 1.0.228
0/0        5/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- serde_core 1.0.228

0/0        5/10         0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### serde_derive-1.0.228

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  serde_derive 1.0.228
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- proc-macro2 1.0.101
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- quote 1.0.40
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- proc-macro2 1.0.101
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  `-- syn 2.0.106
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !      |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- quote 1.0.40
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !      `-- unicode-ident 1.0.19

0/0        106/106      3/3    0/0     5/5     13/13       0/0           0/0    0/0    0/0    0/0

```

### serde_json-1.0.145

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        75/78        0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  serde_json 1.0.145
0/0        8/8          0/0    0/0     0/0     1/1         0/0           0/0    0/0    0/0    0/0     !  |-- itoa 1.0.15
27/41      1973/2421    2/2    0/0     109/147 4/4         0/0           0/0    0/0    0/0    0/0     !  |-- memchr 2.7.5
7/9        572/702      0/0    0/0     2/2     34/42       0/0           0/0    0/0    0/0    0/0     !  |-- ryu 1.0.20
0/0        5/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- serde_core 1.0.220

34/50      2633/3214    2/2    0/0     111/149 39/47       0/0           0/0    0/0    0/0    0/0

```

### slab-0.4.11

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        29/29        0/0    0/0     3/3     5/5         0/0           0/0    0/0    0/0    0/0     !  slab 0.4.11
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde 1.0.219
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- serde_derive 1.0.219
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- proc-macro2 1.0.95
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |   `-- unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?          |   `-- proc-macro2 1.0.95
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?          `-- syn 2.0.104
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?              |-- proc-macro2 1.0.95
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?              |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?              `-- unicode-ident 1.0.18

0/0        29/140       0/3    0/0     3/8     5/18        0/0           0/0    0/0    0/0    0/0

```

### smallvec-2.0.0-alpha.12

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        737/737      6/6    0/0     20/20   6/6         0/0           0/0    0/0    0/0    0/0     !  smallvec 2.0.0-alpha.12
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- serde_core 1.0.228

0/0        737/742      6/6    0/0     20/20   6/6         0/0           0/0    0/0    0/0    0/0

```

### socket2-0.6.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

3/6        656/803      2/4    0/0     5/6     14/15       0/0           0/0    0/0    0/0    0/0     !  socket2 0.6.1
0/90       34/682       0/2    0/0     8/92    3/59        0/0           0/0    0/0    0/0    0/0     !  `-- libc 0.2.172

3/96       690/1485     2/6    0/0     13/98   17/74       0/0           0/0    0/0    0/0    0/0

```

### strsim-0.11.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) strsim 0.11.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### syn-2.0.111

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  syn 2.0.111
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- unicode-ident 1.0.22
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- quote 1.0.42
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  `-- unicode-ident 1.0.22

0/0        106/106      3/3    0/0     5/5     13/13       0/0           0/0    0/0    0/0    0/0

```

### thiserror-2.0.17

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  thiserror 2.0.17
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- thiserror-impl 2.0.17
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !      |-- proc-macro2 1.0.101
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !      |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- quote 1.0.40
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !      |   `-- proc-macro2 1.0.101
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !      `-- syn 2.0.106
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !          |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- quote 1.0.40
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !          `-- unicode-ident 1.0.19

0/0        106/106      3/3    0/0     5/5     13/13       0/0           0/0    0/0    0/0    0/0

```

### thiserror-impl-2.0.17

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  thiserror-impl 2.0.17
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- proc-macro2 1.0.101
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- quote 1.0.40
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- proc-macro2 1.0.101
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  `-- syn 2.0.106
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !      |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- quote 1.0.40
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !      `-- unicode-ident 1.0.19

0/0        106/106      3/3    0/0     5/5     13/13       0/0           0/0    0/0    0/0    0/0

```

### time-0.3.44

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

2/5        356/389      0/0    0/0     6/6     0/0         0/0           0/0    0/0    0/0    0/0     !  time 0.3.44
0/0        1/1          0/0    0/0     1/1     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- deranged 0.5.2
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- num-traits 0.2.17
2/2        29/29        0/0    0/0     0/0     3/3         0/0           0/0    0/0    0/0    0/0     !  |   |-- powerfmt 0.2.0
0/0        0/32         0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- rand 0.8.5
0/90       0/582        0/2    0/0     0/63    0/52        0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- libc 0.2.152
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- rand_chacha 0.3.1
0/2        0/706        0/0    0/0     0/25    0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |-- ppv-lite86 0.2.17
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |-- rand_core 0.6.4
0/7        0/222        0/1    0/0     0/3     0/3         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |   |-- getrandom 0.2.12
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |   |   |-- cfg-if 1.0.0
0/90       0/582        0/2    0/0     0/63    0/52        0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |   |   `-- libc 0.2.152
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |   `-- serde 1.0.217
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |       `-- serde_derive 1.0.217
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |           |-- proc-macro2 1.0.93
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |           |   `-- unicode-ident 1.0.14
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |           |-- quote 1.0.38
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |           |   `-- proc-macro2 1.0.93
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |           `-- syn 2.0.96
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |               |-- proc-macro2 1.0.93
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |               |-- quote 1.0.38
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   |               `-- unicode-ident 1.0.14
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- serde 1.0.217
0/0        0/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- rand_core 0.6.4
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.217
0/1        0/37         0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- rand 0.9.2
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- rand_core 0.9.3
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- serde 1.0.217
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.217
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.217
0/0        0/8          0/0    0/0     0/0     0/1         0/0           0/0    0/0    0/0    0/0     ?  |-- itoa 1.0.14
0/90       0/582        0/2    0/0     0/63    0/52        0/0           0/0    0/0    0/0    0/0     ?  |-- libc 0.2.152
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- num-conv 0.1.0
2/2        29/29        0/0    0/0     0/0     3/3         0/0           0/0    0/0    0/0    0/0     !  |-- powerfmt 0.2.0
0/0        0/32         0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |-- rand 0.8.5
0/1        0/37         0/0    0/0     0/0     0/4         0/0           0/0    0/0    0/0    0/0     ?  |-- rand 0.9.2
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- serde 1.0.217
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- time-core 0.1.6
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- time-macros 0.2.24
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- num-conv 0.1.0
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- time-core 0.1.6

4/107      386/2119     0/6    0/0     7/103   3/80        0/0           0/0    0/0    0/0    0/0

```

### tokio-1.48.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

7/25       113/2761     8/114  0/3     7/117   8/141       0/0           0/0    0/0    0/0    0/0     !  tokio 1.48.0
0/41       0/867        0/14   0/1     0/20    0/35        0/0           0/0    0/0    0/0    0/0     ?  |-- bytes 1.10.1
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.219
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde_derive 1.0.219
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |   `-- unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |           `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               `-- unicode-ident 1.0.18
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |-- libc 0.2.175
0/2        0/686        0/13   0/0     0/23    0/14        0/0           0/0    0/0    0/0    0/0     ?  |-- mio 1.0.4
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?  |   |-- libc 0.2.175
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- log 0.4.27
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde 1.0.219
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |-- pin-project-lite 0.2.16
0/6        0/788        0/4    0/0     0/6     0/15        0/0           0/0    0/0    0/0    0/0     ?  `-- socket2 0.6.0
0/90       0/679        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      `-- libc 0.2.175

7/166      124/6101     8/151  0/4     9/265   9/283       0/0           0/0    0/0    0/0    0/0

```

### tokio-macros-2.6.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  tokio-macros 2.6.0
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- proc-macro2 1.0.101
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- unicode-ident 1.0.18
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- quote 1.0.40
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- proc-macro2 1.0.101
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  `-- syn 2.0.106
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !      |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- quote 1.0.40
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !      `-- unicode-ident 1.0.18

0/0        106/106      3/3    0/0     5/5     13/13       0/0           0/0    0/0    0/0    0/0

```

### tokio-util-0.7.17

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        30/53        1/1    0/0     1/1     2/3         0/0           0/0    0/0    0/0    0/0     !  tokio-util 0.7.17
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !  |-- bytes 1.10.1
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |-- futures-core 0.3.31
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- futures-io 0.3.31
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- futures-sink 0.3.31
0/11       0/550        0/30   0/0     0/10    0/73        0/0           0/0    0/0    0/0    0/0     ?  |-- futures-util 0.3.31
0/0        0/81         0/8    0/0     0/3     0/9         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-channel 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   |   |-- futures-core 0.3.31
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |   `-- futures-sink 0.3.31
0/0        36/36        2/2    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-core 0.3.31
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-io 0.3.31
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-macro 0.3.31
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- proc-macro2 1.0.103
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- unicode-ident 1.0.20
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- quote 1.0.41
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |   `-- proc-macro2 1.0.103
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- syn 2.0.108
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- quote 1.0.41
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- unicode-ident 1.0.20
0/0        2/2          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |-- futures-sink 0.3.31
0/9        0/79         0/12   0/1     0/11    0/2         0/0           0/0    0/0    0/0    0/0     ?  |   |-- futures-task 0.3.31
0/41       0/2421       0/2    0/0     0/147   0/4         0/0           0/0    0/0    0/0    0/0     ?  |   |-- memchr 2.7.6
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |   |-- pin-project-lite 0.2.16
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- pin-utils 0.1.0
0/0        0/29         0/0    0/0     0/3     0/5         0/0           0/0    0/0    0/0    0/0     ?  |   `-- slab 0.4.11
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |-- pin-project-lite 0.2.16
0/0        0/29         0/0    0/0     0/3     0/5         0/0           0/0    0/0    0/0    0/0     ?  |-- slab 0.4.11
7/25       1020/2761    54/114 1/3     36/117  82/141      0/0           0/0    0/0    0/0    0/0     !  `-- tokio 1.48.0
41/41      813/867      12/14  1/1     16/20   35/35       0/0           0/0    0/0    0/0    0/0     !      |-- bytes 1.10.1
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      |-- libc 0.2.177
0/2        0/676        0/15   0/0     0/23    0/14        0/0           0/0    0/0    0/0    0/0     ?      |-- mio 1.1.0
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      |   `-- libc 0.2.177
0/1        0/279        0/17   0/0     0/25    0/3         0/0           0/0    0/0    0/0    0/0     ?      |-- parking_lot 0.12.5
0/0        0/669        0/32   0/14    0/24    0/56        0/0           0/0    0/0    0/0    0/0     ?      |   |-- lock_api 0.4.14
0/0        0/16         0/1    0/0     0/0     0/3         0/0           0/0    0/0    0/0    0/0     ?      |   |   `-- scopeguard 1.2.0
0/16       0/1343       0/0    0/0     0/58    0/89        0/0           0/0    0/0    0/0    0/0     ?      |   `-- parking_lot_core 0.9.12
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |       |-- cfg-if 1.0.4
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      |       |-- libc 0.2.177
0/1        0/556        0/7    0/1     0/14    0/22        0/0           0/0    0/0    0/0    0/0     ?      |       `-- smallvec 1.15.1
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !      |-- pin-project-lite 0.2.16
0/0        0/12         0/0    0/0     0/0     0/3         0/0           0/0    0/0    0/0    0/0     :)     |-- signal-hook-registry 1.4.6
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      |   `-- libc 0.2.177
0/6        0/803        0/4    0/0     0/6     0/15        0/0           0/0    0/0    0/0    0/0     ?      |-- socket2 0.6.1
0/90       0/687        0/2    0/0     0/92    0/59        0/0           0/0    0/0    0/0    0/0     ?      |   `-- libc 0.2.177
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      `-- tokio-macros 2.6.0
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- quote 1.0.41
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?          `-- syn 2.0.108

48/243     1912/12217   69/264 2/20    55/561  124/559     0/0           0/0    0/0    0/0    0/0

```

### toml-0.9.8

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) toml 0.9.8
0/1        0/7          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- anstream 0.6.20
0/0        0/17         0/0    0/0     0/0     0/1         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle-parse 0.2.7
0/0        0/1          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- utf8parse 0.2.2
0/0        0/22         0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle-query 1.1.4
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle 1.0.11
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- colorchoice 1.0.4
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- is_terminal_polyfill 1.70.1
0/0        0/1          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- utf8parse 0.2.2
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- anstyle 1.0.11
0/0        5/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- serde_core 1.0.225
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) |-- serde_spanned 1.0.3
0/0        5/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- serde_core 1.0.225
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) |-- toml_datetime 0.7.3
0/0        5/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- serde_core 1.0.225
0/0        72/72        0/0    0/0     8/8     2/2         0/0           0/0    0/0    0/0    0/0     !  |-- toml_parser 1.0.4
0/1        0/7          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstream 0.6.20
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle 1.0.11
0/0        68/72        0/0    0/0     16/18   4/4         0/0           0/0    0/0    0/0    0/0     !  |   `-- winnow 0.7.13
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- anstyle 1.0.11
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- is_terminal_polyfill 1.70.1
0/41       0/2421       0/2    0/0     0/147   0/4         0/0           0/0    0/0    0/0    0/0     ?  |       `-- memchr 2.7.5
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           `-- log 0.4.28
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               `-- serde 1.0.225
0/0        5/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |                   |-- serde_core 1.0.225
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                   `-- serde_derive 1.0.225
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                       |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                       |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                       |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                       |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |                       `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                           |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                           |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |                           `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) |-- toml_writer 1.0.4
0/0        68/72        0/0    0/0     16/18   4/4         0/0           0/0    0/0    0/0    0/0     !  `-- winnow 0.7.13

0/44       145/2747     0/6    0/0     24/178  6/24        0/0           0/0    0/0    0/0    0/0

```

### toml_edit-0.23.7

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  toml_edit 0.23.7
0/1        0/7          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- anstream 0.6.20
0/0        0/17         0/0    0/0     0/0     0/1         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle-parse 0.2.7
0/0        0/1          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- utf8parse 0.2.2
0/0        0/22         0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle-query 1.1.4
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle 1.0.11
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- colorchoice 1.0.4
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- is_terminal_polyfill 1.70.1
0/0        0/1          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- utf8parse 0.2.2
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- anstyle 1.0.11
0/0        78/83        0/0    0/0     0/0     5/6         0/0           0/0    0/0    0/0    0/0     !  |-- indexmap 2.11.4
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- equivalent 1.0.2
1/1        1244/1513    17/22  1/1     72/85   16/16       0/0           0/0    0/0    0/0    0/0     !  |   |-- hashbrown 0.16.0
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   |-- equivalent 1.0.2
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |   `-- serde 1.0.225
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       |-- serde_core 1.0.225
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |       `-- serde_derive 1.0.225
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- proc-macro2 1.0.101
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |   `-- unicode-ident 1.0.19
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |-- quote 1.0.40
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |           |   `-- proc-macro2 1.0.101
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?  |   |           `-- syn 2.0.106
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               |-- proc-macro2 1.0.101
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               |-- quote 1.0.40
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |               `-- unicode-ident 1.0.19
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde_core 1.0.225
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- serde_core 1.0.225
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) |-- toml_datetime 0.7.3
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde_core 1.0.225
0/0        72/72        0/0    0/0     8/8     2/2         0/0           0/0    0/0    0/0    0/0     !  |-- toml_parser 1.0.4
0/1        0/7          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstream 0.6.20
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- anstyle 1.0.11
0/0        68/72        0/0    0/0     16/18   4/4         0/0           0/0    0/0    0/0    0/0     !  |   `-- winnow 0.7.13
0/0        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- anstyle 1.0.11
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- is_terminal_polyfill 1.70.1
0/41       0/2421       0/2    0/0     0/147   0/4         0/0           0/0    0/0    0/0    0/0     ?  |       `-- memchr 2.7.5
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           `-- log 0.4.28
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               `-- serde 1.0.225
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     :) |-- toml_writer 1.0.4
0/0        68/72        0/0    0/0     16/18   4/4         0/0           0/0    0/0    0/0    0/0     !  `-- winnow 0.7.13

1/45       1462/4343    17/28  1/1     96/263  27/46       0/0           0/0    0/0    0/0    0/0

```

### tracing-0.1.41

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        14/14        1/1    0/0     0/0     4/4         0/0           0/0    0/0    0/0    0/0     !  tracing 0.1.41
0/2        0/18         0/1    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- log 0.4.28
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   `-- serde 1.0.228
0/0        0/5          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       |-- serde_core 1.0.228
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |       `-- serde_derive 1.0.228
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |           |-- proc-macro2 1.0.103
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |           |   `-- unicode-ident 1.0.22
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |           |-- quote 1.0.42
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |           |   `-- proc-macro2 1.0.103
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  |           `-- syn 2.0.111
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |               |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |               |-- quote 1.0.42
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |               `-- unicode-ident 1.0.22
0/0        11/191       0/0    0/0     2/2     1/6         0/0           0/0    0/0    0/0    0/0     !  |-- pin-project-lite 0.2.16
0/1        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- tracing-attributes 0.1.30
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |   |-- quote 1.0.42
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  |   `-- syn 2.0.111
0/0        77/98        1/5    0/0     2/2     7/11        0/0           0/0    0/0    0/0    0/0     !  `-- tracing-core 0.1.34
0/0        81/124       5/9    0/0     3/5     12/15       0/0           0/0    0/0    0/0    0/0     !      `-- once_cell 1.21.3

0/3        289/561      10/19  0/0     12/14   37/49       0/0           0/0    0/0    0/0    0/0

```

### tracing-attributes-0.1.30

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/1        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  tracing-attributes 0.1.30
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |-- proc-macro2 1.0.95
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- unicode-ident 1.0.5
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  |-- quote 1.0.40
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !  |   `-- proc-macro2 1.0.95
0/0        88/88        3/3    0/0     2/2     13/13       0/0           0/0    0/0    0/0    0/0     !  `-- syn 2.0.101
0/0        14/14        0/0    0/0     3/3     0/0         0/0           0/0    0/0    0/0    0/0     !      |-- proc-macro2 1.0.95
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- quote 1.0.40
0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !      `-- unicode-ident 1.0.5

0/1        106/106      3/3    0/0     5/5     13/13       0/0           0/0    0/0    0/0    0/0

```

### tracing-core-0.1.34

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        77/98        1/5    0/0     2/2     7/11        0/0           0/0    0/0    0/0    0/0     !  tracing-core 0.1.34
0/0        81/124       5/9    0/0     3/5     12/15       0/0           0/0    0/0    0/0    0/0     !  `-- once_cell 1.21.3

0/0        158/222      6/14   0/0     5/7     19/26       0/0           0/0    0/0    0/0    0/0

```

### unicode-ident-1.0.22

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     !  unicode-ident 1.0.22

0/0        4/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_aarch64_gnullvm-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_aarch64_gnullvm 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_aarch64_msvc-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_aarch64_msvc 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_i686_gnu-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_i686_gnu 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_i686_msvc-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_i686_msvc 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_x86_64_gnu-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_x86_64_gnu 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_x86_64_gnullvm-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_x86_64_gnullvm 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### windows_x86_64_msvc-0.53.1

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  windows_x86_64_msvc 0.53.1

0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0

```

### zerocopy-0.9.0-alpha.0

```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols:
    :) = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ?  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    !  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Ptr Derefs  Unsafe Calls  Core  Alloc  Std  Other  Dependency

4/4        251/251      25/25  15/15   25/25   3/3         0/0           0/0    0/0    0/0    0/0     !  zerocopy 0.9.0-alpha.0
0/1        0/6          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?  `-- zerocopy-derive 0.9.0-alpha.0
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- proc-macro2 1.0.103
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |   `-- unicode-ident 1.0.22
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?      |-- quote 1.0.42
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?      |   `-- proc-macro2 1.0.103
0/0        0/88         0/3    0/0     0/2     0/13        0/0           0/0    0/0    0/0    0/0     ?      `-- syn 2.0.111
0/0        0/14         0/0    0/0     0/3     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- proc-macro2 1.0.103
0/0        0/0          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          |-- quote 1.0.42
0/0        0/4          0/0    0/0     0/0     0/0         0/0           0/0    0/0    0/0    0/0     ?          `-- unicode-ident 1.0.22

4/5        251/363      25/28  15/15   25/30   3/16        0/0           0/0    0/0    0/0    0/0

```
