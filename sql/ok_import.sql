Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DOB as TEXT) as DateOfBirth

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
            (SELECT aliases as alias
             FROM OKSexOffenders_alias als
             WHERE als.id = OKSexOffenders_main.id
               AND OKSexOffenders_main.state = als.state)
) as "aliases"

     -- addresses
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(address as TEXT), 'type', cast(type as TEXT)) )
       FROM
           (SELECT address,
                   type
            FROM OKSexOffenders_addresses arad
            where arad.ID = OKSexOffenders_main.ID
              and arad.state = OKSexOffenders_main.state

           )) as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT))
               )
       FROM

           (SELECT
                Crime as offense
            FROM OKSexOffenders_offenses aro
            where OKSexOffenders_main.ID = aro.ID
              and OKSexOffenders_main.state = aro.state
           )
) as offenses
     -- scarsTattoos
     ,(select json_group_array( cast(type as TEXT) || ' ' || cast(smt as text))
       from (select Description as smt, type from OKSexOffenders_scars_marks_tattoos smts
             where smts.id = OKSexOffenders_main.id
               and smts.state = OKSexOffenders_main.state)) as scarsTattoos

     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from OKSexOffenders_photos azp
             where azp.id = OKSexOffenders_main.id
               and azp.state = OKSexOffenders_main.state)) as photos
from OKSexOffenders_main
