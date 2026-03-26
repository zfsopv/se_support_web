---
title: 部署到 Cloudflare Pages
description: 如何将项目部署到 Cloudflare Pages
template: doc
---

本文档说明如何将本项目部署到 Cloudflare Pages。

## 前置要求

1. Cloudflare 账号
2. Node.js 环境
3. wrangler CLI 工具

## 安装 wrangler

```bash
pnpm install -g wrangler
```

## 登录 Cloudflare

```bash
wrangler login
```

执行后会打开浏览器，登录并授权。

## 部署步骤

### 1. 构建项目

```bash
pnpm run build
```

### 2. 部署到 Cloudflare Pages

```bash
pnpm run deploy
```

或使用 wrangler 直接部署：

```bash
wrangler pages deploy dist
```

首次部署时会要求输入项目名称和详细信息。

## 自动部署脚本

项目已配置自动部署脚本，运行以下命令即可完成构建和部署：

```bash
pnpm run deploy
```

## SEO 配置

项目已配置禁止搜索引擎索引，通过 `public/robots.txt` 实现：

```
User-agent: *
Disallow: /
```

如需启用搜索引擎索引，请删除或修改此文件。