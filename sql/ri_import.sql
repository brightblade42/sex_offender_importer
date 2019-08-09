Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
select id
     ,name
     ,DOB as DateOfBirth
     ,Eyes
     ,Hair
     ,Height
     ,Weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,json_array("Unknown") as aliases
     --addresses
     ,json_array(json_object('address', cast(Address as TEXT) || ' ' ||  cast(CityTown as TEXT))) as addresses
     --offenses
     ,json_array(json_object('offenses', cast(ConvictedOf as TEXT))) as offenses
     --scars tattoos
     ,json_array("Unknown") as scarsTatoos
     --photos
     ,(select json_group_array(cast(PhotoFile as TEXT))
       from (select PhotoFile from RISexOffenders_photos azp
             where azp.id = RISexOffenders_main.id
               and azp.state = RISexOffenders_main.state)) as photos

from RISexOffenders_main
