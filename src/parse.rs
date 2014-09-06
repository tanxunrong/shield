
//use std::fmt::{Formatter,FormatError};
//use std::str::CharRange;

#[deriving(PartialEq,Eq,Clone,Show)]
pub struct Query {
    orig:String,
    table:String
}


#[deriving(PartialEq,Eq,Clone,Show)]
pub enum ParseError {
    DelMethod,
    ErrFormat
}

pub type ParseResult = Result<Query,ParseError>;

impl Query {
    pub fn parse(sql:&str) -> ParseResult {
        let s = sql.trim_chars([' ','\t','\n','\r'].as_slice());
        if s.len() < 6
        {
            return Err(ErrFormat);
        }
        let method = s.slice_chars(0,6);
        if method == "delete" {
            return Err(DelMethod);
        }
        else if method == "update" {
            let v:Vec<&str> = s.split(' ').collect();
            let q = Query{orig:String::from_str(s),table:String::from_str(v[1])};
            return Ok(q);
        }
        else {
            return Err(ErrFormat);
        }
   }
}

#[test]
fn test_sql_parse() {
    let q = "update t_bag set item_id = 0 where uid = 20100;";
    let ret = Query::parse(q);
    println!("parse ret : {}",ret);

    match ret {
        Ok(v) => { assert!(v.table.as_slice() == "t_bag"); } 
        Err(e) => { fail!("expected ok,parse err {}",e); } 
    }
}

