# akua (アクア)

这是一个给 Blessing Skin 用的版本切换工具。您可以在 `git clone` 了 Blessing Skin 的仓库中使用这个工具来方便地切换不同的通道。

致力于成为一个即使拥有 [アクア](https://zh.moegirl.org/%E9%98%BF%E5%BA%93%E5%A8%85(%E4%B8%BA%E7%BE%8E%E5%A5%BD%E7%9A%84%E4%B8%96%E7%95%8C%E7%8C%AE%E4%B8%8A%E7%A5%9D%E7%A6%8F)#) 般的智力水平的人也能使用的工具。（大雾）

## 使用方法

### 下载

首先，前往 GitHub releases 页面下载可执行文件。（目前只支持 Linux）

您可以放在任何地方，例如可以位于 `$PATH` 下的，总之方便调用就行。

### 运行

先要 `cd` 到 **皮肤站** 所在目录，然后就可以使用这个工具。

切换完通道以后，工具会智能帮您安装依赖并构建前端资源，无需手动操作。

## 可用的命令

> 直接运行 `akua -h` 也可以获得帮助。

- `akua fast` 切换到 fast 通道，即 Blessing Skin 在 GitHub 上的 `dev` 分支。
- `akua slow` 切换到 slow 通道，即 Blessing Skin 在 GitHub 上的 `master` 分支。
- `akua stable <version>` 切换到某个稳定的、已发布的版本。`version` 即为您想要的版本。

## 已知的一些限制

- Git 仓库只能以 HTTPS 的方式来 `git clone` 下来。如果不是，请修改 remote。

## License

MIT License (c) 2019-present Pig Fang
