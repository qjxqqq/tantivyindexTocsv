use tantivy::{Index, schema::Schema, DocAddress};
use csv::{Writer, WriterBuilder};
use std::fs::File;
use tantivy::TantivyError;
use std::io;
use anyhow::anyhow;
use tantivy::schema::Value;

fn main() -> anyhow::Result<()> {
    // 打开索引文件
    let index_path = "index";
    let directory = tantivy::directory::MmapDirectory::open(index_path)?;
    let index = Index::open(directory).map_err(|e| anyhow::anyhow!(e))?;

    // 获取schema
    let schema: &Schema = &index.schema();

    // 获取CSV文件句柄
    let csv_file = File::create("output.csv")?;
    let mut csv_writer = WriterBuilder::new().delimiter(b'\x01').has_headers(true).from_writer(csv_file);

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

        // 将字段写入CSV记录中
        csv_record.push(title.replace("\n", " "));
        csv_record.push(author.replace("\n", " "));
        csv_record.push(publisher.replace("\n", " "));
        csv_record.push(extension.replace("\n", " "));
        csv_record.push(filesize.replace("\n", " "));
        csv_record.push(language.replace("\n", " "));
        csv_record.push(year.replace("\n", " "));
        csv_record.push(pages.replace("\n", " "));
        csv_record.push(isbn.replace("\n", " "));
        csv_record.push(ipfs_cid.replace("\n", " "));

        // 将CSV记录写入CSV文件
        csv_writer.write_record(csv_record)?;
    }

    // 关闭CSV文件 结束
    csv_writer.flush()?;
    Ok(())
}
