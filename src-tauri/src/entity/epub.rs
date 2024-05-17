use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use epub::doc::EpubDoc;
use scraper::{Html, Selector};

use super::bookinfo::BookInfo;

#[derive(Debug)]
pub struct Epub {
    /// Metadata
    /// Epub的公共信息，可以从外部直接获取
    pub info: BookInfo,

    /// 原始数据
    data: EpubDoc<BufReader<File>>,

    /// 目录信息
    catalog: BTreeMap<usize, (PathBuf, String)>,

    /// 目录到资源的映射
    resources_mapping: BTreeMap<usize, usize>,

    /// 当前页面
    current_page: usize,
}

impl Epub {
    pub fn new(path: &PathBuf) -> Self {
        let data = EpubDoc::new(path).unwrap();
        let catalog = Self::init_catalog(&data);
        let mapping = Self::init_resources_mapping(&data, &catalog);

        Epub {
            info: BookInfo::new(path.clone()),
            data,
            catalog,
            resources_mapping: mapping,
            current_page: 0,
        }
    }

    /// 初始化目录
    ///
    /// 返回一个BTreeMap<usize, (PathBuf, String)>
    /// key: 章节索引;
    /// value: (PathBuf: 章节文件路径; String: 章节名)
    fn init_catalog(data: &EpubDoc<BufReader<File>>) -> BTreeMap<usize, (PathBuf, String)> {
        let toc = &data.toc;
        let mut key: usize = 0;
        let mut catalog = BTreeMap::new();

        for item in toc {
            catalog.insert(key, (item.content.clone(), item.label.clone()));
            key += 1;
        }

        catalog
    }

    fn init_resources_mapping(
        data: &EpubDoc<BufReader<File>>,
        catalog: &BTreeMap<usize, (PathBuf, String)>,
    ) -> BTreeMap<usize, usize> {
        let mut index: usize = 0;
        let mut spine_to_resource = BTreeMap::new();

        for current in &data.spine {
            let (path, _) = &data.resources.get(current).unwrap();
            spine_to_resource.insert(path.clone(), index);
            index += 1;
        }

        catalog
            .iter()
            .map(|(index, (path, _))| {
                let target = spine_to_resource.get(path).unwrap();

                (index.clone(), target.clone())
            })
            .collect::<BTreeMap<usize, usize>>()
    }

    pub fn get_catalog(&self) -> Vec<String> {
        let mut catalog = Vec::new();
        for (_, (_, label)) in &self.catalog {
            catalog.push(label.clone());
        }

        catalog
    }

    pub fn go_next(&mut self) -> bool {
        self.set_current_page(self.current_page + 1)
    }

    pub fn go_prev(&mut self) -> bool {
        // 直接减1容易发生下溢出, 需要判断当前是否为0
        if self.current_page == 0 {
            return false;
        }

        self.set_current_page(self.current_page - 1)
    }

    pub fn set_current_page(&mut self, num: usize) -> bool {
        // self.data.get_num_pages()获取到的是resources的数量,
        // 而该数量与真实的章节数量是不同的, 无法使用该方法获取章节总数
        // if num <= self.data.get_num_pages() {

        println!("{}", num);
        if num < self.catalog.len() {
            self.current_page = num;

            return true;
        }

        false
    }

    pub fn get_current_page(&mut self) -> String {
        // get_current_with_epub_uris()函数获取的页面中,
        // 资源使用的EpubURI, 而get_resource_by_path()函数获取的页面中,
        // 资源使用的是相对路径, 所以需要使用get_current_with_epub_uris()函数获取页面.
        // 则需要将我们维护的catalog与EpubDoc中的资源进行匹配, 才能获取到正确的页面.
        self.data.set_current_page(
            self.resources_mapping
                .get(&self.current_page)
                .unwrap()
                .clone(),
        );
        let page = self.data.get_current_with_epub_uris().unwrap();

        let page = String::from_utf8(page).unwrap();

        let document = Html::parse_document(&page);
        let body_selector = Selector::parse("body").unwrap();
        let body = document.select(&body_selector).next().unwrap();
        let content = body.inner_html();
        content
    }

    /// 获取Epub的css
    ///
    /// 返回一个HashMap<String, Vec<u8>>
    /// key为文件名，value为文件内容
    pub fn get_css(&mut self) -> HashMap<String, String> {
        let mut css_list = HashMap::new();

        for (name, (path, mime)) in self.data.resources.clone() {
            if mime == "text/css" {
                let css = String::from_utf8(self.data.get_resource_by_path(path).unwrap()).unwrap();
                css_list.insert(name + ".css", css);
            }
        }

        css_list
    }
}
