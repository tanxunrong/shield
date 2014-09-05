
#[crate_type="lib"]
use std::collections::{HashMap};

enum PartType {
    DIV, // divide to int
    MOD // remainder
}

struct TableCfg {
    ptype:PartType,
    num:uint,
    col:String
}

struct Table {
    pri : Vec<String>, // primary keys
    name : String,
    cfg : TableCfg
}

struct Conf {
    tbls : HashMap<String,Table>,
    db:String
}

pub fn new(dbname:& str) -> Conf {
    Conf{tbls:HashMap::new(),db:String::from_str(dbname)}
}

impl Conf {
    fn add_tbl(&self,tb:&str,col:&str,t:PartType,num:uint) {
        let cfg = TableCfg{ptype:t,num:num,col:String::from_str(col)};
        let t = Table { pri:Vec::new(),name:String::from_str(tb),cfg:cfg};
        self.tbls.insert(String::from_str(tb),t);
    }
}

#[cfg(test)]
fn test() {
    let c = new("pirate");
    c.add_tbl("bag","uid",MOD,10);
}
