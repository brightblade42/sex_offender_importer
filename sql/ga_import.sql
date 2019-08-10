Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT cast(id as TEXT) as id
     ,ifnull(cast(LastName as TEXT),'') || ', ' || ifnull(cast(FirstName as TEXT),'') || ' '
          || ifnull(cast(MiddleName as TEXT),'') as name
     --'' as age,
     ,cast(YearOfBirth as TEXT)   as DateOfBirth

     ,cast(EyeColor as TEXT)   as eyes
     ,cast(HairColor as TEXT)   as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     -- aliases TODO: MISSING TABLE
     ,json_array("Unknown") as aliases
     --addresses
     ,json_array(
        json_object('address', cast(LastKnownAddress as Text))) as addresses

     -- offenses TODO: MISSING TABLE
     ,(SELECT
           json_group_array(json_object ( 'offense', cast(Offense as Text)
               ))
       FROM (SELECT Offense
             FROM GA_SexOffenders_offenses azo
             WHERE azo.id = GA_SexOffenders_main.id
               and GA_SexOffenders_main.state = azo.state
            )
) as offenses

     --scarsTattoos
     ,json_array("Unknown") as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from GA_SexOffenders_photos azp
             where azp.id = GA_SexOffenders_main.id
               and azp.state = GA_SexOffenders_main.state)) as photos

From GA_SexOffenders_main
