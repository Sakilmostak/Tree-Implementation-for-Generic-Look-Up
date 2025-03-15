# Wildcard-Enabled Keychain Tree 

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Minimum Rust Version](https://img.shields.io/badge/rust-1.65%2B-orange)

A generic tree-based data structure for efficient storage and wildcard-enabled searching of keychains. 
Supports patterns like `[a, b, *, d, e]` where `*` matches any key at that position.

## 📖 Overview

**Keychain**: Ordered sequence of keys `[k1, k2, ..., kn]`  
**Value**: Integer stored at terminal nodes  
**Wildcards**: `*` in search patterns matches any single key  

Example Use Case:  
- Store API permissions: `["user", "profile", "read"] = 42`  
- Check access: `["user", "*", "read"] → matches [42]`

## 🌟 Features

- **Tree-Based Storage**: Hierarchical keychain organization
- **Wildcard Search**: `*` matches any key at position
- **Generic Keys**: Works with any hashable key type
- **Efficient Insertion**: O(m) time complexity (m = keychain length)
- **Search Optimization**: Sum are cached for wildcard in every node, optimizing search to the degree  of O(1) for some case eg. [`*`,`*`,`*`,`*`,`*`] 

## 🏗️ Structure

### Insertion Example (`[a, b, c] = 42`)
```plaintext
Root
└── a (depth=1)
    └── b (depth=2)
        └── c (depth=3) → value=42
```

## Wildcard Search (`[a, *, c]`)

```plaintext
Root
└── a 
    ├── x → b (no match)
    └── b 
        ├── c → Match found (42)
        └── d → Continue searching
```

## ⚙️ Complexity
|Operation |Time Complexity |Description                    |
|--------|---------------|----------------------------------|
|Insert	|O(m)	  |m = keychain length                        |
|Search	|O(n*m)	|n = stored keychains, m = pattern length   |

*Space Complexity Note:*  
- **O(n*m)** where:  
  - n = total unique keychains stored  
  - m = average keychain length  
- Worst-case scenario with no shared node paths between keychains
