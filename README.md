# am-links-modifier
配合 [apple-music-downloader](https://github.com/zhaarey/apple-music-downloader) ([教程](https://zh453t.github.io/music/am/tutorial.html))使用
一种替代的批量下载方案，改变`go run main.go url1 url2 url3 ...`等到全部下载后才处理下载错误的机制
## 使用方法
1. 在 `release` 中下载 `modifier.exe`
2. 在任意位置新建 `xxx.txt`，粘贴 Apple Music 链接（每行最多1个链接，可以有其他无关字符）
3. 将 txt 文件拖放给 `modifier.exe` (让 `modifier.exe` 打开)
4. 打开 txt 文件，复制更改后的内容
5. 在 ZeroTermux 中长按粘贴，运行
