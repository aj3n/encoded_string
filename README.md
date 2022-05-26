# encoding_string

`Encoded<Encoder>` 是对标准库`String`和`encoding_rs`的包装, 使其支持对非utf-8编码的字节流实现`Deserialize`/`Serialize`

## when shouldn't I use this crate

如果数据交换格式是可打印的(`json`, `toml`, `csv`等)非`utf8`内容, 应该优先考虑使用`encoding_rs_io`包装一个转码后的字节流解析内容, 而不是使用这个库包装非utf8编码的文本;

## when should I use this crate

如果数据交换格式是不可打印的，并且存在编码混用的情况；
