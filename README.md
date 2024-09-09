# 神秘数字等式构造

## 环境

1. Rust 工具链
2. wasm-pack
3. pnpm

## 编译

首先把 Rust 工程编译为 wasm 文件，在当前目录下运行

```sh
wasm-pack build -t web -d ./mneq-html/src/lib --release
```

构建静态资源，切换到子目录 `mneq-html` 下，首次运行需要运行

```sh
pnpm install
```

构建静态资源，运行

```sh
pnpm run build
```
