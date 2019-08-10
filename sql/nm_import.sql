Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(YearOfBirth as TEXT) as DateOfBirth

     ,cast(EyeColor as TEXT) as eyes
     ,cast(HairColor as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(sex as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM NMSexOffenders_aliases als
             WHERE als.id = NMSexOffenders_main.id
               AND NMSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(street as TEXT) || ' ' || cast(citystatezip as TEXT))
                  )

       FROM (SELECT street, citystatezip
             FROM NMSexOffenders_addresses arad
             where arad.ID = NMSexOffenders_main.ID
               and arad.state = NMSexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || '. ' || cast(description as TEXT) ))
       FROM

           (SELECT OffenseDescription as offense,
                   OffenseDetails as description
            FROM NMSexOffenders_offenses aro
            where NMSexOffenders_main.ID = aro.ID
              and NMSexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))

       from (select ScarsMarksTattoos as smt from NMSexOffenders_smts smts
             where smts.id = NMSexOffenders_main.id
               and smts.state = NMSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NMSexOffenders_photos azp
             where azp.id = NMSexOffenders_main.id
               and azp.state = NMSexOffenders_main.state)) as photos

from NMSexOffenders_main
