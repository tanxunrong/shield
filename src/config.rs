
use std::collections::{HashMap};

#[deriving(PartialEq,Eq,Clone,Show)]
enum PartType {
    DIV = 1, // divide to int
    MOD = 2// remainder
}

#[deriving(PartialEq,Eq,Clone,Show)]
struct TableCfg {
    ptype:PartType,
    num:uint,
    col:String
}

#[deriving(PartialEq,Eq,Clone,Show)]
struct Table {
    pri : Vec<String>, // primary keys
    name : String,
    cfg : TableCfg
}

#[deriving(PartialEq,Eq,Clone,Show)]
pub struct Conf {
    tbls :  HashMap<String,Table>,
    db:String
}

pub fn new(dbname:& str) -> Conf {
    let t = HashMap::new(); 
    Conf{tbls:t,db:String::from_str(dbname)}
}

impl Conf {
    fn add_tbl(&mut self,tb:&str,col:&str,t:PartType,num:uint) {
        let cfg = TableCfg{ptype:t,num:num,col:String::from_str(col)};
        let mut t = Table { pri:Vec::new(),name:String::from_str(tb),cfg:cfg};
        t.pri.push(String::from_str(tb));
        self.tbls.insert(String::from_str(tb),t);
    }
}

#[test]
fn test_conf_init() {
    let mut c = new("pirate");
    c.add_tbl("bag","uid",MOD,10);
}
