Insert into SexOffender (id,name,dateOfBirth, eyes, hair, height, weight, race,sex,state,aliases,addresses,offenses,scarsTattoos,photos)
SELECT id
     ,ifnull(Name,'') as Name

     ,EyeColor as eyes
     ,HairColor as hair
     ,height
     ,weight
     ,race
     ,Gender as sex
     ,DateOfBirth
     ,trim(state) as state
     -- aliases
     ,(SELECT json_group_array(cast(alias as Text))
       FROM
           (SELECT alias
            FROM SDSexOffenders_alias als
            WHERE als.id = SDSexOffenders_main.id
              AND SDSexOffenders_main.state = als.state
           )
) as aliases
     --addresses
     ,json_array(
        json_object('address', cast(Address as Text) || ' ' || cast(CityStateZip as TEXT))) as addresses

     --offenses

     ,(SELECT
           json_group_array (
                   json_object ('offense', cast(offense as TEXT) || ' ' || cast(description as Text))
               )
       FROM

           (SELECT
                CrimesConvicted as offense
                 ,CrimeDescription as description
            FROM SDSexOffenders_convictions aro where SDSexOffenders_main.ID = aro.ID
                                                  and SDSexOffenders_main.state = aro.state
           )) as offenses
     --scarsTattoos
     ,json_array("None Reported") as scarsTattoos
     --photos
     ,(select json_group_array(cast(PhotoFile as Text))
       from (select PhotoFile from SDSexOffenders_photos azp
             where azp.id = SDSexOffenders_main.id
               and azp.state = SDSexOffenders_main.state)) as photos


From SDSexOffenders_main
