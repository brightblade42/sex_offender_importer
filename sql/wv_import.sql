Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT cast(id as TEXT) as id
     ,cast(name as TEXT) as name
     ,cast(DateOfBirth as TEXT) as DateOfBirth
     ,cast(eyes as TEXT) as eyes
     ,cast(hair as TEXT) as hair
     ,cast(height as TEXT) as height
     ,cast(weight as TEXT) as weight
     ,cast(race as TEXT) as race
     ,cast(gender as TEXT) as sex
     ,upper(trim(cast(state as TEXT))) as state
     -- aliases
     ,json_array('Unknown') as aliases
     -- addresses pookie
     ,(SELECT
           json_group_array(
                   json_object ('address', cast(Street as TEXT)
                       || ' ' || cast(City as TEXT)
                       || ' ' || cast(Addr_State as TEXT)
                       || ' ' || cast(Zip as TEXT)) )
       FROM
           (SELECT street,
                   city,
                   Addr_State,
                   Zip
            FROM WV_SexOffenders_addresses arad
            where arad.ID = WV_SexOffenders_main.ID
              and arad.state = WV_SexOffenders_main.state

           )) as addresses
     -- offenses
     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as Text)
                       ))
       FROM

           (SELECT Offense as offense
            FROM WV_SexOffenders_offenses aro
            where WV_SexOffenders_main.ID = aro.ID
              and WV_SexOffenders_main.state = aro.state
           )
) as offenses
     ,json_array("Unknown") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from WV_SexOffenders_photos azp
             where azp.id = WV_SexOffenders_main.id
               and azp.state = WV_SexOffenders_main.state)) as photos


from WV_SexOffenders_main
