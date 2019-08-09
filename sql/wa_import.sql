Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,Name
     ,age as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     --aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM WA_SexOffenders_aliases als
            WHERE als.id = WA_SexOffenders_main.id
              AND WA_SexOffenders_main.state = als.state
           )
) as aliases

     --addresses
     ,json_array(
        json_object('address', cast(Address as Text))) as addresses

     --offenses

     ,(SELECT
           json_group_array ( json_object ('offense', cast(offense as TEXT) ))
       FROM

           (SELECT
                OffenseDescription as offense
            FROM WA_SexOffenders_offenses aro where WA_SexOffenders_main.ID = aro.ID
                                                and WA_SexOffenders_main.state = aro.state
           )) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from WA_SexOffenders_smts smts
             where smts.id = WA_SexOffenders_main.id
               and smts.state = WA_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from WA_SexOffenders_photos azp
             where azp.id = WA_SexOffenders_main.id
               and azp.state = WA_SexOffenders_main.state)) as photos


From WA_SexOffenders_main
