Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,DOB as DateOfBirth

     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM UT_SexOffenders_aliases als
             WHERE als.id = UT_SexOffenders_main.id
               AND UT_SexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT)
                          ))

       FROM (SELECT Address
             FROM UT_SexOffenders_addresses arad
             where arad.ID = UT_SexOffenders_main.ID
               and arad.state = UT_SexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(OffenseDescription as Text) || ' ' || cast(OffenseDetails as TEXT)
                       ))
       FROM

           (SELECT OffenseDescription, OffenseDetails
            FROM UT_SexOffenders_offenses aro
            where UT_SexOffenders_main.ID = aro.ID
              and UT_SexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(ScarsMarksTattoos as text))

       from (select ScarsMarksTattoos from UT_SexOffenders_smts smts
             where smts.id = UT_SexOffenders_main.id
               and smts.state = UT_SexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from UT_SexOffenders_photos azp
             where azp.id = UT_SexOffenders_main.id
               and azp.state = UT_SexOffenders_main.state)) as photos

from UT_SexOffenders_main
