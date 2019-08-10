Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DOB as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
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
             FROM NYSexOffenders_aliases als
             WHERE als.id = NYSexOffenders_main.id
               AND NYSexOffenders_main.state = als.state)
) as aliases
     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Street as TEXT) || ' ' || cast(CityStateZip as TEXT))
                  )

       FROM (SELECT Street, CityStateZip
             FROM NYSexOffenders_addresses arad
             where arad.ID = NYSexOffenders_main.ID
               and arad.state = NYSexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || '. ' || cast(description as TEXT) ))
       FROM

           (SELECT OffenseDescriptions as offense,
                   victimsexage as description
            FROM NYSexOffenders_current_conviction aro
            where NYSexOffenders_main.ID = aro.ID
              and NYSexOffenders_main.state = aro.state
           )
) as offenses

     --scarsTattoos
     ,(select json_group_array( cast(smt as text))

       from (select ScarMarkTattoo as smt from NYSexOffenders_smts smts
             where smts.id = NYSexOffenders_main.id
               and smts.state = NYSexOffenders_main.state)) as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NYSexOffenders_photos azp
             where azp.id = NYSexOffenders_main.id
               and azp.state = NYSexOffenders_main.state)) as photos

from NYSexOffenders_main
