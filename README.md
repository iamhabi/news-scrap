# News Scrap

## 소개
Rust로 작성한 뉴스 사이트 스크랩 프로그램  
스크랩한 뉴스를 저장하기 위해 MySQL을 사용  

### MySQL

- Database: scrap
  - table: news
    - id: INT PRIMARY KEY
    - title: TEXT NOT NULL
    - href: TEXT NOT NULL
    - created: DATE DEFAULT CURRENT_DATE


### read.json
스크랩할 뉴스 사이트를 저장하는 json 파일  

```
{  
    "url": "https://news.exampleurl.com/",  
    "tag": "a",  
    "attr": ["class", "titlelink"]  
},
```

url: 스크랩을 원하는 사이트의 url 입력  
tag: 해당 사이트의 기사 제목의 태그를 입력  
attr: 태그를 특정하기 위한 속성(attributes) 입력

---
/src/sql/db.rs 파일에 database에 접근하기 위한 정보를 저장

```
pub const DB_USER:&'static str = "username";
pub const DB_PASSWORD:&'static str = "password";
pub const DB_IP:&'static str = "localhost";
pub const DB_PORT: u16 = 3306_u16;
pub const DB_NAME:&'static str = "datebase_name";
```

민감한 정보를 숨기기 위하여 이러한 방식을 이용