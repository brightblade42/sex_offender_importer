Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
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
             FROM WYSexOffenders_aliases als
             WHERE als.id = WYSexOffenders_main.id
               AND WYSexOffenders_main.state = als.state)
) as aliases

     --addresses

     ,(SELECT json_group_array(
                      json_object('address', cast(Address as TEXT),'type', cast(AddressType as TEXT)
                          ))

       FROM (SELECT Address, AddressType
             FROM WYSexOffenders_addresses arad
             where arad.ID = WYSexOffenders_main.ID
               and arad.state = WYSexOffenders_main.state)
)as addresses

     --offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text) || ' ' || cast(description as TEXT)
                       ))
       FROM

           (SELECT details as offense, description
            FROM WYSexOffenders_offenses aro
            where WYSexOffenders_main.ID = aro.ID
              and WYSexOffenders_main.state = aro.state
           )
) as offenses
     --scarsTattoos
     ,(select json_group_array( cast(smt as text))

       from (select Scars_Marks_Tattoos as smt from WYSexOffenders_smts smts
             where smts.id = WYSexOffenders_main.id
               and smts.state = WYSexOffenders_main.state)) as scarsTattoos


     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from WYSexOffenders_photos azp
             where azp.id = WYSexOffenders_main.id
               and azp.state = WYSexOffenders_main.state)) as photos

from WYSexOffenders_main;
