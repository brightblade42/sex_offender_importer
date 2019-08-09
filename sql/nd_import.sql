Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,ifnull(Name,'') as Name
     ,DateOfBirth
     ,eyes
     ,hair
     ,height
     ,weight
     ,race
     ,sex
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM NDSexOffenders_alias als
            WHERE als.id = NDSexOffenders_main.id
              AND NDSexOffenders_main.state = als.state
           )
) as aliases

     ,json_array(
        json_object('address', cast(StreetAddress as Text) || ' ' || cast(AddressName as TEXT) || ' ' || cast(CityStateZip as TEXT))) as addresses

     --offenses

     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT))
               )
       FROM

           (SELECT
                offense
            FROM NDSexOffenders_convictions aro where NDSexOffenders_main.ID = aro.ID
                                                  and NDSexOffenders_main.state = aro.state
           )) as offenses
     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from NDSexOffenders_photos azp
             where azp.id = NDSexOffenders_main.id
               and azp.state = NDSexOffenders_main.state)) as photos


From NDSexOffenders_main
