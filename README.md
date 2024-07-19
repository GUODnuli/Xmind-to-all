# Xmind to all

一个Rust编写、简易的Xmind到xlsx转换器。可以轻松地将你的测试用例从xmind文件转换成excel文件。

## 快速开始

1. 安装：
    从releas页面下载对应系统和架构的程序包并解压。（注意：Windows下解压获取的exe文件可能会被识别为病毒，可以在Windows Defence中恢复。）
2. 使用方法：
   1. 将xmind文件复制input目录下（可选）。
   2. 运行可执行程序，运行后通过命令和回车与程序进行交互，当前仅支持2个命令`process`和`exit`。
   3. 转换：`process <path || filename>`
      如果你做了第一步，那么你可以只写文件名就可以转换，ex: `process tmp.xmind`。
      如果你没有做第一步，那么你也可以使用绝对路径来执行转换，ex: `process C:\xxx\xxx\xxx`
   4. 在output目录中获取转换后的xlsx文件。
   5. 退出：`exit`或直接关闭命令行窗口。
