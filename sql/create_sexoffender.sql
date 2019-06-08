CREATE TABLE IF NOT EXISTS SexOffender (
    id Integer,
    name,
    age,
    dateOfBirth,
    state,
    aliases,
    offenses,
    addresses,
    photos,
    personalDetails
);

CREATE UNIQUE INDEX sex_off_idx ON SexOffender (id, state);

DROP TABLE  SexOffender;

select * from SexOffender order by id;

Delete from SexOffender;

CREATE TABLE PPH (ID INTEGER, DATA BLOB)

select * from SexOffender;
--select * from photos limit 20;
select sx.id, sx.name, p.id, p.name, p.state from photos p
join SexOffender sx on sx.id = p.id and sx.state = p.state

Select data from photos where name='1000_1_1.jpg'