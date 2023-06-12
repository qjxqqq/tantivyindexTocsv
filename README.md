# tantivyindexTocsv
tantivy索引转csv文件

# 使用方法

更改代码中的字段，对应索引中的字段

`header_record.push("title");
header_record.push("author");
header_record.push("publisher");
header_record.push("extension");
header_record.push("filesize");
header_record.push("language");
header_record.push("year");
header_record.push("pages");
header_record.push("isbn");
header_record.push("ipfs_cid");`

`// 获取doc中的字段
let title: String = doc.get_first(schema.get_field("title").unwrap()).unwrap().as_text().unwrap().to_string();
let author: String = doc.get_first(schema.get_field("author").unwrap()).unwrap().as_text().unwrap().to_string();
let publisher: String = doc.get_first(schema.get_field("publisher").unwrap()).unwrap().as_text().unwrap().to_string();
let extension: String = doc.get_first(schema.get_field("extension").unwrap()).unwrap().as_text().unwrap().to_string();
let filesize: String = doc.get_first(schema.get_field("filesize").unwrap()).unwrap().as_u64().unwrap().to_string();
let language: String = doc.get_first(schema.get_field("language").unwrap()).unwrap().as_text().unwrap().to_string();
let year: String = doc.get_first(schema.get_field("year").unwrap()).unwrap().as_u64().unwrap().to_string();
let pages: String = doc.get_first(schema.get_field("pages").unwrap()).unwrap().as_u64().unwrap().to_string();
let isbn: String = doc.get_first(schema.get_field("isbn").unwrap()).unwrap().as_text().unwrap().to_string();
let ipfs_cid: String = doc.get_first(schema.get_field("ipfs_cid").unwrap()).unwrap().as_text().unwrap().to_string();

// 将字段写入CSV记录中
csv_record.push(title);
csv_record.push(author);
csv_record.push(publisher);
csv_record.push(extension);
csv_record.push(filesize);
csv_record.push(language);
csv_record.push(year);
csv_record.push(pages);
csv_record.push(isbn);
csv_record.push(ipfs_cid);`







# 其他

后面有时间会优化下，把字段写入配置文件





