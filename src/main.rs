use tantivy::{Index, schema::Schema, DocAddress};
use csv::{Writer, WriterBuilder};
use std::fs::File;
use tantivy::TantivyError;
use std::io;
use anyhow::anyhow;
use tantivy::schema::Value;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    // 打开索引文件
    let index_path = "index";
    let directory = tantivy::directory::MmapDirectory::open(index_path)?;
    let index = Index::open(directory).map_err(|e| anyhow::anyhow!(e))?;

    // 获取schema
    let schema: &Schema = &index.schema();

    // 获取CSV文件句柄
    let csv_file = File::create("output.csv")?;
    let mut csv_writer = WriterBuilder::new().delimiter(b'\x07').has_headers(true).from_writer(csv_file);

    // 获取所有的doc
    let searcher = index.reader()?.searcher();
    let num_docs = searcher.num_docs();
    let top_docs = searcher.search(&tantivy::query::AllQuery, &tantivy::collector::TopDocs::with_limit(num_docs as usize)).map_err(|e| anyhow::anyhow!(e))?;
    let mut a:u64 = 0;

    // 添加标题行
    let mut header_record = vec![];
    header_record.push("title");
    header_record.push("author");
    header_record.push("publisher");
    header_record.push("extension");
    header_record.push("filesize");
    header_record.push("language");
    header_record.push("year");
    header_record.push("pages");
    header_record.push("isbn");
    header_record.push("ipfs_cid");
    csv_writer.write_record(header_record)?;

    // 正则表达式匹配不可见字符
    let regex = Regex::new(r"[\p{Z}\p{C}]").unwrap(); // 匹配空格和不可见字符

    // 遍历所有的doc，将数据写入CSV文件
    for (_score, doc_address) in top_docs {
        if a % 10000 == 0 {
            println!("{}/{}", a, num_docs);
        }
        a += 1;
        let doc = searcher.doc(doc_address)?;
        let mut csv_record = vec![];

        // 获取doc中的字段 注意字段的类型
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

        // 将`\011`替换为空格
        let cleaned_title = title.replace("\011", " ");
        let cleaned_author = author.replace("\011", " ");
        let cleaned_publisher = publisher.replace("\011", " ");
        let cleaned_extension = extension.replace("\011", " ");
        let cleaned_filesize = filesize.replace("\011", " ");
        let cleaned_language = language.replace("\011", " ");
        let cleaned_year = year.replace("\011", " ");
        let cleaned_pages = pages.replace("\011", " ");
        let cleaned_isbn = isbn.replace("\011", " ");
        let cleaned_ipfs_cid = ipfs_cid.replace("\011", " ");

        // 使用正则表达式替换不可见字符
        let cleaned_title = regex.replace_all(&cleaned_title, "");
        let cleaned_author = regex.replace_all(&cleaned_author, "");
        let cleaned_publisher = regex.replace_all(&cleaned_publisher, "");
        let cleaned_extension = regex.replace_all(&cleaned_extension, "");
        let cleaned_filesize = regex.replace_all(&cleaned_filesize, "");
        let cleaned_language = regex.replace_all(&cleaned_language, "");
        let cleaned_year = regex.replace_all(&cleaned_year, "");
        let cleaned_pages = regex.replace_all(&cleaned_pages, "");
        let cleaned_isbn = regex.replace_all(&cleaned_isbn, "");
        let cleaned_ipfs_cid = regex.replace_all(&cleaned_ipfs_cid, "");

        // 将字段写入CSV记录中
        csv_record.push(cleaned_title.to_string());
        csv_record.push(cleaned_author.to_string());
        csv_record.push(cleaned_publisher.to_string());
        csv_record.push(cleaned_extension.to_string());
        csv_record.push(cleaned_filesize.to_string());
        csv_record.push(cleaned_language.to_string());
        csv_record.push(cleaned_year.to_string());
        csv_record.push(cleaned_pages.to_string());
        csv_record.push(cleaned_isbn.to_string());
        csv_record.push(cleaned_ipfs_cid.to_string());

        // 将CSV记录写入CSV文件
        csv_writer.write_record(csv_record)?;
    }

    // 关闭CSV文件 结束
    csv_writer.flush()?;
    Ok(())
}
