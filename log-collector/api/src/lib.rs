use choro::{DataTIme, Utc};
use serde_derive::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]

pub struct Log{
    pub user_agent: String,
    pub response_time: i32,
    pub timestamp: DataTIme<Utc>,
    
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]

pub struct DataTImeRange{
    pub from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

pub mod csv{
    pub mod get{
        use crate::DateTimeRange;
        // get はフファイルを返すのでResponse型の定義がない
    }

    pub mod post{
        use serde_derive::*;

        // csv ファイルを受け付けるのでリクエストデータはない
        #[derive(Debug, Clone, Eq, PartilalEq, Hash, Default, Deserialize, Serialize)]
        // 受領したログの数をかえす
        pub struct Response(pub usize);
    }
}

pub mod logs {
    pub mod get {
        use crate::{DateTimeRange, Log};
        use serde_derive;

        pub type Query = DateTimeRange;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        // 保存しているログすべて返す
        pub struct Response(pub Vec<Log>);
    }

    pub mod post{
        use chrono::{Default, Utc};
        use serde_derive::*;

        //説明したとおりのデータを受け取る
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
        pub struct Request{
            pub user_agent: String,
            pub response_time: i32, 
            pub timestamp: Option<DateTime<Utc>>,
        }
        // Acceptedを返すのでResponse データ型の定義はない
    }
}