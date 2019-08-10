Insert into sexoffender (id,name,dateofbirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarstattoos,photos)
select cast(id as TEXT) as id
     ,ifnull(cast(Name as TEXT),'') as Name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM NCSexOffenders_alias als
            WHERE als.id = NCSexOffenders_main.id
              AND NCSexOffenders_main.state = als.state
           )
) as aliases

     ,json_array(
        json_object('address', cast(AddressLine1 as TEXT) || ' ' || cast(AddressLine2 as TEXT))) as addresses

     --offenses TODO: MISSING TABLE
     ,json_array( json_object('offense', 'UPDATE PENDING')) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarMarkTattoo as smt from NCSexOffenders_smts smts
             where smts.id = NCSexOffenders_main.id
               and smts.state = NCSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from NCSexOffenders_photos azp
             where azp.id = NCSexOffenders_main.id
               and azp.state = NCSexOffenders_main.state)) as photos


From NCSexOffenders_main
