# Xmind to all

一个Rust编写、简易的Xmind到xlsx转换器。可以轻松地将你的测试用例从xmind文件转换成excel文件。

## 快速开始

### 安装

从releas页面下载对应系统和架构的程序包并解压。（注意：Windows下解压获取的exe文件可能会被识别为病毒，可以在Windows Defence中恢复。）

### 配置文件

在转换前，需要编辑config目录下的config.toml文件。需要注意的是根目录`root_title`中无需包含你的需求名称，因为你的需求名称就是xmind的根主题。

### 使用方法

1. 将xmind文件复制input目录下（可选）。
2. 运行可执行程序，运行后通过命令和回车与程序进行交互，当前仅支持2个命令`process`和`exit`。
3. 转换：`process <path || filename>`
   如果你做了第一步，那么你可以只写文件名就可以转换，ex: `process tmp.xmind`。
   如果你没有做第一步，那么你也可以使用绝对路径来执行转换，ex: `process C:\xxx\xxx\xxx`
4. 在output目录中获取转换后的xlsx文件。
5. 退出：`exit`或直接关闭命令行窗口。

### 注意事项

1. 已知用例缺少步骤结果等主题，不会导致报错，但是会在转换后的xslx文件中留空，在上传用例时会提示无效用例，需注意。
2. 无需再设置分隔符，当前可以识别每个目录。
