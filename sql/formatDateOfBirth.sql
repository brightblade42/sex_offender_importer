update SexOffender set dateOfBirth = TRIM(dateOfBirth);
-- SPECIAL CASE of Extra crappy. This fixes double and triple entries like: 01-31-198201-31-1982 There aren't many of these.
update SexOffender set dateOfBirth = substr(dateOfBirth, 1,2) || "/" || substr(dateOfBirth, 4,2) || "/" || substr(dateOfBirth, 7,4) where dateOfBirth like "%-______-%";; -- where name like "Elwood%James"; --dateOfBirth  like "%-______-%";--like "%__-__-____%"; --group by state  --mm-dd-yyyy format
--select dateOfBirth, substr(dateOfBirth, 6,2) || "/" || substr(dateOfBirth, 9,2) || "/" ||  substr(dateOfBirth, 1,4)  as yyyy from SexOffender where dateOfBirth like "%____-__-__%"; -- group by state
-- convert format yyyy-mm-dd -> mm/dd/yyyy
update SexOffender set dateOfBirth = substr(dateOfBirth, 6,2) || "/" || substr(dateOfBirth, 9,2) || "/" ||  substr(dateOfBirth, 1,4) where dateOfBirth like "%____-__-__%";
-- convert mm-dd-yyyy -> mm/dd/yyyy
update SexOffender set dateOfBirth = replace(dateOfBirth, "-", "/") where dateOfBirth like "%__-__-____%"; --group by state  --mm-dd-yyyy format
-- convert yyyymmdd  -> mm/dd/yyyy
update SexOffender set dateOfBirth = substr(dateOfBirth, 5,2) || "/" || substr(dateOfBirth, 7,2) || "/" || substr(dateOfBirth, 1,4) where dateOfBirth like "________" and dateOfBirth not like "%/%";
-- add leading zero to single digit months.
update SexOffender set dateOfBirth = "0" || dateOfBirth where dateOfBirth like "_/%";
-- add leading zero to single digit days (must do this AFTER adding leading zero to single digit months.
update SexOffender set dateOfBirth = substr(dateOfBirth, 1, 2) || "/" || "0" || substr(dateOfBirth, 4,1) || "/" || substr(dateOfBirth, 6, 4)  where dateOfBirth  like "%/_/%"; -- and not state = "CA"; -- and dateOfBirth not like "__";
update SexOffender set dateOfBirth = "01/" || substr(dateOfBirth, 9,2) || "/" || substr(dateOfBirth, 13, 4) where dateOfBirth like "January___,%";  --double digit day
update SexOffender set  dateOfBirth ="01/0" || substr(dateOfBirth, 9,1) || "/" || substr(dateOfBirth, 12, 4) where dateOfBirth like "January__,%";  -- single digit day
--select dateOfBirth, "02/" || substr(dateOfBirth, 10,2) || "/" || substr(dateOfBirth, 14, 4)  from SexOffender where dateOfBirth like "February___,%";  --double digit day
--select dateOfBirth,  "02/0" || substr(dateOfBirth, 10,1) || "/" || substr(dateOfBirth, 13, 4) from SexOffender where dateOfBirth like "February__,%";  --single digit day
update SexOffender set dateOfBirth = "02/" || substr(dateOfBirth, 10,2) || "/" || substr(dateOfBirth, 14, 4)  where dateOfBirth like "February___,%";  --double digit day
update SexOffender set dateOfBirth =  "02/0" || substr(dateOfBirth, 10,1) || "/" || substr(dateOfBirth, 13, 4)  where dateOfBirth like "February__,%";  --single digit day

--select dateOfBirth, "03/" || substr(dateOfBirth, 7,2) || "/" || substr(dateOfBirth, 11, 4) from SexOffender where dateOfBirth like "March___,%";  -- double digit day
--select dateOfBirth , "03/0" || substr(dateOfBirth, 7,1) || "/" || substr(dateOfBirth, 10, 4)  from SexOffender where dateOfBirth like "March__,%";  -- single digit day

update SexOffender set dateOfBirth = "03/" || substr(dateOfBirth, 7,2) || "/" || substr(dateOfBirth, 11, 4) where dateOfBirth like "March___,%";  -- double digit day
update SexOffender set dateOfBirth = "03/0" || substr(dateOfBirth, 7,1) || "/" || substr(dateOfBirth, 10, 4)   where dateOfBirth like "March__,%";  -- single digit day

--select dateOfBirth,   "04/" || substr(dateOfBirth, 7,2) || "/" || substr(dateOfBirth, 11, 4) from SexOffender where dateOfBirth like "April___,%";  -- double digit day
--select dateOfBirth,  "04/0" || substr(dateOfBirth, 7,1) || "/" || substr(dateOfBirth, 10, 4)   from SexOffender where dateOfBirth like "April__,%";  -- single digit day

update SexOffender set dateOfBirth = "04/" || substr(dateOfBirth, 7,2) || "/" || substr(dateOfBirth, 11, 4) where dateOfBirth like "April___,%";  -- double digit day
update SexOffender set  dateOfBirth =  "04/0" || substr(dateOfBirth, 7,1) || "/" || substr(dateOfBirth, 10, 4)  where dateOfBirth like "April__,%";  -- single digit day

--select dateOfBirth, "05/" || substr(dateOfBirth, 5,2) || "/" || substr(dateOfBirth, 9, 4)  from SexOffender where dateOfBirth like "May___,%";  -- double digit day
--select dateOfBirth, "05/0" || substr(dateOfBirth, 5,1) || "/" || substr(dateOfBirth, 8, 4)  from SexOffender where dateOfBirth like "May__,%";  -- single digit day

update SexOffender set dateOfBirth = "05/" || substr(dateOfBirth, 5,2) || "/" || substr(dateOfBirth, 9, 4)  where dateOfBirth like "May___,%";  -- double digit day
update SexOffender set dateOfBirth = "05/0" || substr(dateOfBirth, 5,1) || "/" || substr(dateOfBirth, 8, 4)   where dateOfBirth like "May__,%";  -- single digit day

--select dateOfBirth, "06/" || substr(dateOfBirth, 6,2) || "/" || substr(dateOfBirth, 10, 4)   from SexOffender where dateOfBirth like "June___,%";  -- double digit day
--select dateOfBirth,"06/0" || substr(dateOfBirth, 6,1) || "/" || substr(dateOfBirth, 9, 4)   from SexOffender where dateOfBirth like "June__,%";  -- single digit day

update SexOffender set dateOfBirth = "06/" || substr(dateOfBirth, 6,2) || "/" || substr(dateOfBirth, 10, 4)   where dateOfBirth like "June___,%";  -- double digit day
update SexOffender set dateOfBirth ="06/0" || substr(dateOfBirth, 6,1) || "/" || substr(dateOfBirth, 9, 4)   where dateOfBirth like "June__,%";  -- single digit day

--select dateOfBirth, "07/" || substr(dateOfBirth, 6,2) || "/" || substr(dateOfBirth, 10, 4)   from SexOffender where dateOfBirth like "July___,%";  -- double digit day
--select dateOfBirth,"07/0" || substr(dateOfBirth, 6,1) || "/" || substr(dateOfBirth, 9, 4)   from SexOffender where dateOfBirth like "July__,%";  -- single digit day

update SexOffender set dateOfBirth = "07/" || substr(dateOfBirth, 6,2) || "/" || substr(dateOfBirth, 10, 4)   where dateOfBirth like "July___,%";  -- double digit day
update SexOffender set dateOfBirth ="07/0" || substr(dateOfBirth, 6,1) || "/" || substr(dateOfBirth, 9, 4)   where dateOfBirth like "July__,%";  -- single digit day

--select dateOfBirth, "08/" || substr(dateOfBirth, 8,2) || "/" || substr(dateOfBirth, 12, 4)   from SexOffender where dateOfBirth like "August___,%";  -- double digit day
--select dateOfBirth,"08/0" || substr(dateOfBirth, 8,1) || "/" || substr(dateOfBirth, 11, 4)   from SexOffender where dateOfBirth like "August__,%";  -- single digit day

update SexOffender set dateOfBirth = "08/" || substr(dateOfBirth, 8,2) || "/" || substr(dateOfBirth, 12, 4)   where dateOfBirth like "August___,%";  -- double digit day
update SexOffender set dateOfBirth ="08/0" || substr(dateOfBirth, 8,1) || "/" || substr(dateOfBirth, 11, 4)   where dateOfBirth like "August__,%";  -- single digit day

update SexOffender set dateOfBirth = "09/" || substr(dateOfBirth, 11,2) || "/" || substr(dateOfBirth, 15, 4)   where dateOfBirth like "September___,%";  -- double digit day
update SexOffender set dateOfBirth ="09/0" || substr(dateOfBirth, 11,1) || "/" || substr(dateOfBirth, 14, 4)   where dateOfBirth like "September__,%";  -- single digit day

--select dateOfBirth, "10/" || substr(dateOfBirth, 9,2) || "/" || substr(dateOfBirth, 13, 4)   from SexOffender where dateOfBirth like "October___,%";  -- double digit day
--select dateOfBirth,"10/0" || substr(dateOfBirth, 9,1) || "/" || substr(dateOfBirth, 12, 4)   from SexOffender where dateOfBirth like "October__,%";  -- single digit day

update SexOffender set dateOfBirth = "10/" || substr(dateOfBirth, 9,2) || "/" || substr(dateOfBirth, 13, 4)   where dateOfBirth like "October___,%";  -- double digit day
update SexOffender set dateOfBirth ="10/0" || substr(dateOfBirth, 9,1) || "/" || substr(dateOfBirth, 12, 4)   where dateOfBirth like "October__,%";  -- single digit day

--select dateOfBirth, "11/" || substr(dateOfBirth, 10,2) || "/" || substr(dateOfBirth, 14, 4)   from SexOffender where dateOfBirth like "November___,%";  -- double digit day
--select dateOfBirth,"11/0" || substr(dateOfBirth, 10,1) || "/" || substr(dateOfBirth, 13, 4)   from SexOffender where dateOfBirth like "November__,%";  -- single digit day

update SexOffender set dateOfBirth = "11/" || substr(dateOfBirth, 10,2) || "/" || substr(dateOfBirth, 14, 4)   where dateOfBirth like "November___,%";  -- double digit day
update SexOffender set dateOfBirth ="11/0" || substr(dateOfBirth, 10,1) || "/" || substr(dateOfBirth, 13, 4)   where dateOfBirth like "November__,%";  -- single digit day

--select dateOfBirth, "12/" || substr(dateOfBirth, 10,2) || "/" || substr(dateOfBirth, 14, 4)   from SexOffender where dateOfBirth like "December___,%";  -- double digit day
--select dateOfBirth,"12/0" || substr(dateOfBirth, 10,1) || "/" || substr(dateOfBirth, 13, 4)   from SexOffender where dateOfBirth like "December__,%";  -- single digit day

update SexOffender set dateOfBirth = "12/" || substr(dateOfBirth, 10,2) || "/" || substr(dateOfBirth, 14, 4)   where dateOfBirth like "December___,%";  -- double digit day
update SexOffender set dateOfBirth ="12/0" || substr(dateOfBirth, 10,1) || "/" || substr(dateOfBirth, 13, 4)   where dateOfBirth like "December__,%";  -- single digit day
