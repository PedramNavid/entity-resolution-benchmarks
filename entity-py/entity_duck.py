import duckdb
import logging

FORMAT = logging.Formatter("%(asctime)s %(message)s")
log = logging.getLogger(__name__)
log.setLevel(logging.INFO)
ch = logging.StreamHandler()
ch.setFormatter(FORMAT)
log.addHandler(ch)

con = duckdb.connect(":memory:")


def read_files(con=con):
    con.execute("create or replace table dblp as SELECT * FROM 'data/DBLP2utf8.csv'")
    con.execute("create or replace table scholar as SELECT * FROM 'data/ACM.csv'")


def tokenize(string, n=20):
    con.execute(
        """ 
    select *,
    str_split(lower(regexp_replace(authors, '[^\w\,]', '', 'g'), ',') as authors
    from dblp
    """
    )


if __name__ == "__main__":
    log.info("Reading files")
    read_files()
    log.info("Done reading files")
