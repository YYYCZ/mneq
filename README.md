# 神秘数字等式构造

## 环境

1. Rust 工具链
2. wasm-pack
3. pnpm

## 编译

首先在 `rust` 目录下把 Rust 工程编译为 wasm 文件，在当前目录下运行

```sh
wasm-pack build -t web -d ../src/lib --release
```

构建静态资源，首次运行需要运行

```sh
pnpm install
```

构建静态资源，运行

```sh
pnpm run build
```

## 部署

最后，把 `dist` 下的文件部署到服务器即可。
