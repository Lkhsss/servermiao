# Servermiao
简单的http服务器。kb级别！

## 食用方法
```bash
./servermiao
```
此命令将启动一个简单的http服务器，默认目录为工作目录，默认端口为8000。将自动打开工作目录下的`index.html`文件。

在命令后增加端口和路径将自动识别添加。如`servermiao /test 80`将在工作目录下的`test`文件夹启动端口为80的服务器。

> 当传入多个可别解析为目录和端口的参数时，将默认使用最后传入的有效参数。

## 构建
```bash
cargo build -r
```

> 建议放到`C:\Windows\System32`食用更加方便哦!