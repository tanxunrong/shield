
//use std::fmt::{Formatter,FormatError};
//use std::str::CharRange;

#[deriving(PartialEq,Eq,Clone,Show)]
pub struct Query {
    orig:String,
    table:String,
    set:String,
    wheres:String
}


#[deriving(PartialEq,Eq,Clone,Show)]
pub enum ParseError {
    DelMethod,
    ErrFormat,
    InvalidSet,
    InvalidWhere
}

pub type ParseResult = Result<Query,ParseError>;

fn keyToLowCase(sql:&str) -> str {
    let keyre = regex!(r"[update|insert|delete|select|set|into|where|group|by|table|values|from|order|limit|asc|desc]i");
impl Query {
    pub fn parse(sql:&str) -> ParseResult {
        let s = sql.trim_chars([' ','\t','\n','\r'].as_slice());
        if s.len() < 6
        {
            return Err(ErrFormat);
        }
        let mut method = s.slice_chars(0,6).to_ascii_lower().as_slice();
        if method == "delete" {
            return Err(DelMethod);
        }
        else if method == "update" {
            let mut v:Vec<&str> = s.split(' ').filter(|x| { x == " " }).collect();
            if v.len() < 3 {
                return Err(ErrFormat);
            }
            let tbl = String::from_str(v[1]);
            if v[2].to_ascii_lower().as_slice() != "set" {
                return Err(InvalidSet);
            }

            v = s.split_str("set"
            let mut set = String::new();
            let mut whereIdx = 3;
            let mut hasWhere = false;
            while whereIdx < v.len() {
                if v[whereIdx] != "where" {
                    set = set.append(v[whereIdx]).append(" ");
                    whereIdx += 1;
                }
                else {
                    hasWhere = true;
                    break;
                }
            }

            if set.is_empty() {
                return Err(InvalidSet);
            }

            whereIdx += 1;

            if !hasWhere || whereIdx == v.len() {
                return Err(InvalidWhere);
            }

            let mut wheres = String::new();
            while whereIdx < v.len() {
                wheres = wheres.append(v[whereIdx]).append(" ");
                whereIdx += 1;
            }

            let q = Query{orig:String::from_str(s),table:tbl,set:set,wheres:wheres};
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
        Ok(v) => { 
            assert!(v.table.as_slice() == "t_bag");
            assert!(v.set.as_slice() == "item_id = 0 ");
            assert!(v.wheres.as_slice() == "uid = 20100; ");
        } 
        Err(e) => { fail!("expected ok,parse err {}",e); } 
    }
}

