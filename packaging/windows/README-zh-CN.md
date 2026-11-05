# OpenPhoto 简体中文版 · Windows x64 便携版

本压缩包适用于 Windows 10/11 64 位（x64），采用 MSVC 编译。无需安装 Rust、付费 SDK 或安装器。请先将整个 ZIP 解压到有写入权限的目录，再运行 `openphoto.exe`；不要直接在压缩包内运行。程序未进行商业代码签名，Windows 可能显示未知发布者提示；请核对下载来源与发布页提供的 SHA-256 后再决定是否运行。

## 打开与保存图片

双击 `openphoto.exe` 启动简体中文图形界面。使用“文件”菜单打开图片，也可以将图片拖到程序窗口。命令行可传入一个或多个图片路径：

```powershell
.\openphoto.exe "C:\图片\示例.png"
```

所有命令行参数均按文件路径处理，未提供 `--help` 等选项。建议使用较新的显卡驱动；图形界面通过 wgpu 渲染，需要兼容的图形设备。便携包不会安装显卡驱动。

项目仍处于早期开发阶段。处理重要作品时，请保留原件并另存副本；PSD 的复杂效果、文字和图层兼容性需要自行验证。关闭程序前保存修改。

## 使用 MCP 图像处理服务

`op-mcp.exe` 是独立的 MCP 标准输入/输出服务，无需启动图形界面。请在 MCP 客户端配置中使用解压后的实际绝对路径，例如：

```json
{
  "mcpServers": {
    "openphoto": {
      "command": "C:\\Tools\\OpenPhoto\\op-mcp.exe",
      "args": []
    }
  }
}
```

服务使用逐行 JSON-RPC 2.0，公开 `search_tools` 和 `call_tool` 两个工具。先搜索所需操作，再调用对应内部工具。协议方法、工具名及参数名保持英文。

MCP 服务处理的是自身内存中的文档，不会操作当前图形窗口里的文档，也不会自动同步。进程退出后未保存的修改会丢失；退出前调用 `save_image` 或 `export_composite`。服务能按客户端要求读取和写入本机文件，请只接入信任的客户端，并保留原始图片副本。

## 文件与许可

- `openphoto.exe`：图形编辑器。
- `op-mcp.exe`：独立 MCP 图像处理服务。
- `Licenses/LICENSE-MIT`、`Licenses/LICENSE-APACHE`：项目许可，可按相应条款免费使用（包括商业用途）。
- `Licenses/SourceSans3-OFL.txt`、`Licenses/NotoSansCJK-LICENSE.txt`：随程序内嵌字体的许可文本。

这是 OpenPhoto 的非官方简体中文分支，与 Adobe 无关联。字体已嵌入程序，无需单独安装。MSVC C 运行时静态链接，不需要另装 Visual C++ 运行库；Windows 系统组件仍由操作系统提供。

## 校验下载

发布产物同时提供 `.zip.sha256` 文件。可在 PowerShell 中计算 ZIP 校验值并与其比对：

```powershell
Get-FileHash .\OpenPhoto-*-zh-CN-windows-x64.zip -Algorithm SHA256
```

CI 检查格式、Clippy、CPU 测试、编译、包内文件和 MCP 握手；无 GPU 的 CI 不代表完整界面、显卡渲染或截图测试通过。
