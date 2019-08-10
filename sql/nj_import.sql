Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(age as TEXT) as DateOfBirth
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
             FROM NJSexOffenders_aliases als
             WHERE als.id = NJSexOffenders_main.id
               AND NJSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT))
                  )

       FROM (SELECT Address
             FROM NJSexOffenders_addresses arad
             where arad.ID = NJSexOffenders_main.ID
               and arad.state = NJSexOffenders_main.state)
)as addresses
     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || '. ' || cast(description as TEXT) ))
       FROM

           (SELECT offense_description as offense,
                   offense_details as description
            FROM NJSexOffenders_convictions aro
            where NJSexOffenders_main.ID = aro.ID
              and NJSexOffenders_main.state = aro.state
           )
) as offenses


     ,(select json_group_array( cast(smt as text))

       from (select scars_marks_tattoos as smt from NJSexOffenders_smts smts
             where smts.id = NJSexOffenders_main.id
               and smts.state = NJSexOffenders_main.state)) as scarsTattoos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from NJSexOffenders_photos azp
             where azp.id = NJSexOffenders_main.id
               and azp.state = NJSexOffenders_main.state)) as photos

from NJSexOffenders_main
