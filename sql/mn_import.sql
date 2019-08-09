Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,ifnull(Name,'') as Name
     ,DateOfBirth

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,RaceEthnicity as race
     ,'' as sex
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM MN_SexOffenders_aliases als
            WHERE als.id = MN_SexOffenders_main.id
              AND MN_SexOffenders_main.state = als.state
           )
) as aliases

     ,json_array(
        json_object('address', cast(RegisteredAddress as TEXT) || ' ' || cast(RegisteredCityStateZip as TEXT))) as addresses

     --offenses
     ,json_array(
        json_object('offense', cast(OffenseInformation as TEXT))
    ) as offenses

     ,json_array("Update Pending") as scarsTattoos

     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from MN_SexOffenders_photos azp
             where azp.id = MN_SexOffenders_main.id
               and azp.state = MN_SexOffenders_main.state)) as photos

From MN_SexOffenders_main
