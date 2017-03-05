use nom::is_alphanumeric;
use nom::{IResult, Err, ErrorKind, Needed};

use std::str;
use std::str::FromStr;

/// Matches any SQL reserved keyword
named!(pub sql_keyword<&[u8], &[u8]>,
    complete!(chain!(
        kw: alt_complete!(
          caseless_tag!("ABORT")
        | caseless_tag!("ACTION")
        | caseless_tag!("ADD")
        | caseless_tag!("AFTER")
        | caseless_tag!("ALL")
        | caseless_tag!("ALTER")
        | caseless_tag!("ANALYZE")
        | caseless_tag!("AND")
        | caseless_tag!("AS")
        | caseless_tag!("ASC")
        | caseless_tag!("ATTACH")
        | caseless_tag!("AUTOINCREMENT")
        | caseless_tag!("BEFORE")
        | caseless_tag!("BEGIN")
        | caseless_tag!("BETWEEN")
        | caseless_tag!("BY")
        | caseless_tag!("CASCADE")
        | caseless_tag!("CASE")
        | caseless_tag!("CAST")
        | caseless_tag!("CHECK")
        | caseless_tag!("COLLATE")
        | caseless_tag!("COLUMN")
        | caseless_tag!("COMMIT")
        | caseless_tag!("CONFLICT")
        | caseless_tag!("CONSTRAINT")
        | caseless_tag!("CREATE")
        | caseless_tag!("CROSS")
        | caseless_tag!("CURRENT_DATE")
        | caseless_tag!("CURRENT_TIME")
        | caseless_tag!("CURRENT_TIMESTAMP")
        | caseless_tag!("DATABASE")
        | caseless_tag!("DEFAULT")
        | caseless_tag!("DEFERRABLE")
        | caseless_tag!("DEFERRED")
        | caseless_tag!("DELETE")
        | caseless_tag!("DESC")
        | caseless_tag!("DETACH")
        | caseless_tag!("DISTINCT")
        | caseless_tag!("DROP")
        | caseless_tag!("EACH")
        | caseless_tag!("ELSE")
        | caseless_tag!("END")
        | caseless_tag!("ESCAPE")
        | caseless_tag!("EXCEPT")
        | caseless_tag!("EXCLUSIVE")
        | caseless_tag!("EXISTS")
        | caseless_tag!("EXPLAIN")
        | caseless_tag!("FAIL")
        | caseless_tag!("FOR")
        | caseless_tag!("FOREIGN")
        | caseless_tag!("FROM")
        | caseless_tag!("FULL")
        | caseless_tag!("GLOB")
        | caseless_tag!("GROUP")
        | caseless_tag!("HAVING")
        | caseless_tag!("IF")
        | caseless_tag!("IGNORE")
        | caseless_tag!("IMMEDIATE")
        | caseless_tag!("IN")
        | caseless_tag!("INDEX")
        | caseless_tag!("INDEXED")
        | caseless_tag!("INITIALLY")
        | caseless_tag!("INNER")
        | caseless_tag!("INSERT")
        | caseless_tag!("INSTEAD")
        | caseless_tag!("INTERSECT")
        | caseless_tag!("INTO")
        | caseless_tag!("IS")
        | caseless_tag!("ISNULL")
        | caseless_tag!("JOIN")
        | caseless_tag!("KEY")
        | caseless_tag!("LEFT")
        | caseless_tag!("LIKE")
        | caseless_tag!("LIMIT")
        | caseless_tag!("MATCH")
        | caseless_tag!("NATURAL")
        | caseless_tag!("NO")
        | caseless_tag!("NOT")
        | caseless_tag!("NOTNULL")
        | caseless_tag!("NULL")
        | caseless_tag!("OF")
        | caseless_tag!("OFFSET")
        | caseless_tag!("ON")
        | caseless_tag!("OR")
        | caseless_tag!("ORDER")
        | caseless_tag!("OUTER")
        | caseless_tag!("PLAN")
        | caseless_tag!("PRAGMA")
        | caseless_tag!("PRIMARY")
        | caseless_tag!("QUERY")
        | caseless_tag!("RAISE")
        | caseless_tag!("RECURSIVE")
        | caseless_tag!("REFERENCES")
        | caseless_tag!("REGEXP")
        | caseless_tag!("REINDEX")
        | caseless_tag!("RELEASE")
        | caseless_tag!("RENAME")
        | caseless_tag!("REPLACE")
        | caseless_tag!("RESTRICT")
        | caseless_tag!("RIGHT")
        | caseless_tag!("ROLLBACK")
        | caseless_tag!("ROW")
        | caseless_tag!("SAVEPOINT")
        | caseless_tag!("SELECT")
        | caseless_tag!("SET")
        | caseless_tag!("TABLE")
        | caseless_tag!("TEMP")
        | caseless_tag!("TEMPORARY")
        | caseless_tag!("THEN")
        | caseless_tag!("TO")
        | caseless_tag!("TRANSACTION")
        | caseless_tag!("TRIGGER")
        | caseless_tag!("UNION")
        | caseless_tag!("UNIQUE")
        | caseless_tag!("UPDATE")
        | caseless_tag!("USING")
        | caseless_tag!("VACUUM")
        | caseless_tag!("VALUES")
        | caseless_tag!("VIEW")
        | caseless_tag!("VIRTUAL")
        | caseless_tag!("WHEN")
        | caseless_tag!("WHERE")
        | caseless_tag!("WITH")
        | caseless_tag!("WITHOUT")
    ) ~
    peek!(one_of!(" \n;(\t,=")),
    || { kw }
    ))
);