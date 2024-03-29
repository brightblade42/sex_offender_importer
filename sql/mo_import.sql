Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(EyeColor as TEXT) as eyes
     ,cast(HairColor as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(Gender as TEXT) as sex
     ,trim(cast(state as TEXT)) as state
     -- aliases
     -- aliases
     ,( SELECT json_group_array (cast(alias as TEXT))
        FROM
            (SELECT alias
             FROM MOSexOffenders_aliases als
             WHERE als.id = MOSexOffenders_main.id
               AND MOSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Street as TEXT) || ' ' || cast(CityState as TEXT)
                       || ' ' || cast(Zip as TEXT)
                       ))

       FROM
           (SELECT Street,
                   CityState,
                   Zip
            FROM MOSexOffenders_addresses arad where arad.ID = MOSexOffenders_main.ID
                                                 and arad.state = MOSexOffenders_main.state)

) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) ))
       FROM

           (SELECT OffenseDescription as offense
            FROM MOSexOffenders_offenses aro
            where MOSexOffenders_main.ID = aro.ID
              and MOSexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(smt as text))
       from (select ScarsMarksTattoos as smt from MOSexOffenders_smts smts
             where smts.id = MOSexOffenders_main.id
               and smts.state = MOSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from MOSexOffenders_photos azp
             where azp.id = MOSexOffenders_main.id
               and azp.state = MOSexOffenders_main.state)) as photos

from MOSexOffenders_main
